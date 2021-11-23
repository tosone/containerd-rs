#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishRequest {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardRequest {
    #[prost(message, optional, tag = "1")]
    pub envelope: ::core::option::Option<Envelope>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRequest {
    #[prost(string, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Envelope {
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "2")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub topic: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub event: ::core::option::Option<::prost_types::Any>,
}
#[doc = r" Generated client implementations."]
pub mod events_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct EventsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EventsClient<tonic::transport::Channel> {
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
    impl<T> EventsClient<T>
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
        ) -> EventsClient<InterceptedService<T, F>>
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
            EventsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Publish an event to a topic."]
        #[doc = ""]
        #[doc = " The event will be packed into a timestamp envelope with the namespace"]
        #[doc = " introspected from the context. The envelope will then be dispatched."]
        pub async fn publish(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.events.v1.Events/Publish",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Forward sends an event that has already been packaged into an envelope"]
        #[doc = " with a timestamp and namespace."]
        #[doc = ""]
        #[doc = " This is useful if earlier timestamping is required or when forwarding on"]
        #[doc = " behalf of another component, namespace or publisher."]
        pub async fn forward(
            &mut self,
            request: impl tonic::IntoRequest<super::ForwardRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.events.v1.Events/Forward",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Subscribe to a stream of events, possibly returning only that match any"]
        #[doc = " of the provided filters."]
        #[doc = ""]
        #[doc = " Unlike many other methods in containerd, subscribers will get messages"]
        #[doc = " from all namespaces unless otherwise specified. If this is not desired,"]
        #[doc = " a filter can be provided in the format 'namespace==<namespace>' to"]
        #[doc = " restrict the received events."]
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Envelope>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.events.v1.Events/Subscribe",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
