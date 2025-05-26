use reqwest::{Error as ReqwestError, header::HeaderMap};
use std::{env, error::Error, process::Command, str};

struct Params {
    url: String,
    fqdn: String,
    flags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let params: Params = collect_params();

    let headers = fetch_headers(&params.url).await?;
    eprintln!("\n{:?}", headers);

    let nmap = nmap(params.fqdn, params.flags);
    eprintln!("\n{:?}", nmap);

    Ok(())
}

fn collect_params() -> Params {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Params {
            url: "http://scanme.nmap.org".to_string(),
            fqdn: "scanme.nmap.org".to_string(),
            flags: vec![String::from("-sV")]                
        };
    }

    let url = String::from(&args[1]);
    let fqdn = String::from(&args[2]);
    let flags = Vec::from(&args[3..]);
    // dbg!(&url, &fqdn, &flags);

    Params { url, fqdn, flags }
}

// nmap doesn't like protocols, use fqdn instead of url
// -sV: service & version detection (port scan)
// -O: OS fingerprinting
// -p: use 1-65535 for full scan, as nmap defaults to top 1000 ports
fn nmap(address: String, flags: Vec<String>) -> Result<String, Box<dyn Error>> {
    let output = Command::new("nmap")
        .args(&flags)
        .arg(&address)
        .output()
        .expect("failed to execute nmap command");

    if !output.status.success() {
        return Err(format!("nmap failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let result = String::from_utf8(output.stdout)?;
    Ok(result)
}

async fn fetch_headers(address: &str) -> Result<HeaderMap, ReqwestError> {
    let client = reqwest::Client::new();
    let res = client.get(address).send().await?;

    Ok(res.headers().clone())
}
