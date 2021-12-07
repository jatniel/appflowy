use crate::{
    core::{
        history::{History, UndoResult},
        view::{View, RECORD_THRESHOLD},
    },
    errors::DocumentError,
    user_default::doc_initial_delta,
};
use lib_ot::{
    core::*,
    rich_text::{RichTextAttribute, RichTextDelta},
};
use tokio::sync::mpsc;

pub trait CustomDocument {
    fn init_delta() -> RichTextDelta;
}

pub struct PlainDoc();
impl CustomDocument for PlainDoc {
    fn init_delta() -> RichTextDelta { RichTextDelta::new() }
}

pub struct FlowyDoc();
impl CustomDocument for FlowyDoc {
    fn init_delta() -> RichTextDelta { doc_initial_delta() }
}

pub struct Document {
    delta: RichTextDelta,
    history: History,
    view: View,
    last_edit_time: usize,
    notify: Option<mpsc::UnboundedSender<()>>,
}

impl Document {
    pub fn new<C: CustomDocument>() -> Self { Self::from_delta(C::init_delta()) }

    pub fn from_delta(delta: RichTextDelta) -> Self {
        Document {
            delta,
            history: History::new(),
            view: View::new(),
            last_edit_time: 0,
            notify: None,
        }
    }

    pub fn from_json(json: &str) -> Result<Self, DocumentError> {
        let delta = RichTextDelta::from_json(json)?;
        Ok(Self::from_delta(delta))
    }

    pub fn to_json(&self) -> String { self.delta.to_json() }

    pub fn to_bytes(&self) -> Vec<u8> { self.delta.clone().to_bytes().to_vec() }

    pub fn to_plain_string(&self) -> String { self.delta.apply("").unwrap() }

    pub fn delta(&self) -> &RichTextDelta { &self.delta }

    pub fn set_notify(&mut self, notify: mpsc::UnboundedSender<()>) { self.notify = Some(notify); }

    pub fn set_delta(&mut self, data: RichTextDelta) {
        self.delta = data;

        match &self.notify {
            None => {},
            Some(notify) => {
                let _ = notify.send(());
            },
        }
    }

    pub fn compose_delta(&mut self, mut delta: RichTextDelta) -> Result<(), DocumentError> {
        trim(&mut delta);
        tracing::trace!("{} compose {}", &self.delta.to_json(), delta.to_json());
        let mut composed_delta = self.delta.compose(&delta)?;
        let mut undo_delta = delta.invert(&self.delta);

        let now = chrono::Utc::now().timestamp_millis() as usize;
        if now - self.last_edit_time < RECORD_THRESHOLD {
            if let Some(last_delta) = self.history.undo() {
                tracing::trace!("compose previous change");
                tracing::trace!("current = {}", undo_delta);
                tracing::trace!("previous = {}", last_delta);
                undo_delta = undo_delta.compose(&last_delta)?;
            }
        } else {
            self.last_edit_time = now;
        }

        tracing::trace!("👉 receive change undo: {}", undo_delta);
        if !undo_delta.is_empty() {
            self.history.record(undo_delta);
        }

        tracing::trace!("compose result: {}", composed_delta.to_json());
        trim(&mut composed_delta);

        self.set_delta(composed_delta);
        Ok(())
    }

    pub fn insert<T: ToString>(&mut self, index: usize, data: T) -> Result<RichTextDelta, DocumentError> {
        let interval = Interval::new(index, index);
        let _ = validate_interval(&self.delta, &interval)?;

        let text = data.to_string();
        let delta = self.view.insert(&self.delta, &text, interval)?;
        tracing::trace!("👉 receive change: {}", delta);
        self.compose_delta(delta.clone())?;
        Ok(delta)
    }

    pub fn delete(&mut self, interval: Interval) -> Result<RichTextDelta, DocumentError> {
        let _ = validate_interval(&self.delta, &interval)?;
        debug_assert_eq!(interval.is_empty(), false);
        let delete = self.view.delete(&self.delta, interval)?;
        if !delete.is_empty() {
            tracing::trace!("👉 receive change: {}", delete);
            let _ = self.compose_delta(delete.clone())?;
        }
        Ok(delete)
    }

    pub fn format(&mut self, interval: Interval, attribute: RichTextAttribute) -> Result<RichTextDelta, DocumentError> {
        let _ = validate_interval(&self.delta, &interval)?;
        tracing::trace!("format with {} at {}", attribute, interval);
        let format_delta = self.view.format(&self.delta, attribute, interval).unwrap();

        tracing::trace!("👉 receive change: {}", format_delta);
        self.compose_delta(format_delta.clone())?;
        Ok(format_delta)
    }

    pub fn replace<T: ToString>(&mut self, interval: Interval, data: T) -> Result<RichTextDelta, DocumentError> {
        let _ = validate_interval(&self.delta, &interval)?;
        let mut delta = RichTextDelta::default();
        let text = data.to_string();
        if !text.is_empty() {
            delta = self.view.insert(&self.delta, &text, interval)?;
            tracing::trace!("👉 receive change: {}", delta);
            self.compose_delta(delta.clone())?;
        }

        if !interval.is_empty() {
            let delete = self.delete(interval)?;
            delta = delta.compose(&delete)?;
        }

        Ok(delta)
    }

    pub fn can_undo(&self) -> bool { self.history.can_undo() }

    pub fn can_redo(&self) -> bool { self.history.can_redo() }

    pub fn undo(&mut self) -> Result<UndoResult, DocumentError> {
        match self.history.undo() {
            None => Err(DocumentError::undo().context("Undo stack is empty")),
            Some(undo_delta) => {
                let (new_delta, inverted_delta) = self.invert(&undo_delta)?;
                let result = UndoResult::success(new_delta.target_len as usize);
                self.set_delta(new_delta);
                self.history.add_redo(inverted_delta);

                Ok(result)
            },
        }
    }

    pub fn redo(&mut self) -> Result<UndoResult, DocumentError> {
        match self.history.redo() {
            None => Err(DocumentError::redo()),
            Some(redo_delta) => {
                let (new_delta, inverted_delta) = self.invert(&redo_delta)?;
                let result = UndoResult::success(new_delta.target_len as usize);
                self.set_delta(new_delta);

                self.history.add_undo(inverted_delta);
                Ok(result)
            },
        }
    }
}

impl Document {
    fn invert(&self, delta: &RichTextDelta) -> Result<(RichTextDelta, RichTextDelta), DocumentError> {
        // c = a.compose(b)
        // d = b.invert(a)
        // a = c.compose(d)
        tracing::trace!("Invert {}", delta);
        let new_delta = self.delta.compose(delta)?;
        let inverted_delta = delta.invert(&self.delta);
        Ok((new_delta, inverted_delta))
    }
}

fn validate_interval(delta: &RichTextDelta, interval: &Interval) -> Result<(), DocumentError> {
    if delta.target_len < interval.end {
        log::error!("{:?} out of bounds. should 0..{}", interval, delta.target_len);
        return Err(DocumentError::out_of_bound());
    }
    Ok(())
}

/// Removes trailing retain operation with empty attributes, if present.
pub fn trim(delta: &mut RichTextDelta) {
    if let Some(last) = delta.ops.last() {
        if last.is_retain() && last.is_plain() {
            delta.ops.pop();
        }
    }
}
