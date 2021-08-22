use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server: Option<Server>,
    pub feeds: Option<Vec<Feed>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feed {
    pub url: Option<String>,
    pub matchers: Option<Vec<Matcher>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Matcher {
    pub regexp: Option<String>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub struct RegexMatcher {
    pub matcher: Regex,
    pub path: Option<String>,
}
