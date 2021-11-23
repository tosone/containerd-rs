#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyRequest {
    /// Diff is the descriptor of the diff to be extracted
    #[prost(message, optional, tag = "1")]
    pub diff: ::core::option::Option<super::super::super::types::Descriptor>,
    #[prost(message, repeated, tag = "2")]
    pub mounts: ::prost::alloc::vec::Vec<super::super::super::types::Mount>,
    #[prost(map = "string, message", tag = "3")]
    pub payloads: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyResponse {
    /// Applied is the descriptor for the object which was applied.
    /// If the input was a compressed blob then the result will be
    /// the descriptor for the uncompressed blob.
    #[prost(message, optional, tag = "1")]
    pub applied: ::core::option::Option<super::super::super::types::Descriptor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiffRequest {
    /// Left are the mounts which represent the older copy
    /// in which is the base of the computed changes.
    #[prost(message, repeated, tag = "1")]
    pub left: ::prost::alloc::vec::Vec<super::super::super::types::Mount>,
    /// Right are the mounts which represents the newer copy
    /// in which changes from the left were made into.
    #[prost(message, repeated, tag = "2")]
    pub right: ::prost::alloc::vec::Vec<super::super::super::types::Mount>,
    /// MediaType is the media type descriptor for the created diff
    /// object
    #[prost(string, tag = "3")]
    pub media_type: ::prost::alloc::string::String,
    /// Ref identifies the pre-commit content store object. This
    /// reference can be used to get the status from the content store.
    #[prost(string, tag = "4")]
    pub r#ref: ::prost::alloc::string::String,
    /// Labels are the labels to apply to the generated content
    /// on content store commit.
    #[prost(map = "string, string", tag = "5")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiffResponse {
    /// Diff is the descriptor of the diff which can be applied
    #[prost(message, optional, tag = "3")]
    pub diff: ::core::option::Option<super::super::super::types::Descriptor>,
}
#[doc = r" Generated client implementations."]
pub mod diff_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Diff service creates and applies diffs"]
    #[derive(Debug, Clone)]
    pub struct DiffClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DiffClient<tonic::transport::Channel> {
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
    impl<T> DiffClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> DiffClient<InterceptedService<T, F>>
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
            DiffClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Apply applies the content associated with the provided digests onto"]
        #[doc = " the provided mounts. Archive content will be extracted and"]
        #[doc = " decompressed if necessary."]
        pub async fn apply(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyRequest>,
        ) -> Result<tonic::Response<super::ApplyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.diff.v1.Diff/Apply");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Diff creates a diff between the given mounts and uploads the result"]
        #[doc = " to the content store."]
        pub async fn diff(
            &mut self,
            request: impl tonic::IntoRequest<super::DiffRequest>,
        ) -> Result<tonic::Response<super::DiffResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.diff.v1.Diff/Diff");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
