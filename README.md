# ticker-rs

> Real-time stock tickers from the command-line written in Rust.

CLI tool using the Yahoo Finance API as a data source. It features colored output and is able to display pre- and post-market prices (denoted with `*`).

Its goal is to behave exactly like [ticker.sh](https://github.com/pstadler/ticker.sh) and is meant as an excerise to learn Rust.

## Install

```sh
$ cargo install --git https://github.com/pstadler/ticker-rs.git
```

## Usage

```sh
# Single symbol:
$ ticker AAPL

# Multiple symbols:
$ ticker AAPL MSFT GOOG BTC-USD

# Read from file:
$ echo "AAPL MSFT GOOG BTC-USD" > ~/.ticker.conf
$ ticker $(cat ~/.ticker.conf)

# Use different colors:
$ COLOR_BOLD="\e[38;5;248m" \
  COLOR_GREEN="\e[38;5;154m" \
  COLOR_RED="\e[38;5;202m" \
  ticker AAPL

# Disable colors:
$ NO_COLOR=1 ticker AAPL

# Update every five seconds:
$ watch -n 5 -t -c ticker AAPL MSFT GOOG BTC-USD
# Or if `watch` is not available:
$ while true; do clear; ticker AAPL MSFT GOOG BTC-USD; sleep 5; done
```
