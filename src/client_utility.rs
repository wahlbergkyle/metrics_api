use surreal_simple_client::SurrealClient;

pub async fn get_client() -> SurrealClient {
    println!("Attempting to make ws connection");
    let mut client = SurrealClient::new("ws://sdb.52.23.170.220.nip.io:8000/rpc")
        .await
        .expect("RPC handshake error");

    println!("Connected!");

    client.signin("root", "root").await.expect("Signin error");
    client
        .use_namespace("kwahl", "sampleDBname")
        .await
        .expect("Namespace error");

    println!("Signed in!");
    client
}
