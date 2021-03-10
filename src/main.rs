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

use serde_json::Value as JsonValue;

use std::{intrinsics::type_name, io::Read};
// use std::any::type_name;

// use reqwest;
use reqwest::{Error, Result, blocking::Response};

fn type_of(_: T) -> &'static str {
    type_name::()
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
    res = coin_market_chart()?;
    res.read_to_string(&mut body).unwrap();
    println!("Status: {}", res.status());
    println!("Headers:\n {:#?}", res.headers());
    println!("Body: {}", body);
    println!("{}", type_of(body));

    let json_res = serde_json::from_str(body);
    if json_res.is_ok() {
        let p: JsonValue = json_res.unwrap();
        println!("============= Prices =============\n {}", p["prices"].as_str().unwrap());
    } else {
        println!("Didn't work!");
    }

    Ok(())
}


