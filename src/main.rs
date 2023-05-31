mod client;
mod color;
mod entities;
mod persistence;

use client::Client;
use color::get_colors;
use entities::{get_ticker_data, MarketState};
use std::{env, process};

fn main() {
    let symbols: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    if symbols.is_empty() {
        println!("Usage: ticker AAPL MSFT GOOG BTC-USD");
        process::exit(1);
    }

    let mut client = Client::new();

    let res = match client.fetch_quotes(&symbols) {
        Ok(r) => r,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    let colors = get_colors();

    for symbol in symbols {
        let result = res
            .quote_response
            .result
            .iter()
            .find(|r| r.symbol == symbol);

        if result.is_none() {
            println!("No results for symbol \"{}\"", symbol);
            continue;
        }

        let (symbol, market_state, price, diff, percent) = get_ticker_data(result.unwrap());

        let market_sign = match market_state {
            MarketState::Regular => "",
            _ => "*",
        };

        let price_color = match diff {
            x if x > 0.0 => &colors.green,
            x if x < 0.0 => &colors.red,
            _ => &colors.none,
        };

        println!(
            "{:<10}{color_bold}{:8.2}{color_reset}{price_color}{:10.2}{:>12}{color_reset} {}",
            symbol,
            price,
            diff,
            format!("({:.2}%)", percent),
            market_sign,
            price_color = price_color,
            color_bold = colors.bold,
            color_reset = colors.reset
        );
    }
}
