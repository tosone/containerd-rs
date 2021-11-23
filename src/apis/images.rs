#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// Name provides a unique name for the image.
    ///
    /// Containerd treats this as the primary identifier.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Labels provides free form labels for the image. These are runtime only
    /// and do not get inherited into the package image in any way.
    ///
    /// Labels may be updated using the field mask.
    /// The combined size of a key/value pair cannot exceed 4096 bytes.
    #[prost(map = "string, string", tag = "2")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Target describes the content entry point of the image.
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<super::types::Descriptor>,
    /// CreatedAt is the time the image was first created.
    #[prost(message, optional, tag = "7")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// UpdatedAt is the last time the image was mutated.
    #[prost(message, optional, tag = "8")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImageRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImageResponse {
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<Image>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateImageRequest {
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<Image>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateImageResponse {
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<Image>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateImageRequest {
    /// Image provides a full or partial image for update.
    ///
    /// The name field must be set or an error will be returned.
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<Image>,
    /// UpdateMask specifies which fields to perform the update on. If empty,
    /// the operation applies to all fields.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateImageResponse {
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<Image>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImagesRequest {
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
    #[prost(string, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImagesResponse {
    #[prost(message, repeated, tag = "1")]
    pub images: ::prost::alloc::vec::Vec<Image>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteImageRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Sync indicates that the delete and cleanup should be done
    /// synchronously before returning to the caller
    ///
    /// Default is false
    #[prost(bool, tag = "2")]
    pub sync: bool,
}
#[doc = r" Generated client implementations."]
pub mod images_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Images is a service that allows one to register images with containerd."]
    #[doc = ""]
    #[doc = " In containerd, an image is merely the mapping of a name to a content root,"]
    #[doc = " described by a descriptor. The behavior and state of image is purely"]
    #[doc = " dictated by the type of the descriptor."]
    #[doc = ""]
    #[doc = " From the perspective of this service, these references are mostly shallow,"]
    #[doc = " in that the existence of the required content won't be validated until"]
    #[doc = " required by consuming services."]
    #[doc = ""]
    #[doc = " As such, this can really be considered a \"metadata service\"."]
    #[derive(Debug, Clone)]
    pub struct ImagesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ImagesClient<tonic::transport::Channel> {
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
    impl<T> ImagesClient<T>
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
        ) -> ImagesClient<InterceptedService<T, F>>
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
            ImagesClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Get returns an image by name."]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetImageRequest>,
        ) -> Result<tonic::Response<super::GetImageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.images.v1.Images/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List returns a list of all images known to containerd."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListImagesRequest>,
        ) -> Result<tonic::Response<super::ListImagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.images.v1.Images/List");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create an image record in the metadata store."]
        #[doc = ""]
        #[doc = " The name of the image must be unique."]
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateImageRequest>,
        ) -> Result<tonic::Response<super::CreateImageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.images.v1.Images/Create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update assigns the name to a given target image based on the provided"]
        #[doc = " image."]
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateImageRequest>,
        ) -> Result<tonic::Response<super::UpdateImageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.images.v1.Images/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete deletes the image by name."]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteImageRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.images.v1.Images/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
