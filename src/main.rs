use std::time::Duration;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let local_url = "http://127.0.0.1:9052";
    let remote_url = "http://213.239.193.208:9053";
    let client = ergo_client::node::NodeClient::from_url_str(
        local_url,
        "hello".to_owned(),
        Duration::from_secs(10),
    )?;
    let endpoints = client.endpoints();
    let extensions = client.extensions();

    dbg!(endpoints.root()?.info().await?);
    dbg!(endpoints.wallet()?.status().await?);
    dbg!(endpoints.wallet()?.boxes()?.unspent(None).await?);
    dbg!(extensions.get_utxos_summing_amount(10000).await?);

    Ok(())
}
