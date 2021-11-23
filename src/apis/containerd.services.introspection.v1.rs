#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plugin {
    /// Type defines the type of plugin.
    ///
    /// See package plugin for a list of possible values. Non core plugins may
    /// define their own values during registration.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// ID identifies the plugin uniquely in the system.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Requires lists the plugin types required by this plugin.
    #[prost(string, repeated, tag = "3")]
    pub requires: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Platforms enumerates the platforms this plugin will support.
    ///
    /// If values are provided here, the plugin will only be operable under the
    /// provided platforms.
    ///
    /// If this is empty, the plugin will work across all platforms.
    ///
    /// If the plugin prefers certain platforms over others, they should be
    /// listed from most to least preferred.
    #[prost(message, repeated, tag = "4")]
    pub platforms: ::prost::alloc::vec::Vec<super::super::super::types::Platform>,
    /// Exports allows plugins to provide values about state or configuration to
    /// interested parties.
    ///
    /// One example is exposing the configured path of a snapshotter plugin.
    #[prost(map = "string, string", tag = "5")]
    pub exports:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Capabilities allows plugins to communicate feature switches to allow
    /// clients to detect features that may not be on be default or may be
    /// different from version to version.
    ///
    /// Use this sparingly.
    #[prost(string, repeated, tag = "6")]
    pub capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// InitErr will be set if the plugin fails initialization.
    ///
    /// This means the plugin may have been registered but a non-terminal error
    /// was encountered during initialization.
    ///
    /// Plugins that have this value set cannot be used.
    #[prost(message, optional, tag = "7")]
    pub init_err: ::core::option::Option<super::super::super::super::google::rpc::Status>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginsRequest {
    /// Filters contains one or more filters using the syntax defined in the
    /// containerd filter package.
    ///
    /// The returned result will be those that match any of the provided
    /// filters. Expanded, plugins that match the following will be
    /// returned:
    ///
    ///   filters\[0\] or filters\[1\] or ... or filters\[n-1\] or filters\[n\]
    ///
    /// If filters is zero-length or nil, all items will be returned.
    #[prost(string, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginsResponse {
    #[prost(message, repeated, tag = "1")]
    pub plugins: ::prost::alloc::vec::Vec<Plugin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerResponse {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod introspection_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct IntrospectionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntrospectionClient<tonic::transport::Channel> {
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
    impl<T> IntrospectionClient<T>
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
        ) -> IntrospectionClient<InterceptedService<T, F>>
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
            IntrospectionClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Plugins returns a list of plugins in containerd."]
        #[doc = ""]
        #[doc = " Clients can use this to detect features and capabilities when using"]
        #[doc = " containerd."]
        pub async fn plugins(
            &mut self,
            request: impl tonic::IntoRequest<super::PluginsRequest>,
        ) -> Result<tonic::Response<super::PluginsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.introspection.v1.Introspection/Plugins",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Server returns information about the containerd server"]
        pub async fn server(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::ServerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.introspection.v1.Introspection/Server",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
