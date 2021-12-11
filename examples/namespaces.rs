use containerd::client::Client;

#[tokio::main]
async fn example(client: Client) {
    let result = client.list_namespaces().await;
    match result {
        Ok(namespaces) => {
            for namespace in namespaces {
                println!("{}", namespace.name);
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
