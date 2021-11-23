#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Process {
    #[prost(string, tag = "1")]
    pub container_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub pid: u32,
    #[prost(enumeration = "Status", tag = "4")]
    pub status: i32,
    #[prost(string, tag = "5")]
    pub stdin: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub stdout: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub stderr: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub terminal: bool,
    #[prost(uint32, tag = "9")]
    pub exit_status: u32,
    #[prost(message, optional, tag = "10")]
    pub exited_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessInfo {
    /// PID is the process ID.
    #[prost(uint32, tag = "1")]
    pub pid: u32,
    /// Info contains additional process information.
    ///
    /// Info varies by platform.
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Unknown = 0,
    Created = 1,
    Running = 2,
    Stopped = 3,
    Paused = 4,
    Pausing = 5,
}
