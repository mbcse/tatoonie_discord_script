extern crate reqwest;
use std::collections::HashMap;

static SERVER_URL:&'static str="http://127.0.0.1:8080/";
static DISCORD_WEBHOOK_URL:&'static str="https://discord.com/api/webhooks/871390641786073138/r1MugCEl_ONzzymk5ieZIaqBZz66UR6KpgqgzdnYO_jxD-tRcEZ_lYA911yqCriOHzz5";

fn send_discord_update(){
    let client = reqwest::blocking::Client::new();

    let mut body = HashMap::new();
    body.insert("username", "tatoonie_script");
    body.insert("content", "Faucet Server Is Down");

    let resp = client.post(DISCORD_WEBHOOK_URL)
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
  
   let response = reqwest::blocking::get(SERVER_URL);
   match response {
      Ok(res)=> {
        if res.status()==reqwest::StatusCode::OK{
            println!("Faucet Server Working Fine {:?}",res.text()); 
        }else{
            send_discord_update();
        }  
      },
      Err(..)=> {
          send_discord_update();
      }
   }

}
