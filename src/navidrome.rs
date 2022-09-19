use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NavidromeLogin {
    pub id: String,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    pub message: String,
    pub name: String,
    pub token: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NavidromeRegisterPayload {
    #[serde(rename = "userName")]
    pub username: String,
    pub name: String,
    pub password: String,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
}
#[derive(Clone)]
pub struct Navidrome {
    pub url: String,
    pub client: reqwest::Client,
    pub login: NavidromeLogin,
}

impl Navidrome {
    pub async fn new(
        url: String,
        username: String,
        password: String,
    ) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::new();
        let data: HashMap<&str, &str> = std::array::IntoIter::new([
            ("username", &username as &str),
            ("password", &password as &str),
        ])
        .collect();
        let login: NavidromeLogin = client
            .post(format!("{}/app/login", url))
            .json(&data)
            .send()
            .await?
            .json::<NavidromeLogin>()
            .await?;
        if !login.is_admin {
            panic!(
                "[Navidrome] '{}' isn't an admin account. Please use a different account.",
                login.username
            )
        }
        Ok(Self { url, client, login })
    }

    pub async fn create_account(self, username: String, password: String) {
        let data: NavidromeRegisterPayload = NavidromeRegisterPayload {
            username: username.clone(),
            password,
            name: username,
            is_admin: false,
        };

        self.client
            .post(format!("{}/app/api/user", self.url))
            .json(&data)
            .header("x-nd-authorization", format!("Bearer {}", self.login.token))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    }
}
