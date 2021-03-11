// use std::panic::resume_unwind;

// Using rust-cookbook recipes

// $ cargo install cargo-edit
// $ cargo add 

// use error_chain::error_chain;
// use std::io::{self, stdin, Read};
// use reqwest;


// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

// // 1. Create and load Environment variables file

// // 2. Import Body Parser

// // 3. Import HTTP server

// // 4. Import Moment-timezone

// // 5. Import Promise based HTTP client

// #[tokio::main]
// async fn main() -> Result<()> {
//     let res = reqwest::get("http://httpbin.org/get").await?;
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());

//     let body = res.text().await?;
//     println!("Body:\n{}", body);
//     Ok(())
// }
// extern crate reqwest;
// use error_chain::error_chain;
// use std::io::Read;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

// fn main() -> Result<()> {
//     let mut res = reqwest::get("http://httpbin.org/get").await?;
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;

//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());
//     println!("Body:\n{}", body);

//     Ok(())
// }

extern crate reqwest;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use serde_json::Value as JsonValue;
use serde_json::json;

use std::{any::type_name, io::Read};
// use std::any::type_name;

// use reqwest;
use reqwest::{Error, Result, blocking::Response};

#[derive(Serialize, Deserialize)]
struct CoinHistory {
    prices: Vec<f32>,
    market_caps: Vec<f32>,
    total_volumes: Vec<f32>

}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

// fn simple_price(url: &str) -> Result<reqwest::blocking::Response> {
fn simple_price() -> Result<reqwest::blocking::Response> {
    // let api_link = "https://api.coingecko.com/api/v3".to_owned();
    // let endpoint = "/simple/price?ids=iota&vs+currencies=usd";

    // let client = reqwest::blocking::Client::new();
    // let res = client.get(api_link + endpoint).send();

    // let price = res.unwrap().json().unwrap().text();
    // price
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=iota&vs_currencies=usd";

    let res = reqwest::blocking::get(url)?;

    Ok(res)
}

fn coin_market_chart() -> Result<reqwest::blocking::Response> {
    let url = "https://api.coingecko.com/api/v3/coins/iota/market_chart?vs_currency=usd&days=30&interval=daily";

    let res = reqwest::blocking::get(url)?;

    Ok(res)
}


fn main() -> Result<()> {
    // https://www.coingecko.com/en/api#explore-api
    let api_link = "https://api.coingecko.com/api/v3".to_owned();
    let req_link = "/ping";

    // match reqwest::blocking::get(api_link + req_link) {
    //     Ok(mut response) => {
    //         // Check if 200 Ok
    //         if response.status() == reqwest::StatusCode::OK.is_success(){
    //             match response.text(){
    //                 Ok(text) => println!("Response Text: {}", text),
    //                 Err(_) => println!("Could not read response text!")
    //             }
    //         } else {
    //             println!("Response was not 200 OK.");
    //         }
    //     }
    //     Err(_) => println!("Could not make the request!")
    // }

    let response_text = reqwest::blocking::get(api_link + req_link)   
    .expect("Couldn't make request")
        .text().expect("Could not read response text!");

    println!("Ping Response Text: {}", response_text);
    println!("---------------------------------------");

    // let url = "https://api.coingecko.com/api/v3/simple/price?ids=iota&vs_currencies=usd";
    // let mut res = simple_price(url)?;
    println!("============= Simple Price =============");
    let mut res = simple_price()?;
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("Status: {}", res.status());
    println!("Headers:\n {:#?}", res.headers());
    println!("Body: {}", body);

    println!("============= Coin Market Chart =============");
    let mut res2 = coin_market_chart()?;
    let mut body2 = String::new();

    res2.read_to_string(&mut body2).unwrap();
    println!("Status: {}", &res2.status());
    println!("Headers:\n {:#?}", &res2.headers());
    println!("Body: {}", &body2);
    // println!("{}", type_of(&body2));

    let json_res = serde_json::from_str(&body2);
    if json_res.is_ok() {
        let p: JsonValue = json_res.unwrap();
        // println!("============= Prices =============\n {}", p["prices"].as_str().unwrap());
        println!("============= Prices =============\n {:#?}", p["prices"]);
        println!("type of p {}", type_of(&p));
    } else {
        println!("Didn't work!");
    }

    let response_text = coin_market_chart()
        .expect("Couln't make request")
        .text().expect("Could not read response text!");
    
    println!("type of response_text {:#?}", type_of(&response_text));
    // println!("response_text {:#?}", &response_text);
    
    // let coinstuff = json!(response_text);

    // if response_text.is_empty(){
    //     println!("Sorry, Serde Derive didn't work.");
    // } else {
    //     let p: CoinHistory = coinstuff;
    //     println!("============= Prices (derives) =============\n {:#?}", p.prices);
    // }

    let res3: serde_json::Value = reqwest::blocking::get("https://api.coingecko.com/api/v3/coins/iota/market_chart?vs_currency=usd&days=3&interval=daily")
        .expect("Couldn't make request")
        .json().expect("Couldn't parse JSON");

    println!("\ntype of res3 {}", type_of(&res3["prices"]));
    println!("============= res3 Prices =============\n {}", &res3["prices"][0]);

    for price in &res3["prices"].as_array(){
        println!("{:#?}", price);
    }



    Ok(())
}


