pub mod hello_world {
    include!(concat!("../apis/containerd.services.namespaces.v1.rs"));
}

use hello_world::namespaces_client::NamespacesClient;
use hello_world::ListNamespacesRequest;

use std::convert::TryFrom;
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: tonic need get endpoint instance from try_from, but it is useless
    let channel = Endpoint::try_from("http://127.0.0.1:80")?
        .connect_with_connector(service_fn(|_: Uri| {
            let conn = "/run/containerd/containerd.sock";
            UnixStream::connect(conn)
        }))
        .await?;
    let mut client = NamespacesClient::new(channel);

    let request = tonic::Request::new(ListNamespacesRequest {
        filter: "sss".into(),
    });
    let result = client.list(request).await?;

    for item in result.into_inner().namespaces {
        println!("{:?}", item.name);
        println!("{:?}", item.labels);
    }

    Ok(())
}
