use notify_rust::{Notification, Timeout};
use reqwest::blocking::get;
use std::env;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let links: Vec<String> = if args.is_empty() {
        // читаем из stdin
        io::stdin().lock().lines().filter_map(Result::ok).collect()
    } else {
        args
    };

    let mut summary = String::new();

    for link in links {
        match get(&link) {
            Ok(res) => summary += &format!("{}: {};\n", link, res.status()),
            Err(err) => summary += &format!("{}: ERROR ({});\n", link, err),
        }
    }

    Notification::new()
        .summary("Responces")
        .body(&summary)
        .icon("/usr/share/icons/breeze-dark/actions/16/globe.svg")
        .timeout(Timeout::Milliseconds(6000)) //milliseconds
        .show()
        .unwrap();
}
