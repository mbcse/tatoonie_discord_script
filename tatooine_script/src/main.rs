extern crate reqwest;
extern crate simplelog;
#[macro_use] extern crate log;
use std::collections::HashMap;
use std::env;
use simplelog::*;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;

fn send_discord_message(discord_url: &str, msg: &str){

    info!("{:?}", msg);

    let client = reqwest::blocking::Client::new();
    let mut body = HashMap::new();

    body.insert("username", "Tatooine_Script");
    body.insert("content", msg);
  
    let resp = client.post(discord_url)
    .json(&body)
    .send();

    match resp {
        Ok(res) => {
            if res.status() == 204 {
               info!("Message Sent!");
            }else {
                error!("Problem while Sending Message!");
            }
        },
        Err(..) => {
            error!("Problem while Sending Message!");
        }
    } 

}

fn send_serverstatus_update(faucet_url: &str, discord_url: &str){

    info!("Checking Server Status!");

    let response = reqwest::blocking::get(faucet_url);
    match response {
       Ok(res) => {
         if res.status() == reqwest::StatusCode::OK{
             info!("Faucet Server Is Working Fine : {:?}", res.text().unwrap()); 
         }else {
             send_discord_message(discord_url, "Faucet Server Down!");
         }  
       },
       Err(..) => {
           send_discord_message(discord_url, "Faucet Server Down!");
       }
    }

}


fn send_balance_update(faucet_url: &str, discord_url: &str, password: &str){

    info!("Checking Balance Status!");

    let response = reqwest::blocking::Client::new()
    .get(faucet_url.to_owned() + "/getbalance")
    .basic_auth("discord_script", Some(password)).send();

    match response {
       Ok(res) => {
         if res.status() == reqwest::StatusCode::OK{
             let balance: &str = &res.text().unwrap();
             send_discord_message(discord_url, balance);
         }else{
             send_discord_message(discord_url, "Can't get the balance, Faucet server may be down!");
         }  
       },
       Err(..) => {
           send_discord_message(discord_url, "Can't get the balance, Faucet server may be down!");
       }
    }

}

fn main() {
   //Logger
    let file;
    if !Path::new("./tatooine_discord.log").exists(){
        file = File::create("tatooine_discord.log").unwrap();
    }else{
        file = OpenOptions::new().append(true).open("./tatooine_discord.log").unwrap();   
    }
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(),file),
        ]
    ).unwrap(); 

   //Configurations
   let mut settings = config::Config::default();
   settings.merge(config::File::with_name("config/settings")).unwrap();
   let conf = settings.clone().try_into::<HashMap<String, String>>().unwrap(); 

   let faucet_url: &str = conf.get("FAUCET_URL").unwrap();
   let discord_webhook_url: &str = conf.get("DISCORD_WEBHOOK_URL").unwrap();
   let password: &str = conf.get("PASSWORD").unwrap();

   let args: Vec<String> = env::args().collect();
   if args.len() == 1{
        println!("Problem parsing arguments: Not enough arguments");   
   }else{
        match args[1].as_str() {
            "server" => send_serverstatus_update(faucet_url, discord_webhook_url),
            "balance" => send_balance_update(faucet_url,discord_webhook_url, password),
            _ => println!("Invalid argument: Use one of the following\nserver: For server status\nbalance: For balance status")
        }
   }

   

}
