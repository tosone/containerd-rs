#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    /// ID is the user-specified identifier.
    ///
    /// This field may not be updated.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Labels provides an area to include arbitrary data on containers.
    ///
    /// The combined size of a key/value pair cannot exceed 4096 bytes.
    ///
    /// Note that to add a new value to this field, read the existing set and
    /// include the entire result in the update call.
    #[prost(map = "string, string", tag = "2")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Image contains the reference of the image used to build the
    /// specification and snapshots for running this container.
    ///
    /// If this field is updated, the spec and rootfs needed to updated, as well.
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    /// Runtime specifies which runtime to use for executing this container.
    #[prost(message, optional, tag = "4")]
    pub runtime: ::core::option::Option<container::Runtime>,
    /// Spec to be used when creating the container. This is runtime specific.
    #[prost(message, optional, tag = "5")]
    pub spec: ::core::option::Option<::prost_types::Any>,
    /// Snapshotter specifies the snapshotter name used for rootfs
    #[prost(string, tag = "6")]
    pub snapshotter: ::prost::alloc::string::String,
    /// SnapshotKey specifies the snapshot key to use for the container's root
    /// filesystem. When starting a task from this container, a caller should
    /// look up the mounts from the snapshot service and include those on the
    /// task create request.
    ///
    /// Snapshots referenced in this field will not be garbage collected.
    ///
    /// This field is set to empty when the rootfs is not a snapshot.
    ///
    /// This field may be updated.
    #[prost(string, tag = "7")]
    pub snapshot_key: ::prost::alloc::string::String,
    /// CreatedAt is the time the container was first created.
    #[prost(message, optional, tag = "8")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// UpdatedAt is the last time the container was mutated.
    #[prost(message, optional, tag = "9")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Extensions allow clients to provide zero or more blobs that are directly
    /// associated with the container. One may provide protobuf, json, or other
    /// encoding formats. The primary use of this is to further decorate the
    /// container object with fields that may be specific to a client integration.
    ///
    /// The key portion of this map should identify a "name" for the extension
    /// that should be unique against other extensions. When updating extension
    /// data, one should only update the specified extension using field paths
    /// to select a specific map key.
    #[prost(map = "string, message", tag = "10")]
    pub extensions: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Any>,
}
/// Nested message and enum types in `Container`.
pub mod container {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Runtime {
        /// Name is the name of the runtime.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Options specify additional runtime initialization options.
        #[prost(message, optional, tag = "2")]
        pub options: ::core::option::Option<::prost_types::Any>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContainerRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContainerResponse {
    #[prost(message, optional, tag = "1")]
    pub container: ::core::option::Option<Container>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContainersRequest {
    /// Filters contains one or more filters using the syntax defined in the
    /// containerd filter package.
    ///
    /// The returned result will be those that match any of the provided
    /// filters. Expanded, containers that match the following will be
    /// returned:
    ///
    ///   filters\[0\] or filters\[1\] or ... or filters\[n-1\] or filters\[n\]
    ///
    /// If filters is zero-length or nil, all items will be returned.
    #[prost(string, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContainersResponse {
    #[prost(message, repeated, tag = "1")]
    pub containers: ::prost::alloc::vec::Vec<Container>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContainerRequest {
    #[prost(message, optional, tag = "1")]
    pub container: ::core::option::Option<Container>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContainerResponse {
    #[prost(message, optional, tag = "1")]
    pub container: ::core::option::Option<Container>,
}
/// UpdateContainerRequest updates the metadata on one or more container.
///
/// The operation should follow semantics described in
/// <https://developers.google.com/protocol-buffers/docs/reference/csharp/class/google/protobuf/well-known-types/field-mask,>
/// unless otherwise qualified.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateContainerRequest {
    /// Container provides the target values, as declared by the mask, for the update.
    ///
    /// The ID field must be set.
    #[prost(message, optional, tag = "1")]
    pub container: ::core::option::Option<Container>,
    /// UpdateMask specifies which fields to perform the update on. If empty,
    /// the operation applies to all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateContainerResponse {
    #[prost(message, optional, tag = "1")]
    pub container: ::core::option::Option<Container>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteContainerRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContainerMessage {
    #[prost(message, optional, tag = "1")]
    pub container: ::core::option::Option<Container>,
}
#[doc = r" Generated client implementations."]
pub mod containers_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Containers provides metadata storage for containers used in the execution"]
    #[doc = " service."]
    #[doc = ""]
    #[doc = " The objects here provide an state-independent view of containers for use in"]
    #[doc = " management and resource pinning. From that perspective, containers do not"]
    #[doc = " have a \"state\" but rather this is the set of resources that will be"]
    #[doc = " considered in use by the container."]
    #[doc = ""]
    #[doc = " From the perspective of the execution service, these objects represent the"]
    #[doc = " base parameters for creating a container process."]
    #[doc = ""]
    #[doc = " In general, when looking to add fields for this type, first ask yourself"]
    #[doc = " whether or not the function of the field has to do with runtime execution or"]
    #[doc = " is invariant of the runtime state of the container. If it has to do with"]
    #[doc = " runtime, or changes as the \"container\" is started and stops, it probably"]
    #[doc = " doesn't belong on this object."]
    #[derive(Debug, Clone)]
    pub struct ContainersClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ContainersClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ContainersClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ContainersClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ContainersClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetContainerRequest>,
        ) -> Result<tonic::Response<super::GetContainerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.containers.v1.Containers/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContainersRequest>,
        ) -> Result<tonic::Response<super::ListContainersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.containers.v1.Containers/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContainersRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListContainerMessage>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.containers.v1.Containers/ListStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateContainerRequest>,
        ) -> Result<tonic::Response<super::CreateContainerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.containers.v1.Containers/Create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateContainerRequest>,
        ) -> Result<tonic::Response<super::UpdateContainerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.containers.v1.Containers/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteContainerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.containers.v1.Containers/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
