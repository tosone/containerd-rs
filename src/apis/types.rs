/// Descriptor describes a blob in a content store.
///
/// This descriptor can be used to reference content from an
/// oci descriptor found in a manifest.
/// See <https://godoc.org/github.com/opencontainers/image-spec/specs-go/v1#Descriptor>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Descriptor {
    #[prost(string, tag = "1")]
    pub media_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub digest: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub size: i64,
    #[prost(map = "string, string", tag = "5")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Mount describes mounts for a container.
///
/// This type is the lingua franca of ContainerD. All services provide mounts
/// to be used with the container at creation time.
///
/// The Mount type follows the structure of the mount syscall, including a type,
/// source, target and options.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mount {
    /// Type defines the nature of the mount.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Source specifies the name of the mount. Depending on mount type, this
    /// may be a volume name or a host path, or even ignored.
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    /// Target path in container
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Options specifies zero or more fstab style mount options.
    #[prost(string, repeated, tag = "4")]
    pub options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Platform follows the structure of the OCI platform specification, from
/// descriptors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Platform {
    #[prost(string, tag = "1")]
    pub os: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub architecture: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub variant: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<::prost_types::Any>,
}
