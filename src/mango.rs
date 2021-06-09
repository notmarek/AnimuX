use std::collections::HashMap;

use cookie::{Cookie, CookieJar};
use http::HeaderValue;

#[derive(Clone)]
pub struct Mango {
    url: String,
    jar: CookieJar,
    client: reqwest::Client,
}

impl Mango {
    pub async fn new(
        url: String,
        username: String,
        password: String,
    ) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let form: HashMap<&str, &str> = std::array::IntoIter::new([
            ("username", &username as &str),
            ("password", &password as &str),
        ])
        .collect();
        let resp = client
            .post(&format!("{}/login", url))
            .form(&form)
            .send()
            .await?;
        let header_value = resp.headers().get("set-cookie").unwrap();
        let mut jar = CookieJar::new();
        jar.add(Cookie::parse(String::from(header_value.to_str().unwrap())).unwrap());
        Ok(Self {
            url: url,
            jar: jar,
            client: client,
        })
    }

    pub async fn create_account(self, username: String, password: String) {
        let form: HashMap<&str, &str> = std::array::IntoIter::new([
            ("username", &username as &str),
            ("password", &password as &str),
        ])
        .collect();
        let r = self
            .client
            .post(&format!("{}/admin/user/edit", self.url))
            .form(&form)
            .header(
                "cookie",
                format!(
                    "mango-sessid-9000={}",
                    self.jar.get("mango-sessid-9000").unwrap().value()
                ),
            )
            .send()
            .await
            .unwrap();
        ()
    }
}
