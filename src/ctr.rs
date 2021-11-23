pub mod hello_world {
    // tonic::include_proto!("containerd.services.namespaces.v1");
    include!(concat!("apis/", "containerd.services.namespaces.v1", ".rs"));
    // include!(concat!("apis/", "containerd.types", ".rs"));
    // include!(concat!("apis/", "containerd.services.images.v1", ".rs"));
}

use hello_world::namespaces_client::NamespacesClient;
use hello_world::ListNamespacesRequest;

use std::convert::TryFrom;
#[cfg(unix)]
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = tonic::Request::new(ListNamespacesRequest {
        filter: "sss".into(),
    });
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| {
            let conn = "/run/containerd/containerd.sock";
            UnixStream::connect(conn)
        }))
        .await?;
    let mut client = NamespacesClient::new(channel);
    let result = client.list(request).await?;
    // for item in result. {
    //     println!("{:?}", item);
    // }
    // let a = result.unwrap();
    for item in result.into_inner().namespaces {
        println!("{:?}", item.name);
        println!("{:?}", item.labels);
    }
    // println!("{:?}", result.namespaces);

    Ok(())
}
