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

        let client = BlockingClient::builder()
            .cookie_store(true)
            .default_headers(headers)
            .build()
            .unwrap();

        let session = persistence::Session::load();

        Self { client, session }
    }

    fn preflight(&mut self) -> Result<(), String> {
        let res = self
            .client
            .get("https://finance.yahoo.com")
            .header(header::ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"))
            .send()
            .map_err(|e| format!("Error getting session during preflight: {}", e))?;

        let cookies = res
            .headers()
            .get_all(header::SET_COOKIE)
            .iter()
            .map(|v| v.to_str().unwrap())
            .collect::<Vec<&str>>()
            .join(",");

        let res = self
            .client
            .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
            .send()
            .map_err(|e| format!("Error fetching crumb during preflight: {}", e))?;

        let crumb = res
            .text()
            .map_err(|e| format!("Error fetching crumb during preflight: {}", e))?;

        self.session.crumb = crumb;
        self.session.cookies = cookies;
        self.session.persist().expect("Error persisting session.");

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
            .send()
    }

    pub fn fetch_quotes(&mut self, symbols: &[String]) -> Result<entities::Response, String> {
        if self.session.is_empty() {
            self.preflight()?;
        }

        let mut res: reqwest::blocking::Response = self
            ._fetch_quotes(symbols)
            .map_err(|e| format!("Error fetching quotes: {}", e))?;

        if !res.status().is_success() {
            if res.status() != reqwest::StatusCode::UNAUTHORIZED {
                return Err(format!("Error fetching quotes: {}", res.status()));
            }

            // acquire new session and retry once if 401
            self.preflight()?;

            res = self
                ._fetch_quotes(symbols)
                .map_err(|e| format!("Error fetching quotes: {}", e))?;
        }

        res.json()
            .map_err(|e| format!("Error parsing response: {}", e))
    }
}
