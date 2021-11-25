use std::convert::TryFrom;
use std::string::String;
use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;

pub mod namespaces {
    include!(concat!("./apis/containerd.services.namespaces.v1.rs"));
}

use namespaces::namespaces_client::NamespacesClient;
use namespaces::ListNamespacesRequest;

pub struct Client {
    pub runtime: Option<String>,
    pub defaultns: Option<String>,
    pub address: String,
}

impl Client {
    #[tokio::main]
    pub async fn new(
        runtime: Option<String>,
        defaultns: Option<String>,
        address: Option<String>,
    ) -> Self {
        let address = match address {
            Some(address) => address,
            None => "/run/containerd/containerd.sock".to_string(),
        };
        Self {
            runtime: Some(runtime.unwrap_or("linux".to_string())),
            defaultns: Some(defaultns.unwrap_or("default".to_string())),
            address,
        }
    }

    pub async fn connect(self) -> Result<Channel, Box<dyn std::error::Error>> {
        let channel = Endpoint::try_from("http://127.0.0.1:80")?
            .connect_with_connector(service_fn(|_: Uri| {
                let conn = "/run/containerd/containerd.sock";
                UnixStream::connect(conn)
            }))
            .await?;
        Ok(channel)
    }

    pub async fn list_namespaces(self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let channel = self.connect().await?;
        let mut client = NamespacesClient::new(channel);
        let request = tonic::Request::new(ListNamespacesRequest { filter: "".into() });
        let response = client.list(request).await?;
        let mut result = Vec::new();
        for item in response.into_inner().namespaces {
            println!("{:?}", item);
            result.push(item.name);
        }
        Ok(result)
    }
}
