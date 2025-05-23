use std::process::Command;
use std::str;
use std::error::Error;
// use std::collections::HashMap;
use reqwest::Error as ReqwestError;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let address = "http://scanme.nmap.org".to_string();

    let _headers = fetch_headers(&address).await?;

    let mut flags = Vec::new();
    flags.push("-sV".to_string());


    let nmap = nmap(flags, "scanme.nmap.org".to_string());
    eprintln!("\n{:?}", nmap);

    Ok(())
}


fn nmap(flags: Vec<String>, address: String) -> Result<String, Box<dyn Error>> {
    let output = Command::new("nmap")
        .args(&flags)
        .args(&[address])
        .output()
        .expect("failed to execute nmap command");

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

    let headers = res.headers();
    for (key, value) in headers.iter() {
        eprintln!("{:?}: {:?}", key, value);
    }

    Ok(())
}