use core::panic;

use crate::entities;
use crate::persistence;
use reqwest::{
    blocking::Client as BlockingClient,
    header::{self, HeaderMap, HeaderValue},
};

pub struct Client {
    client: BlockingClient,
    session: persistence::Session,
}

impl Client {
    const API_ENDPOINT: &str = "https://query1.finance.yahoo.com/v7/finance/quote?lang=en-US&region=US&corsDomain=finance.yahoo.com";
    const FIELDS: [&str; 11] = [
        "symbol",
        "marketState",
        "regularMarketPrice",
        "regularMarketChange",
        "regularMarketChangePercent",
        "preMarketPrice",
        "preMarketChange",
        "preMarketChangePercent",
        "postMarketPrice",
        "postMarketChange",
        "postMarketChangePercent",
    ];

    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));

        let client = match BlockingClient::builder()
            .cookie_store(true)
            .default_headers(headers)
            .build()
        {
            Ok(r) => r,
            Err(err) => panic!("Error building client: {}", err),
        };

        let session = persistence::Session::load();

        Self { client, session }
    }

    fn preflight(&mut self) -> Result<(), String> {
        let res = match self
            .client
            .get("https://finance.yahoo.com")
            .header(header::ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"))
            .send()
        {
            Ok(r) => r,
            Err(err) => return Err(format!("Error getting session during preflight: {}", err)),
        };

        let cookies = res
            .headers()
            .get_all(header::SET_COOKIE)
            .iter()
            .map(|v| v.to_str().unwrap())
            .collect::<Vec<&str>>()
            .join(",");

        let res = match self
            .client
            .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
            .send()
        {
            Ok(r) => r,
            Err(err) => return Err(format!("Error fetching crumb during preflight: {}", err)),
        };

        let crumb = match res.text() {
            Ok(r) => r,
            Err(err) => return Err(format!("Error fetching crumb during preflight: {}", err)),
        };

        self.session.crumb = crumb;
        self.session.cookies = cookies;
        self.session.persist().expect("Error persiting session.");

        Ok(())
    }

    fn _fetch_quotes(
        &mut self,
        symbols: &[String],
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.client
            .get(Client::API_ENDPOINT)
            .header(header::COOKIE, &self.session.cookies)
            .query(&[
                ("fields", Client::FIELDS.join(",")),
                ("symbols", symbols.join(",")),
                ("crumb", self.session.crumb.clone()),
            ])
            .header(reqwest::header::ACCEPT_ENCODING, "application/json")
            .send()
    }

    pub fn fetch_quotes(&mut self, symbols: &[String]) -> Result<entities::Response, String> {
        if self.session.is_empty() {
            match self.preflight() {
                Ok(_) => (),
                Err(err) => return Err(err),
            };
        }
        let mut res: reqwest::blocking::Response = match self._fetch_quotes(symbols) {
            Ok(r) => r,
            Err(err) => return Err(format!("Error fetching quotes: {}", err)),
        };

        if !res.status().is_success() {
            if res.status() != reqwest::StatusCode::UNAUTHORIZED {
                return Err(format!("Error fetching quotes: {}", res.status()));
            }

            // acquire new session and retry once if 401
            match self.preflight() {
                Ok(_) => (),
                Err(err) => return Err(err),
            };

            res = match self._fetch_quotes(symbols) {
                Ok(r) => r,
                Err(err) => return Err(format!("Error fetching quotes: {}", err)),
            };
        }

        match res.json() {
            Ok(r) => Ok(r),
            Err(err) => Err(format!("Error parsing response: {}", err)),
        }
    }
}
