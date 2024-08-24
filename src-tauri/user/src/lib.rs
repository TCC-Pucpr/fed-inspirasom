use std::{env, error::Error};

use dotenvy::dotenv;
use endpoints::ACCOUNTS;
use reqwest::{Client, Response};
use serde::Serialize;
mod endpoints;

const ENV_FIREBASE_KEY: &str = "FIREBASE_API_KEY";
const ENV_FIREBASE_URL_KEY: &str = "FIREBASE_URL";

type FirebaseResponse = Result<Response, Box<dyn Error>>;

pub struct FirebaseData {
    url: String,
    key: String,
    client: Client,
}

#[derive(Serialize)]
pub struct UserSign {
    email: String,
    pass: String,
}

impl FirebaseData {
    pub fn new() -> Self {
        let _ = dotenv();
        let key = if let Ok(s) = env::var(ENV_FIREBASE_KEY) {
            s
        } else {
            String::new()
        };
        let url = if let Ok(s) = env::var(ENV_FIREBASE_URL_KEY) {
            s
        } else {
            String::new()
        };
        let client = Client::new();
        Self { url, key, client }
    }

    pub async fn sign_up(&self, sign_up: UserSign) -> FirebaseResponse {
        let url = format!("{}{}:signUp?key={}", self.url, ACCOUNTS, self.key);
        let res = self.client.post(url).json(&sign_up).send().await?;
        Ok(res)
    }

    pub async fn sign_in(&self, sign_in: UserSign) -> FirebaseResponse {
        let url = format!(
            "{}{}:signInWithPassword?key={}",
            self.url, ACCOUNTS, self.key
        );
        let res = self.client.post(url).json(&sign_in).send().await?;
        Ok(res)
    }
}
