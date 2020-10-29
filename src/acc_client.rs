use crate::config::state::Cookie;
use crate::util;
use easy_scraper::Pattern;
use reqwest::{header, Client, Response};
use std::process;

pub const USER_AGENT: &str = "acc/1.1.1";
pub const LOGIN_URL: &str = "https://atcoder.jp/login?continue=https%3A%2F%2Fatcoder.jp%2Fcontests%2Fpractice%2Fsubmissions%2Fme";
pub const PRACTICE_URL: &str = "https://atcoder.jp/contests/practice/submissions/me";
pub const TASK_URL: &str = "https://atcoder.jp/contests/<CONTEST>/tasks/<CONTEST_TASK>_<TASK>";
pub const SUBMIT_URL: &str = "https://atcoder.jp/contests/<CONTEST>/submit";
pub const SUBMISSIONS_URL: &str = "https://atcoder.jp/contests/<CONTEST>/submissions/me";
pub const TESTCASE_PATTERN: &str =
    r#"<span class="lang-ja"><div class="part"><section>{{io:*}}</section></div></span>"#;
pub const CSRF_TOKEN_PATTERN: &str = r#"<input type="hidden" name="csrf_token" value={{token}} />"#;

fn extract_cookies(response: &Response) -> Vec<Cookie> {
    response
        .headers()
        .get_all("set-cookie")
        .iter()
        .map(|c| Cookie {
            value: c.to_str().unwrap().to_string(),
        })
        .collect()
}

pub struct AccClient {
    client: Client,
    csrf_token: String,
}

impl AccClient {
    pub fn new(needs_load: bool) -> AccClient {
        let mut headers = header::HeaderMap::new();
        let mut csrf_token = String::new();
        /*クッキーとcsrfトークン情報が必要なら読み込んでヘッダーに登録*/
        if needs_load {
            let (token, cookies) = util::load_state();
            csrf_token = token;
            for cookie in cookies {
                headers.insert(
                    header::COOKIE,
                    header::HeaderValue::from_str(&cookie.value).unwrap(),
                );
            }
        }
        let client = Client::builder()
            .cookie_store(true)
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()
            .unwrap();
        AccClient {
            client: client,
            csrf_token: csrf_token,
        }
    }

    pub fn get_page(&self, url: &str) -> Option<(String, Vec<Cookie>)> {
        let task = async { self.client.get(url).send().await };
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let response = rt.block_on(task);
        if let Ok(response) = response {
            let cookies = extract_cookies(&response);
            let task = async { response.text().await };
            let content = rt.block_on(task);
            match content {
                Ok(content) => Some((content, cookies)),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn post_form_data(
        &self,
        url: &str,
        form_data: Vec<(&str, String)>,
    ) -> Option<(String, String, Vec<Cookie>)> {
        let mut form = reqwest::multipart::Form::new();
        for data in form_data {
            form = form.text(data.0.to_string(), data.1);
        }
        let task = async { self.client.post(url).multipart(form).send().await };
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let response = rt.block_on(task);
        if let Ok(response) = response {
            let cookies = extract_cookies(&response);
            let url = response.url().to_string();
            let task = async { response.text().await };
            let content = rt.block_on(task);
            match content {
                Ok(content) => Some((url, content, cookies)),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn login_atcoder(
        &self,
        username: &str,
        password: &str,
    ) -> Option<(String, String, Vec<Cookie>)> {
        let url = LOGIN_URL;
        let result = self.get_page(url).unwrap_or_else(|| {
            util::print_error(format!("Can not get page({})", url));
            process::exit(1);
        });
        let (content, _) = result;
        let pattern = Pattern::new(CSRF_TOKEN_PATTERN).unwrap();
        let token = pattern
            .matches(&content)
            .iter()
            .map(|x| x["token"].to_string())
            .next()
            .unwrap_or_else(|| {
                util::print_error("CSRF_TOKEN is not found");
                process::exit(1);
            });
        let token = token.to_string();
        let username = util::remove_last_indent(username);
        let form_data = vec![
            ("csrf_token", token.clone()),
            ("username", username),
            ("password", password.to_string()),
        ];
        let result = self.post_form_data(url, form_data);
        match result {
            // url, csrf_token, cookies
            Some(result) => Some((result.0, token, result.2)),
            None => None,
        }
    }

    pub fn get_csrf_token(&self) -> Option<String> {
        if self.csrf_token.is_empty() {
            None
        } else {
            Some(self.csrf_token.clone())
        }
    }
}
