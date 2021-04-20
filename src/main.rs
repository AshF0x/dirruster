use clap::App;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use isahc::prelude::*;

fn main() {
    let args = App::new("dirruster")
       .version("0.1")
       .about("Project about learning how to write a directory bruteforcer in Rust")
       .author("written by AshF0x")
       .args_from_usage("
       -u, --url=[TARGET_URL] 'Sets your target URL(required)'
       -w, --wordlist=[PATH_TO_WORDLIST] 'Sets your wordlist file(required)'"
        )
       .get_matches();
    
    let target_host = args.value_of("url").unwrap();
    let wordlist = args.value_of("wordlist").unwrap();

    //Testing if we can read the host
    println!("The url: {}",target_host);
    
    // Opening the file
    let mut urls: Vec<String> = Vec::new(); 
    let file = File::open(wordlist).expect("Failed to open file");
    let file = BufReader::new(file);
    //for i in file.lines() {
    //    // Look this up noob
   //    if let Ok(s) = i{
    //        println!("{}",s);
   //    };
   // }
    request(&target_host).expect("Connection could not be established.");
    // Look up the "Ok(())"
    //Ok(());
}

fn request (t: &str) -> Result<(), isahc::Error> {
    // Send a GET request and wait for the response headers.
    // Must be `mut` so we can read the response body.
    let response = isahc::get(t)?;

    // TESTING!!!
    let status_response = response.status();
    if status_response.is_success(){
        println!("Request was successful with code {}!",status_response);
    } else {
        println!("Not 200 I guess? Status Code: {}",status_response);
    }

    Ok(())
}
