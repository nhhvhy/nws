use std::process::Command;
use std::str;
use std::error::Error;
use reqwest::Error as ReqwestError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    fetch_headers("http://testphp.vulnweb.com").await?;
    Ok(())
}


fn nmap(flags: Vec<&str>, address: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("nmap")
        .args(&flags)
        .args(&[address])
        .output()?;

    if !output.status.success() {
        return Err(format!("nmap failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let result = String::from_utf8(output.stdout)?;
    Ok(result)
}

async fn fetch_headers(address: &str) -> Result<(), ReqwestError> {
    let client = reqwest::Client::new();

    let res = client.get(address)
        .send()
        .await?;

    eprintln!("Headers: {:#?}\n", res.headers());
    Ok(())
}