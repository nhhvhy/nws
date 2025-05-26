use std::process::Command;
use std::str;
use std::env;
use std::error::Error;
// use std::collections::HashMap;
use reqwest::Error as ReqwestError;

struct Params {
    url: String,
    fqdn: String,
    flags: Vec<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let params: Params = collect_params();

    let _headers = fetch_headers(&params.url).await?;
    
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
            flags: vec![]
        };
    }

    let url = String::from(&args[1]);
    let fqdn = String::from(&args[2]);
    let flags = Vec::from(&args[3..]);
    // dbg!(&url, &fqdn, &flags);

    return Params {
        url: url,
        fqdn: fqdn,
        flags: flags
    }

}

// nmap doesn't like protocols, use fqdn instead of url
fn nmap(address: String, flags: Vec<String>) -> Result<String, Box<dyn Error>> {
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