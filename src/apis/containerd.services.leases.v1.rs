/// Lease is an object which retains resources while it exists.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lease {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(map = "string, string", tag = "3")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    /// ID is used to identity the lease, when the id is not set the service
    /// generates a random identifier for the lease.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "3")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResponse {
    #[prost(message, optional, tag = "1")]
    pub lease: ::core::option::Option<Lease>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Sync indicates that the delete and cleanup should be done
    /// synchronously before returning to the caller
    ///
    /// Default is false
    #[prost(bool, tag = "2")]
    pub sync: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRequest {
    #[prost(string, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResponse {
    #[prost(message, repeated, tag = "1")]
    pub leases: ::prost::alloc::vec::Vec<Lease>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// For snapshotter resource, there are many snapshotter types here, like
    /// overlayfs, devmapper etc. The type will be formatted with type,
    /// like "snapshotter/overlayfs".
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddResourceRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<Resource>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResourceRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub resource: ::core::option::Option<Resource>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResourcesRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResourcesResponse {
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
}
#[doc = r" Generated client implementations."]
pub mod leases_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Leases service manages resources leases within the metadata store."]
    #[derive(Debug, Clone)]
    pub struct LeasesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LeasesClient<tonic::transport::Channel> {
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
    impl<T> LeasesClient<T>
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
        ) -> LeasesClient<InterceptedService<T, F>>
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
            LeasesClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Create creates a new lease for managing changes to metadata. A lease"]
        #[doc = " can be used to protect objects from being removed."]
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRequest>,
        ) -> Result<tonic::Response<super::CreateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.leases.v1.Leases/Create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete deletes the lease and makes any unreferenced objects created"]
        #[doc = " during the lease eligible for garbage collection if not referenced"]
        #[doc = " or retained by other resources during the lease."]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.leases.v1.Leases/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List lists all active leases, returning the full list of"]
        #[doc = " leases and optionally including the referenced resources."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRequest>,
        ) -> Result<tonic::Response<super::ListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.leases.v1.Leases/List");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " AddResource references the resource by the provided lease."]
        pub async fn add_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::AddResourceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.leases.v1.Leases/AddResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DeleteResource dereferences the resource by the provided lease."]
        pub async fn delete_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteResourceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.leases.v1.Leases/DeleteResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ListResources lists all the resources referenced by the lease."]
        pub async fn list_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListResourcesRequest>,
        ) -> Result<tonic::Response<super::ListResourcesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.leases.v1.Leases/ListResources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
