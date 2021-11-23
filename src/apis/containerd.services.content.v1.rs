#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Info {
    /// Digest is the hash identity of the blob.
    #[prost(string, tag = "1")]
    pub digest: ::prost::alloc::string::String,
    /// Size is the total number of bytes in the blob.
    #[prost(int64, tag = "2")]
    pub size: i64,
    /// CreatedAt provides the time at which the blob was committed.
    #[prost(message, optional, tag = "3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// UpdatedAt provides the time the info was last updated.
    #[prost(message, optional, tag = "4")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels are arbitrary data on snapshots.
    ///
    /// The combined size of a key/value pair cannot exceed 4096 bytes.
    #[prost(map = "string, string", tag = "5")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoRequest {
    #[prost(string, tag = "1")]
    pub digest: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResponse {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<Info>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<Info>,
    /// UpdateMask specifies which fields to perform the update on. If empty,
    /// the operation applies to all fields.
    ///
    /// In info, Digest, Size, and CreatedAt are immutable,
    /// other field may be updated using this mask.
    /// If no mask is provided, all mutable field are updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<Info>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContentRequest {
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
pub struct ListContentResponse {
    #[prost(message, repeated, tag = "1")]
    pub info: ::prost::alloc::vec::Vec<Info>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteContentRequest {
    /// Digest specifies which content to delete.
    #[prost(string, tag = "1")]
    pub digest: ::prost::alloc::string::String,
}
/// ReadContentRequest defines the fields that make up a request to read a portion of
/// data from a stored object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadContentRequest {
    /// Digest is the hash identity to read.
    #[prost(string, tag = "1")]
    pub digest: ::prost::alloc::string::String,
    /// Offset specifies the number of bytes from the start at which to begin
    /// the read. If zero or less, the read will be from the start. This uses
    /// standard zero-indexed semantics.
    #[prost(int64, tag = "2")]
    pub offset: i64,
    /// size is the total size of the read. If zero, the entire blob will be
    /// returned by the service.
    #[prost(int64, tag = "3")]
    pub size: i64,
}
/// ReadContentResponse carries byte data for a read request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadContentResponse {
    /// offset of the returned data
    #[prost(int64, tag = "1")]
    pub offset: i64,
    /// actual data
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(message, optional, tag = "1")]
    pub started_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "3")]
    pub r#ref: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub offset: i64,
    #[prost(int64, tag = "5")]
    pub total: i64,
    #[prost(string, tag = "6")]
    pub expected: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {
    #[prost(string, tag = "1")]
    pub r#ref: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<Status>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStatusesRequest {
    #[prost(string, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStatusesResponse {
    #[prost(message, repeated, tag = "1")]
    pub statuses: ::prost::alloc::vec::Vec<Status>,
}
/// WriteContentRequest writes data to the request ref at offset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteContentRequest {
    /// Action sets the behavior of the write.
    ///
    /// When this is a write and the ref is not yet allocated, the ref will be
    /// allocated and the data will be written at offset.
    ///
    /// If the action is write and the ref is allocated, it will accept data to
    /// an offset that has not yet been written.
    ///
    /// If the action is write and there is no data, the current write status
    /// will be returned. This works differently from status because the stream
    /// holds a lock.
    #[prost(enumeration = "WriteAction", tag = "1")]
    pub action: i32,
    /// Ref identifies the pre-commit object to write to.
    #[prost(string, tag = "2")]
    pub r#ref: ::prost::alloc::string::String,
    /// Total can be set to have the service validate the total size of the
    /// committed content.
    ///
    /// The latest value before or with the commit action message will be use to
    /// validate the content. If the offset overflows total, the service may
    /// report an error. It is only required on one message for the write.
    ///
    /// If the value is zero or less, no validation of the final content will be
    /// performed.
    #[prost(int64, tag = "3")]
    pub total: i64,
    /// Expected can be set to have the service validate the final content against
    /// the provided digest.
    ///
    /// If the digest is already present in the object store, an AlreadyExists
    /// error will be returned.
    ///
    /// Only the latest version will be used to check the content against the
    /// digest. It is only required to include it on a single message, before or
    /// with the commit action message.
    #[prost(string, tag = "4")]
    pub expected: ::prost::alloc::string::String,
    /// Offset specifies the number of bytes from the start at which to begin
    /// the write. For most implementations, this means from the start of the
    /// file. This uses standard, zero-indexed semantics.
    ///
    /// If the action is write, the remote may remove all previously written
    /// data after the offset. Implementations may support arbitrary offsets but
    /// MUST support reseting this value to zero with a write. If an
    /// implementation does not support a write at a particular offset, an
    /// OutOfRange error must be returned.
    #[prost(int64, tag = "5")]
    pub offset: i64,
    /// Data is the actual bytes to be written.
    ///
    /// If this is empty and the message is not a commit, a response will be
    /// returned with the current write state.
    #[prost(bytes = "vec", tag = "6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Labels are arbitrary data on snapshots.
    ///
    /// The combined size of a key/value pair cannot exceed 4096 bytes.
    #[prost(map = "string, string", tag = "7")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// WriteContentResponse is returned on the culmination of a write call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteContentResponse {
    /// Action contains the action for the final message of the stream. A writer
    /// should confirm that they match the intended result.
    #[prost(enumeration = "WriteAction", tag = "1")]
    pub action: i32,
    /// StartedAt provides the time at which the write began.
    ///
    /// This must be set for stat and commit write actions. All other write
    /// actions may omit this.
    #[prost(message, optional, tag = "2")]
    pub started_at: ::core::option::Option<::prost_types::Timestamp>,
    /// UpdatedAt provides the last time of a successful write.
    ///
    /// This must be set for stat and commit write actions. All other write
    /// actions may omit this.
    #[prost(message, optional, tag = "3")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Offset is the current committed size for the write.
    #[prost(int64, tag = "4")]
    pub offset: i64,
    /// Total provides the current, expected total size of the write.
    ///
    /// We include this to provide consistency with the Status structure on the
    /// client writer.
    ///
    /// This is only valid on the Stat and Commit response.
    #[prost(int64, tag = "5")]
    pub total: i64,
    /// Digest, if present, includes the digest up to the currently committed
    /// bytes. If action is commit, this field will be set. It is implementation
    /// defined if this is set for other actions.
    #[prost(string, tag = "6")]
    pub digest: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbortRequest {
    #[prost(string, tag = "1")]
    pub r#ref: ::prost::alloc::string::String,
}
/// WriteAction defines the behavior of a WriteRequest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WriteAction {
    /// WriteActionStat instructs the writer to return the current status while
    /// holding the lock on the write.
    Stat = 0,
    /// WriteActionWrite sets the action for the write request to write data.
    ///
    /// Any data included will be written at the provided offset. The
    /// transaction will be left open for further writes.
    ///
    /// This is the default.
    Write = 1,
    /// WriteActionCommit will write any outstanding data in the message and
    /// commit the write, storing it under the digest.
    ///
    /// This can be used in a single message to send the data, verify it and
    /// commit it.
    ///
    /// This action will always terminate the write.
    Commit = 2,
}
#[doc = r" Generated client implementations."]
pub mod content_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Content provides access to a content addressable storage system."]
    #[derive(Debug, Clone)]
    pub struct ContentClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ContentClient<tonic::transport::Channel> {
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
    impl<T> ContentClient<T>
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
        ) -> ContentClient<InterceptedService<T, F>>
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
            ContentClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Info returns information about a committed object."]
        #[doc = ""]
        #[doc = " This call can be used for getting the size of content and checking for"]
        #[doc = " existence."]
        pub async fn info(
            &mut self,
            request: impl tonic::IntoRequest<super::InfoRequest>,
        ) -> Result<tonic::Response<super::InfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.content.v1.Content/Info",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update updates content metadata."]
        #[doc = ""]
        #[doc = " This call can be used to manage the mutable content labels. The"]
        #[doc = " immutable metadata such as digest, size, and committed at cannot"]
        #[doc = " be updated."]
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRequest>,
        ) -> Result<tonic::Response<super::UpdateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.content.v1.Content/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List streams the entire set of content as Info objects and closes the"]
        #[doc = " stream."]
        #[doc = ""]
        #[doc = " Typically, this will yield a large response, chunked into messages."]
        #[doc = " Clients should make provisions to ensure they can handle the entire data"]
        #[doc = " set."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContentRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListContentResponse>>,
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
                "/containerd.services.content.v1.Content/List",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Delete will delete the referenced object."]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteContentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.content.v1.Content/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Read allows one to read an object based on the offset into the content."]
        #[doc = ""]
        #[doc = " The requested data may be returned in one or more messages."]
        pub async fn read(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadContentRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ReadContentResponse>>,
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
                "/containerd.services.content.v1.Content/Read",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Status returns the status for a single reference."]
        pub async fn status(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusRequest>,
        ) -> Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.content.v1.Content/Status",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ListStatuses returns the status of ongoing object ingestions, started via"]
        #[doc = " Write."]
        #[doc = ""]
        #[doc = " Only those matching the regular expression will be provided in the"]
        #[doc = " response. If the provided regular expression is empty, all ingestions"]
        #[doc = " will be provided."]
        pub async fn list_statuses(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStatusesRequest>,
        ) -> Result<tonic::Response<super::ListStatusesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.content.v1.Content/ListStatuses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Write begins or resumes writes to a resource identified by a unique ref."]
        #[doc = " Only one active stream may exist at a time for each ref."]
        #[doc = ""]
        #[doc = " Once a write stream has started, it may only write to a single ref, thus"]
        #[doc = " once a stream is started, the ref may be omitted on subsequent writes."]
        #[doc = ""]
        #[doc = " For any write transaction represented by a ref, only a single write may"]
        #[doc = " be made to a given offset. If overlapping writes occur, it is an error."]
        #[doc = " Writes should be sequential and implementations may throw an error if"]
        #[doc = " this is required."]
        #[doc = ""]
        #[doc = " If expected_digest is set and already part of the content store, the"]
        #[doc = " write will fail."]
        #[doc = ""]
        #[doc = " When completed, the commit flag should be set to true. If expected size"]
        #[doc = " or digest is set, the content will be validated against those values."]
        pub async fn write(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::WriteContentRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::WriteContentResponse>>,
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
                "/containerd.services.content.v1.Content/Write",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Abort cancels the ongoing write named in the request. Any resources"]
        #[doc = " associated with the write will be collected."]
        pub async fn abort(
            &mut self,
            request: impl tonic::IntoRequest<super::AbortRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.content.v1.Content/Abort",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
