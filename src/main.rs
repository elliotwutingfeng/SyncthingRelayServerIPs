use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::net::Ipv4Addr;
use reqwest;

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let url = "https://relays.syncthing.net/endpoint";

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap();
    
    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => {
                    let mut ipv4_list: Vec<Ipv4Addr> = Vec::new();
                    for relay in parsed.relays {
                        let ip_address: Ipv4Addr = relay.url
                        .split("relay://") // remove "relay://" prefix
                        .collect::<Vec<_>>()[1]
                        .split(':') // remove port number suffix
                        .collect::<Vec<_>>()[0]
                        .parse()
                        .expect("Unable to parse ipv4 address");

                        ipv4_list.push(ip_address);
                    }

                    ipv4_list.sort();

                    let ipv4_string: String = ipv4_list
                    .into_iter()
                    .map(|ip| ip.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");

                    if ipv4_string.chars().count() != 0 {
                    let path = Path::new("ips.txt");
                    let mut file = match File::create(&path) {
                        Err(why) => panic!("couldn't create file: {}", why),
                        Ok(file) => file,
                    };

                    match file.write_all(ipv4_string.as_bytes()) {
                        Err(why) => panic!("couldn't write to file: {}", why),
                        Ok(_) => println!("successfully wrote to file"),
                    }
                }
                },
                Err(_) => println!("Bad response shape"),
            };
        }
        other => {
            panic!("GET request failed: {:?}", other);
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
struct Relay {url: String}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {relays: Vec<Relay>}
