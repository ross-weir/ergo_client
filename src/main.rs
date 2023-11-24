use std::time::Duration;

#[tokio::main]
pub async fn main() {
    let local_url = "http://127.0.0.1:9052";
    let remote_url = "http://213.239.193.208:9053";
    let client = ergo_client::node::NodeClient::from_url_str(
        local_url,
        "hello".to_owned(),
        Duration::from_secs(10),
    )
    .unwrap();

    dbg!(client.root().info().await.unwrap());
    dbg!(client.wallet().status().await.unwrap());
    dbg!(client.wallet().boxes().unspent(None).await.unwrap());
}
