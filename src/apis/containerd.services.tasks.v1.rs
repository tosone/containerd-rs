#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    /// RootFS provides the pre-chroot mounts to perform in the shim before
    /// executing the container task.
    ///
    /// These are for mounts that cannot be performed in the user namespace.
    /// Typically, these mounts should be resolved from snapshots specified on
    /// the container object.
    #[prost(message, repeated, tag = "3")]
    pub rootfs: ::prost::alloc::vec::Vec<super::super::super::types::Mount>,
    #[prost(string, tag = "4")]
    pub stdin: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub stdout: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub stderr: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub terminal: bool,
    #[prost(message, optional, tag = "8")]
    pub checkpoint: ::core::option::Option<super::super::super::types::Descriptor>,
    #[prost(message, optional, tag = "9")]
    pub options: ::core::option::Option<::prost_types::Any>,
    #[prost(string, tag = "10")]
    pub runtime_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTaskResponse {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub pid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exec_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartResponse {
    #[prost(uint32, tag = "1")]
    pub pid: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTaskRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub pid: u32,
    #[prost(uint32, tag = "3")]
    pub exit_status: u32,
    #[prost(message, optional, tag = "4")]
    pub exited_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProcessRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exec_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exec_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag = "1")]
    pub process: ::core::option::Option<super::super::super::v1::types::Process>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksRequest {
    #[prost(string, tag = "1")]
    pub filter: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksResponse {
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<super::super::super::v1::types::Process>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KillRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exec_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub signal: u32,
    #[prost(bool, tag = "4")]
    pub all: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecProcessRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub stdin: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub stdout: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub stderr: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub terminal: bool,
    /// Spec for starting a process in the target container.
    ///
    /// For runc, this is a process spec, for example.
    #[prost(message, optional, tag = "6")]
    pub spec: ::core::option::Option<::prost_types::Any>,
    /// id of the exec process
    #[prost(string, tag = "7")]
    pub exec_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecProcessResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResizePtyRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exec_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub width: u32,
    #[prost(uint32, tag = "4")]
    pub height: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseIoRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exec_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub stdin: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseTaskRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeTaskRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPidsRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPidsResponse {
    /// Processes includes the process ID and additional process information
    #[prost(message, repeated, tag = "1")]
    pub processes: ::prost::alloc::vec::Vec<super::super::super::v1::types::ProcessInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckpointTaskRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub parent_checkpoint: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub options: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckpointTaskResponse {
    #[prost(message, repeated, tag = "1")]
    pub descriptors: ::prost::alloc::vec::Vec<super::super::super::types::Descriptor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub resources: ::core::option::Option<::prost_types::Any>,
    #[prost(map = "string, string", tag = "3")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricsRequest {
    #[prost(string, repeated, tag = "1")]
    pub filters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricsResponse {
    #[prost(message, repeated, tag = "1")]
    pub metrics: ::prost::alloc::vec::Vec<super::super::super::types::Metric>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitRequest {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exec_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitResponse {
    #[prost(uint32, tag = "1")]
    pub exit_status: u32,
    #[prost(message, optional, tag = "2")]
    pub exited_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[doc = r" Generated client implementations."]
pub mod tasks_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct TasksClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TasksClient<tonic::transport::Channel> {
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
    impl<T> TasksClient<T>
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
        ) -> TasksClient<InterceptedService<T, F>>
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
            TasksClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Create a task."]
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTaskRequest>,
        ) -> Result<tonic::Response<super::CreateTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/Create");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Start a process."]
        pub async fn start(
            &mut self,
            request: impl tonic::IntoRequest<super::StartRequest>,
        ) -> Result<tonic::Response<super::StartResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/Start");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete a task and on disk state."]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTaskRequest>,
        ) -> Result<tonic::Response<super::DeleteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/Delete");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_process(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProcessRequest>,
        ) -> Result<tonic::Response<super::DeleteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.tasks.v1.Tasks/DeleteProcess",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTasksRequest>,
        ) -> Result<tonic::Response<super::ListTasksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/List");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Kill a task or process."]
        pub async fn kill(
            &mut self,
            request: impl tonic::IntoRequest<super::KillRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/Kill");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exec(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecProcessRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/Exec");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resize_pty(
            &mut self,
            request: impl tonic::IntoRequest<super::ResizePtyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.tasks.v1.Tasks/ResizePty",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn close_io(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseIoRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/CloseIO");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn pause(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseTaskRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/Pause");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resume(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeTaskRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/Resume");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_pids(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPidsRequest>,
        ) -> Result<tonic::Response<super::ListPidsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.tasks.v1.Tasks/ListPids",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn checkpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckpointTaskRequest>,
        ) -> Result<tonic::Response<super::CheckpointTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/containerd.services.tasks.v1.Tasks/Checkpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTaskRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/Update");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::MetricsRequest>,
        ) -> Result<tonic::Response<super::MetricsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/Metrics");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn wait(
            &mut self,
            request: impl tonic::IntoRequest<super::WaitRequest>,
        ) -> Result<tonic::Response<super::WaitResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/containerd.services.tasks.v1.Tasks/Wait");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
