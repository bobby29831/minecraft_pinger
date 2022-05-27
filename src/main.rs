use std::collections::HashMap;
use reqwest;
use std::env;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Server {
    online: bool,
    ip: String,
    port: i32,
    version: Option<String>,
    software: Option<String>,
    players: Option<Players>
}

#[derive(Serialize, Deserialize, Debug)]
struct Players {
    online: i64,
    max: i64,
    list: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
struct PlayerIds {
    map: HashMap<String, String>
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];
    let base_url = "https://api.mcsrvstat.us/2/".to_owned() + ip;

    if args[1].trim().is_empty() {
        println!("Please actually include an IP argument!");
        return;
    }

    println!("Attempting to find details on '{}'...", ip);

    let client = reqwest::Client::new();
    let result = client.get(base_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match result.status() {
        reqwest::StatusCode::OK => {
            match result.json::<Server>().await {
                Ok(srv) => {
                    let status = srv.online;
                    println!("Server status: {}", if srv.online { "Online" } else { "Offline" });
                    if status {
                        println!("Server version: {} ({})",
                                 srv.version.unwrap_or(String::from("Version Unknown")),
                                 srv.software.unwrap_or(String::from("Software Unknown"))
                        );
                        let players = srv.players.unwrap();
                        let list = players.list.unwrap_or(vec![]);
                        let names: String = if list.is_empty() { "".to_owned() } else { format!("[{}]", list.join(", ")).to_owned() };
                        println!("Players: ({}/{}) {}", players.online, players.max, names);
                    }
                }
                Err(err) => println!("Ah poop! The response didn't work with the struct!\n{}", err)
            }
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            println!("You're making too many requests! Wait a few minutes and try again!")
        }
        other => {
            panic!("Awh poop! For some reason I couldn't get the server's details!\n{}", other);
        }
    }
}
