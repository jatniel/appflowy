// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `event_map.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FolderEvent {
    CreateWorkspace = 0,
    ReadCurWorkspace = 1,
    ReadWorkspaces = 2,
    DeleteWorkspace = 3,
    OpenWorkspace = 4,
    ReadWorkspaceApps = 5,
    CreateApp = 101,
    DeleteApp = 102,
    ReadApp = 103,
    UpdateApp = 104,
    CreateView = 201,
    ReadView = 202,
    UpdateView = 203,
    DeleteView = 204,
    DuplicateView = 205,
    CopyLink = 206,
    OpenDocument = 207,
    CloseView = 208,
    ReadTrash = 300,
    PutbackTrash = 301,
    DeleteTrash = 302,
    RestoreAllTrash = 303,
    DeleteAllTrash = 304,
    ApplyDocDelta = 400,
    ExportDocument = 500,
}

impl ::protobuf::ProtobufEnum for FolderEvent {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FolderEvent> {
        match value {
            0 => ::std::option::Option::Some(FolderEvent::CreateWorkspace),
            1 => ::std::option::Option::Some(FolderEvent::ReadCurWorkspace),
            2 => ::std::option::Option::Some(FolderEvent::ReadWorkspaces),
            3 => ::std::option::Option::Some(FolderEvent::DeleteWorkspace),
            4 => ::std::option::Option::Some(FolderEvent::OpenWorkspace),
            5 => ::std::option::Option::Some(FolderEvent::ReadWorkspaceApps),
            101 => ::std::option::Option::Some(FolderEvent::CreateApp),
            102 => ::std::option::Option::Some(FolderEvent::DeleteApp),
            103 => ::std::option::Option::Some(FolderEvent::ReadApp),
            104 => ::std::option::Option::Some(FolderEvent::UpdateApp),
            201 => ::std::option::Option::Some(FolderEvent::CreateView),
            202 => ::std::option::Option::Some(FolderEvent::ReadView),
            203 => ::std::option::Option::Some(FolderEvent::UpdateView),
            204 => ::std::option::Option::Some(FolderEvent::DeleteView),
            205 => ::std::option::Option::Some(FolderEvent::DuplicateView),
            206 => ::std::option::Option::Some(FolderEvent::CopyLink),
            207 => ::std::option::Option::Some(FolderEvent::OpenDocument),
            208 => ::std::option::Option::Some(FolderEvent::CloseView),
            300 => ::std::option::Option::Some(FolderEvent::ReadTrash),
            301 => ::std::option::Option::Some(FolderEvent::PutbackTrash),
            302 => ::std::option::Option::Some(FolderEvent::DeleteTrash),
            303 => ::std::option::Option::Some(FolderEvent::RestoreAllTrash),
            304 => ::std::option::Option::Some(FolderEvent::DeleteAllTrash),
            400 => ::std::option::Option::Some(FolderEvent::ApplyDocDelta),
            500 => ::std::option::Option::Some(FolderEvent::ExportDocument),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FolderEvent] = &[
            FolderEvent::CreateWorkspace,
            FolderEvent::ReadCurWorkspace,
            FolderEvent::ReadWorkspaces,
            FolderEvent::DeleteWorkspace,
            FolderEvent::OpenWorkspace,
            FolderEvent::ReadWorkspaceApps,
            FolderEvent::CreateApp,
            FolderEvent::DeleteApp,
            FolderEvent::ReadApp,
            FolderEvent::UpdateApp,
            FolderEvent::CreateView,
            FolderEvent::ReadView,
            FolderEvent::UpdateView,
            FolderEvent::DeleteView,
            FolderEvent::DuplicateView,
            FolderEvent::CopyLink,
            FolderEvent::OpenDocument,
            FolderEvent::CloseView,
            FolderEvent::ReadTrash,
            FolderEvent::PutbackTrash,
            FolderEvent::DeleteTrash,
            FolderEvent::RestoreAllTrash,
            FolderEvent::DeleteAllTrash,
            FolderEvent::ApplyDocDelta,
            FolderEvent::ExportDocument,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<FolderEvent>("FolderEvent", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for FolderEvent {
}

impl ::std::default::Default for FolderEvent {
    fn default() -> Self {
        FolderEvent::CreateWorkspace
    }
}

impl ::protobuf::reflect::ProtobufValue for FolderEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fevent_map.proto*\xd6\x03\n\x0bFolderEvent\x12\x13\n\x0fCreateWorks\
    pace\x10\0\x12\x14\n\x10ReadCurWorkspace\x10\x01\x12\x12\n\x0eReadWorksp\
    aces\x10\x02\x12\x13\n\x0fDeleteWorkspace\x10\x03\x12\x11\n\rOpenWorkspa\
    ce\x10\x04\x12\x15\n\x11ReadWorkspaceApps\x10\x05\x12\r\n\tCreateApp\x10\
    e\x12\r\n\tDeleteApp\x10f\x12\x0b\n\x07ReadApp\x10g\x12\r\n\tUpdateApp\
    \x10h\x12\x0f\n\nCreateView\x10\xc9\x01\x12\r\n\x08ReadView\x10\xca\x01\
    \x12\x0f\n\nUpdateView\x10\xcb\x01\x12\x0f\n\nDeleteView\x10\xcc\x01\x12\
    \x12\n\rDuplicateView\x10\xcd\x01\x12\r\n\x08CopyLink\x10\xce\x01\x12\
    \x11\n\x0cOpenDocument\x10\xcf\x01\x12\x0e\n\tCloseView\x10\xd0\x01\x12\
    \x0e\n\tReadTrash\x10\xac\x02\x12\x11\n\x0cPutbackTrash\x10\xad\x02\x12\
    \x10\n\x0bDeleteTrash\x10\xae\x02\x12\x14\n\x0fRestoreAllTrash\x10\xaf\
    \x02\x12\x13\n\x0eDeleteAllTrash\x10\xb0\x02\x12\x12\n\rApplyDocDelta\
    \x10\x90\x03\x12\x13\n\x0eExportDocument\x10\xf4\x03J\xab\x08\n\x06\x12\
    \x04\0\0\x1b\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x05\0\x12\x04\
    \x01\0\x1b\x01\n\n\n\x03\x05\0\x01\x12\x03\x01\x05\x10\n\x0b\n\x04\x05\0\
    \x02\0\x12\x03\x02\x04\x18\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x02\x04\
    \x13\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x02\x16\x17\n\x0b\n\x04\x05\0\
    \x02\x01\x12\x03\x03\x04\x19\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x03\
    \x04\x14\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x03\x17\x18\n\x0b\n\x04\
    \x05\0\x02\x02\x12\x03\x04\x04\x17\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\
    \x04\x04\x12\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x04\x15\x16\n\x0b\n\
    \x04\x05\0\x02\x03\x12\x03\x05\x04\x18\n\x0c\n\x05\x05\0\x02\x03\x01\x12\
    \x03\x05\x04\x13\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x05\x16\x17\n\x0b\
    \n\x04\x05\0\x02\x04\x12\x03\x06\x04\x16\n\x0c\n\x05\x05\0\x02\x04\x01\
    \x12\x03\x06\x04\x11\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x06\x14\x15\n\
    \x0b\n\x04\x05\0\x02\x05\x12\x03\x07\x04\x1a\n\x0c\n\x05\x05\0\x02\x05\
    \x01\x12\x03\x07\x04\x15\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\x07\x18\
    \x19\n\x0b\n\x04\x05\0\x02\x06\x12\x03\x08\x04\x14\n\x0c\n\x05\x05\0\x02\
    \x06\x01\x12\x03\x08\x04\r\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\x08\x10\
    \x13\n\x0b\n\x04\x05\0\x02\x07\x12\x03\t\x04\x14\n\x0c\n\x05\x05\0\x02\
    \x07\x01\x12\x03\t\x04\r\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03\t\x10\x13\
    \n\x0b\n\x04\x05\0\x02\x08\x12\x03\n\x04\x12\n\x0c\n\x05\x05\0\x02\x08\
    \x01\x12\x03\n\x04\x0b\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x03\n\x0e\x11\n\
    \x0b\n\x04\x05\0\x02\t\x12\x03\x0b\x04\x14\n\x0c\n\x05\x05\0\x02\t\x01\
    \x12\x03\x0b\x04\r\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03\x0b\x10\x13\n\x0b\
    \n\x04\x05\0\x02\n\x12\x03\x0c\x04\x15\n\x0c\n\x05\x05\0\x02\n\x01\x12\
    \x03\x0c\x04\x0e\n\x0c\n\x05\x05\0\x02\n\x02\x12\x03\x0c\x11\x14\n\x0b\n\
    \x04\x05\0\x02\x0b\x12\x03\r\x04\x13\n\x0c\n\x05\x05\0\x02\x0b\x01\x12\
    \x03\r\x04\x0c\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\x03\r\x0f\x12\n\x0b\n\
    \x04\x05\0\x02\x0c\x12\x03\x0e\x04\x15\n\x0c\n\x05\x05\0\x02\x0c\x01\x12\
    \x03\x0e\x04\x0e\n\x0c\n\x05\x05\0\x02\x0c\x02\x12\x03\x0e\x11\x14\n\x0b\
    \n\x04\x05\0\x02\r\x12\x03\x0f\x04\x15\n\x0c\n\x05\x05\0\x02\r\x01\x12\
    \x03\x0f\x04\x0e\n\x0c\n\x05\x05\0\x02\r\x02\x12\x03\x0f\x11\x14\n\x0b\n\
    \x04\x05\0\x02\x0e\x12\x03\x10\x04\x18\n\x0c\n\x05\x05\0\x02\x0e\x01\x12\
    \x03\x10\x04\x11\n\x0c\n\x05\x05\0\x02\x0e\x02\x12\x03\x10\x14\x17\n\x0b\
    \n\x04\x05\0\x02\x0f\x12\x03\x11\x04\x13\n\x0c\n\x05\x05\0\x02\x0f\x01\
    \x12\x03\x11\x04\x0c\n\x0c\n\x05\x05\0\x02\x0f\x02\x12\x03\x11\x0f\x12\n\
    \x0b\n\x04\x05\0\x02\x10\x12\x03\x12\x04\x17\n\x0c\n\x05\x05\0\x02\x10\
    \x01\x12\x03\x12\x04\x10\n\x0c\n\x05\x05\0\x02\x10\x02\x12\x03\x12\x13\
    \x16\n\x0b\n\x04\x05\0\x02\x11\x12\x03\x13\x04\x14\n\x0c\n\x05\x05\0\x02\
    \x11\x01\x12\x03\x13\x04\r\n\x0c\n\x05\x05\0\x02\x11\x02\x12\x03\x13\x10\
    \x13\n\x0b\n\x04\x05\0\x02\x12\x12\x03\x14\x04\x14\n\x0c\n\x05\x05\0\x02\
    \x12\x01\x12\x03\x14\x04\r\n\x0c\n\x05\x05\0\x02\x12\x02\x12\x03\x14\x10\
    \x13\n\x0b\n\x04\x05\0\x02\x13\x12\x03\x15\x04\x17\n\x0c\n\x05\x05\0\x02\
    \x13\x01\x12\x03\x15\x04\x10\n\x0c\n\x05\x05\0\x02\x13\x02\x12\x03\x15\
    \x13\x16\n\x0b\n\x04\x05\0\x02\x14\x12\x03\x16\x04\x16\n\x0c\n\x05\x05\0\
    \x02\x14\x01\x12\x03\x16\x04\x0f\n\x0c\n\x05\x05\0\x02\x14\x02\x12\x03\
    \x16\x12\x15\n\x0b\n\x04\x05\0\x02\x15\x12\x03\x17\x04\x1a\n\x0c\n\x05\
    \x05\0\x02\x15\x01\x12\x03\x17\x04\x13\n\x0c\n\x05\x05\0\x02\x15\x02\x12\
    \x03\x17\x16\x19\n\x0b\n\x04\x05\0\x02\x16\x12\x03\x18\x04\x19\n\x0c\n\
    \x05\x05\0\x02\x16\x01\x12\x03\x18\x04\x12\n\x0c\n\x05\x05\0\x02\x16\x02\
    \x12\x03\x18\x15\x18\n\x0b\n\x04\x05\0\x02\x17\x12\x03\x19\x04\x18\n\x0c\
    \n\x05\x05\0\x02\x17\x01\x12\x03\x19\x04\x11\n\x0c\n\x05\x05\0\x02\x17\
    \x02\x12\x03\x19\x14\x17\n\x0b\n\x04\x05\0\x02\x18\x12\x03\x1a\x04\x19\n\
    \x0c\n\x05\x05\0\x02\x18\x01\x12\x03\x1a\x04\x12\n\x0c\n\x05\x05\0\x02\
    \x18\x02\x12\x03\x1a\x15\x18b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
