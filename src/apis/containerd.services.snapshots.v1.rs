#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareSnapshotRequest {
    #[prost(string, tag = "1")]
    pub snapshotter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Labels are arbitrary data on snapshots.
    ///
    /// The combined size of a key/value pair cannot exceed 4096 bytes.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareSnapshotResponse {
    #[prost(message, repeated, tag = "1")]
    pub mounts: ::prost::alloc::vec::Vec<super::super::super::types::Mount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewSnapshotRequest {
    #[prost(string, tag = "1")]
    pub snapshotter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Labels are arbitrary data on snapshots.
    ///
    /// The combined size of a key/value pair cannot exceed 4096 bytes.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewSnapshotResponse {
    #[prost(message, repeated, tag = "1")]
    pub mounts: ::prost::alloc::vec::Vec<super::super::super::types::Mount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MountsRequest {
    #[prost(string, tag = "1")]
    pub snapshotter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MountsResponse {
    #[prost(message, repeated, tag = "1")]
    pub mounts: ::prost::alloc::vec::Vec<super::super::super::types::Mount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSnapshotRequest {
    #[prost(string, tag = "1")]
    pub snapshotter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitSnapshotRequest {
    #[prost(string, tag = "1")]
    pub snapshotter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
    /// Labels are arbitrary data on snapshots.
    ///
    /// The combined size of a key/value pair cannot exceed 4096 bytes.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatSnapshotRequest {
    #[prost(string, tag = "1")]
    pub snapshotter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Info {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    #[prost(enumeration = "Kind", tag = "3")]
    pub kind: i32,
    /// CreatedAt provides the time at which the snapshot was created.
    #[prost(message, optional, tag = "4")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// UpdatedAt provides the time the info was last updated.
    #[prost(message, optional, tag = "5")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels are arbitrary data on snapshots.
    ///
    /// The combined size of a key/value pair cannot exceed 4096 bytes.
    #[prost(map = "string, string", tag = "6")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatSnapshotResponse {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<Info>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSnapshotRequest {
    #[prost(string, tag = "1")]
    pub snapshotter: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<Info>,
    /// UpdateMask specifies which fields to perform the update on. If empty,
    /// the operation applies to all fields.
    ///
    /// In info, Name, Parent, Kind, Created are immutable,
    /// other field may be updated using this mask.
    /// If no mask is provided, all mutable field are updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSnapshotResponse {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<Info>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsRequest {
    #[prost(string, tag = "1")]
    pub snapshotter: ::prost::alloc::string::String,
    /// Filters contains one or more filters using the syntax defined in the
    /// containerd filter package.
    ///
    /// The returned result will be those that match any of the provided
    /// filters. Expanded, images that match the following will be
    /// returned:
    ///
    ///   filters\[0\] or filters\[1\] or ... or filters\[n-1\] or filters\[n\]
    ///
    /// If filters is zero-length or nil, all items will be returned.
    #[prost(string, repeated, tag = "2")]
    pub filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsResponse {
    #[prost(message, repeated, tag = "1")]
    pub info: ::prost::alloc::vec::Vec<Info>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageRequest {
    #[prost(string, tag = "1")]
    pub snapshotter: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageResponse {
    #[prost(int64, tag = "1")]
    pub size: i64,
    #[prost(int64, tag = "2")]
    pub inodes: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CleanupRequest {
    #[prost(string, tag = "1")]
    pub snapshotter: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Kind {
    Unknown = 0,
    View = 1,
    Active = 2,
    Committed = 3,
}
#[doc = r" Generated client implementations."]
pub mod snapshots_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Snapshot service manages snapshots"]
    #[derive(Debug, Clone)]
    pub struct SnapshotsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SnapshotsClient<tonic::transport::Channel> {
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
    impl<T> SnapshotsClient<T>
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
        ) -> SnapshotsClient<InterceptedService<T, F>>
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
            SnapshotsClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn prepare(
            &mut self,
            request: impl tonic::IntoRequest<super::PrepareSnapshotRequest>,
        ) -> Result<tonic::Response<super::PrepareSnapshotResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.snapshots.v1.Snapshots/Prepare",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn view(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewSnapshotRequest>,
        ) -> Result<tonic::Response<super::ViewSnapshotResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.snapshots.v1.Snapshots/View",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn mounts(
            &mut self,
            request: impl tonic::IntoRequest<super::MountsRequest>,
        ) -> Result<tonic::Response<super::MountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.snapshots.v1.Snapshots/Mounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn commit(
            &mut self,
            request: impl tonic::IntoRequest<super::CommitSnapshotRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.snapshots.v1.Snapshots/Commit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveSnapshotRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.snapshots.v1.Snapshots/Remove",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn stat(
            &mut self,
            request: impl tonic::IntoRequest<super::StatSnapshotRequest>,
        ) -> Result<tonic::Response<super::StatSnapshotResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.snapshots.v1.Snapshots/Stat",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSnapshotRequest>,
        ) -> Result<tonic::Response<super::UpdateSnapshotResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.snapshots.v1.Snapshots/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSnapshotsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListSnapshotsResponse>>,
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
                "/containerd.services.snapshots.v1.Snapshots/List",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn usage(
            &mut self,
            request: impl tonic::IntoRequest<super::UsageRequest>,
        ) -> Result<tonic::Response<super::UsageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.snapshots.v1.Snapshots/Usage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cleanup(
            &mut self,
            request: impl tonic::IntoRequest<super::CleanupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.snapshots.v1.Snapshots/Cleanup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
