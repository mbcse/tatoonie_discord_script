extern crate reqwest;
use std::collections::HashMap;

fn send_discord_message(discord_url:&str,msg:&str){
    let client = reqwest::blocking::Client::new();

    let mut body = HashMap::new();
    body.insert("username", "Tatooine_Script");
    body.insert("content", msg);

    let resp = client.post(discord_url)
    .json(&body)
    .send();

    match resp {
        Ok(res) => {
            if res.status()==204 {
                println!("Message Sent!");
            }else {
                println!("Problem while Sending Message!");
            }
        },
        Err(..)=>{
            println!("Problem while Sending Message!");
        }
    } 

}

fn send_serverstatus_update(faucet_url:&str,discord_url:&str){
    let response = reqwest::blocking::get(faucet_url);
    match response {
       Ok(res)=> {
         if res.status()==reqwest::StatusCode::OK{
             println!("Faucet Server Is Working Fine : {:?}",res.text().unwrap()); 
         }else{
             send_discord_message(discord_url,"Faucet Server Down!");
         }  
       },
       Err(..)=> {
           send_discord_message(discord_url,"Faucet Server Down!");
       }
    }

}


fn send_balance_update(faucet_url:&str,discord_url:&str,password:&str){
    let response = reqwest::blocking::Client::new()
    .get(faucet_url.to_owned()+"/getbalance")
    .basic_auth("discord_script",Some(password)).send();

    match response {
       Ok(res)=> {
         if res.status()==reqwest::StatusCode::OK{
             let balance:&str=&res.text().unwrap();
             println!("{:?}",balance);
             send_discord_message(discord_url,balance);
         }else{
             send_discord_message(discord_url,"Can't get the balance, Faucet server may be down!");
         }  
       },
       Err(..)=> {
           send_discord_message(discord_url,"Can't get the balance, Faucet server may be down!");
       }
    }

}

fn main() {

   //Configurations
   let mut settings = config::Config::default();
   settings.merge(config::File::with_name("config/Settings")).unwrap();
   let conf=settings.clone().try_into::<HashMap<String, String>>().unwrap(); 

   let faucet_url: &str=conf.get("FAUCET_URL").unwrap();
   let discord_webhook_url: &str=conf.get("DISCORD_WEBHOOK_URL").unwrap();
   let password: &str=conf.get("PASSWORD").unwrap();

   send_serverstatus_update(faucet_url,discord_webhook_url);
   send_balance_update(faucet_url,discord_webhook_url,password);
  

}
