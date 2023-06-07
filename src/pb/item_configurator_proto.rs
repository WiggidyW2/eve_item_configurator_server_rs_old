// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListItem {
    #[prost(uint32, tag="1")]
    pub type_id: u32,
    #[prost(bool, tag="2")]
    pub enabled: bool,
    #[prost(map="string, uint32", tag="3")]
    pub json_idx: ::std::collections::HashMap<::prost::alloc::string::String, u32>,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub market_group_idx: u32,
    #[prost(uint32, tag="6")]
    pub group_idx: u32,
    #[prost(uint32, tag="7")]
    pub category_idx: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRep {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<ListItem>,
    #[prost(string, repeated, tag="2")]
    pub json: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub market_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// new ESI refresh token (for authentication)
    #[prost(string, tag="6")]
    pub refresh_token: ::prost::alloc::string::String,
    /// whether the character from the token is authorized
    #[prost(bool, tag="7")]
    pub authorized: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReq {
    /// name of the list
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// ESI refresh token (for authentication)
    #[prost(string, tag="2")]
    pub refresh_token: ::prost::alloc::string::String,
    /// whether to include enabled items
    #[prost(enumeration="Query", tag="3")]
    pub include_enabled: i32,
    /// whether to include configured items
    #[prost(enumeration="Query", tag="4")]
    pub include_configured: i32,
    #[prost(bool, tag="5")]
    pub include_json: bool,
    #[prost(string, tag="6")]
    pub language: ::prost::alloc::string::String,
    #[prost(bool, tag="7")]
    pub include_name: bool,
    #[prost(bool, tag="8")]
    pub include_market_group: bool,
    #[prost(bool, tag="9")]
    pub include_group: bool,
    #[prost(bool, tag="10")]
    pub include_category: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateItem {
    #[prost(uint32, tag="1")]
    pub type_id: u32,
    #[prost(bool, tag="2")]
    pub enabled: bool,
    #[prost(map="string, uint32", tag="3")]
    pub json_idx: ::std::collections::HashMap<::prost::alloc::string::String, u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRep {
    /// new ESI refresh token (for authentication)
    #[prost(string, tag="1")]
    pub refresh_token: ::prost::alloc::string::String,
    /// whether the character from the token is authorized
    #[prost(bool, tag="2")]
    pub authorized: bool,
}
/// Note: Don't delete JSON when things are disabled, keep storing it
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReq {
    /// name of the list
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// ESI refresh token (for authentication)
    #[prost(string, tag="2")]
    pub refresh_token: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub items: ::prost::alloc::vec::Vec<UpdateItem>,
    #[prost(string, repeated, tag="4")]
    pub json: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCharactersRep {
    #[prost(string, repeated, tag="1")]
    pub characters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// new ESI refresh token (for authentication)
    #[prost(string, tag="2")]
    pub refresh_token: ::prost::alloc::string::String,
    /// whether the character from the token is authorized
    #[prost(bool, tag="3")]
    pub authorized: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCharactersReq {
    /// name of the list
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// false for read, true for write
    #[prost(bool, tag="2")]
    pub auth_kind: bool,
    /// false for items, true for characters
    #[prost(bool, tag="3")]
    pub auth_scope: bool,
    /// ESI refresh token (for authentication)
    #[prost(string, tag="4")]
    pub refresh_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCharactersRep {
    /// new ESI refresh token (for authentication)
    #[prost(string, tag="1")]
    pub refresh_token: ::prost::alloc::string::String,
    /// whether the character from the token is authorized
    #[prost(bool, tag="2")]
    pub authorized: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCharactersReq {
    /// name of the list
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// false for read, true for write
    #[prost(bool, tag="2")]
    pub auth_kind: bool,
    /// false for items, true for characters
    #[prost(bool, tag="3")]
    pub auth_scope: bool,
    /// ESI refresh token (for authentication)
    #[prost(string, tag="4")]
    pub refresh_token: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub characters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelCharactersRep {
    /// new ESI refresh token (for authentication)
    #[prost(string, tag="1")]
    pub refresh_token: ::prost::alloc::string::String,
    /// whether the character from the token is authorized
    #[prost(bool, tag="2")]
    pub authorized: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelCharactersReq {
    /// name of the list
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// false for read, true for write
    #[prost(bool, tag="2")]
    pub auth_kind: bool,
    /// false for items, true for characters
    #[prost(bool, tag="3")]
    pub auth_scope: bool,
    /// ESI refresh token (for authentication)
    #[prost(string, tag="4")]
    pub refresh_token: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub characters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Query {
    /// items for which value is true
    True = 0,
    /// items for which value is false
    False = 1,
    /// all items
    Both = 2,
}
impl Query {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Query::True => "TRUE",
            Query::False => "FALSE",
            Query::Both => "BOTH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRUE" => Some(Self::True),
            "FALSE" => Some(Self::False),
            "BOTH" => Some(Self::Both),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthKind {
    Read = 0,
    Write = 1,
}
impl AuthKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthKind::Read => "READ",
            AuthKind::Write => "WRITE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "READ" => Some(Self::Read),
            "WRITE" => Some(Self::Write),
            _ => None,
        }
    }
}
include!("item_configurator_proto.tonic.rs");
// @@protoc_insertion_point(module)
