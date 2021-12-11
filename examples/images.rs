use containerd::client::Client;

#[tokio::main]
async fn example(client: Client) {
    let result = client.list_images("default".to_string()).await;
    match result {
        Ok(images) => {
            for image in images {
                println!("{}", image);
            }
        }
        Err(e) => println!("{}", e),
    }
}

fn main() -> Result<(), std::io::Error> {
    let client = Client::new(None, None, None);
    example(client);
    Ok(())
}
