extern crate reqwest;
use std::collections::HashMap;

fn send_discord_update(url:&str){
    let client = reqwest::blocking::Client::new();

    let mut body = HashMap::new();
    body.insert("username", "tatoonie_script");
    body.insert("content", "Faucet Server Is Down");

    let resp = client.post(url)
    .json(&body)
    .send();

    match resp {
        Ok(res) => {
            if res.status()==204 {
                println!("Server Down Message Sent!");
            }else {
                println!("Problem while Sending Message!");
            }
        },
        Err(..)=>{
            println!("Problem while Sending Message!");
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

   let response = reqwest::blocking::get(faucet_url);
   match response {
      Ok(res)=> {
        if res.status()==reqwest::StatusCode::OK{
            println!("Faucet Server Working Fine {:?}",res.text()); 
        }else{
            send_discord_update(discord_webhook_url);
        }  
      },
      Err(..)=> {
          send_discord_update(discord_webhook_url);
      }
   }

}
