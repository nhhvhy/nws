use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();

    let res = client.get("http://testphp.vulnweb.com")
        .send()
        .await?;

    eprintln!("Headers: {:#?}\n", res.headers());
    Ok(())
}
