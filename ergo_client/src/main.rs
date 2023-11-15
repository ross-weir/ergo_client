pub fn main() {
    let client =
        ergo_client::node::NodeClient::from_url_str("http://localhost:9052", "hello".to_owned())
            .unwrap();
    let resp = client.info_blocking().unwrap();
    println!("{:?}", resp);
}
