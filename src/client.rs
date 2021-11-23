// use ttrpc::client::Client as ttClient;
// use ttrpc::context;

// use crate::apis::images::ListImagesRequest;
// use crate::apis::images_ttrpc::ImagesClient;

#[derive(Clone, Debug)]
pub struct Client {
    #[warn(dead_code)]
    platform: Option<String>,
}

impl Client {
    #[allow(dead_code)]
    fn new() -> Client {
        let c = Client {
            platform: Some("linux".to_string()),
        };
        // let m = ttClient::connect("unix:///run/containerd/containerd.sock").unwrap();
        // let images_client = ImagesClient::new(m);
        // let ctx = context::with_timeout(0);
        // let req = ListImagesRequest {
        //     filters: ::protobuf::RepeatedField::new(),
        //     unknown_fields: ::protobuf::UnknownFields::new(),
        //     ..ListImagesRequest::default()
        // };
        // let result = images_client.list(ctx, &req);
        // println!("{:?}", result);
        c
    }
    #[allow(dead_code)]
    fn connect(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let client = Client::new();
        assert!(client.connect());
    }
}
