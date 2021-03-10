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

fn main() {
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

    println!("Response Text: {}", response_text);
}


