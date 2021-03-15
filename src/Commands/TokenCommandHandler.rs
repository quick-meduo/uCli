use crate::*;
use reqwest;
use std::collections::HashMap;
use serde_json::Value;
use serde_json::json;

#[derive(Default)]
pub struct Command;

impl Command {
    pub fn handle(&self, cmd: TokenCommand) {
        match &cmd.cmd[..] {
            "create" => {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build().unwrap();
                runtime.block_on(
                    Command::create_token(cmd.name,cmd.email,cmd.domain)
                );
            },
            _ => {
                println!("invalid command")
            }
        }
    }

    async fn create_token(name: String, email: String, domain: String){
        let token = format!("{}","eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTU1MTQyNjEsImV4cCI6MTYxNjExOTA2MSwidXNlciI6ImFub255bW91cyIsInJvbGUiOiIiLCJkb21haW4iOiJwdWJsaWMiLCJsb2dpbl9zZXNzaW9uIjoiMTAwMTAifQ.unTxKTi3BUGzjOxaRDpx5a0af--YRfaaSpfFsCXnejI");

        let body = json!( {
           "name": name,
           "email": email,
           "domain": domain
        });


        let reqwest = reqwest::Client::new();
        let result =  reqwest.post("http://127.0.0.1:8086/api/token/create")
            .bearer_auth(token)
            .json(&body)
            .send()
            .await;


        match result {
            Ok(response) => {
                let body = response.json::<HashMap<String,Value>>().await;
                match body {
                    Ok(body) => {
                        println!("please save token usage later");
                        println!("------------------------------user token generated---------------------------------");
                        println!("{}",body.get("data").unwrap().get("token").unwrap());
                        println!("-----------------------------------------------------------------------------------");
                    },
                    Err(err) => {
                        println!("Failed to get token from server side, error: {}",err);
                    }
                }

            },
            _ => { }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use tokio;
    use reqwest::Response;
    use std::collections::HashMap;
    use serde_json::value::Value;

    #[tokio::test]
    async fn test_request(){
        let cmd = TokenCommand{
            cmd: "create".to_string(),
            name: "admin".to_string(),
            email: "b@bb.net".to_string(),
            domain: "public".to_string()
        };

        let command = self::Command::default();
        command.handle(cmd).await
    }
}
