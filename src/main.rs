// plan:
//
// 1. create a cli utility
// 2. make a request / parse output
// 3. nice output (maybe?)
//

use clap::Parser;
use dotenv;
use reqwest;

const LAT: f32 = -41.2;
const LON: f32 = 174.7;
const ZIP: u128 = 98034;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    /// Number of days for forecast
    #[arg(short, default_value_t = 0)]
    days: u8, // up to 255 days (api only gives 5 days)
}

fn main() {
    dotenv::dotenv().unwrap(); // !!

    let mut api_key: Option<String> = None;
    for (key, value) in std::env::vars() {
        if key != "API_KEY" {
            continue;
        }
        api_key = Some(value);
    }

    if api_key.is_none() {
        println!("API_KEY not found in .env");
    }

    // override option with string itself (unwrap)
    let api_key: String = api_key.unwrap();

    let args: Args = Args::parse();

    let url = format!(
        "https://api.weatherapi.com/v1/forecast.json?q={}&days=1&key={}",
        ZIP, api_key,
    );

    println!("api_key: {}", api_key);
    println!("url: {}", url);

    // fetch from reqwest documentation
    let body = reqwest::blocking::get("https://www.rust-lang.org")
        .unwrap()
        .text()
        .unwrap();

    // println!("body = {:?}", body);
}
