use clap::App;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use isahc::prelude::*;
use http::Request;
use chrono;

fn main() {
    let args = App::new("dirruster")
       .version("0.1")
       .about("Project about learning how to write a directory bruteforcer in Rust")
       .author("written by AshF0x")
       .args_from_usage("
       -u, --url=[TARGET_URL] 'Sets your target URL(required)'
       -w, --wordlist=[PATH_TO_WORDLIST] 'Sets your wordlist file(required)'
       -a, --uagent=[USER_AGENT] 'Sets your desired User Agent in the request header (optional)'"
        )
       .get_matches();
    
    let mut user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.138 Safari/537.36";
    let target_host = args.value_of("url").unwrap();
    let wordlist = args.value_of("wordlist").unwrap();
    
    match args.occurrences_of("uagent") {
        0 => user_agent = user_agent,
        1 => user_agent = args.value_of("uagent").unwrap(),
        _ => println!(
            "Something went wrong witht the User-Agent.\nPlease try again."
        ),
    }

    //Testing if we can read the host & user agent
    println!("------------------------------");
    println!("The url: {}",target_host);
    println!("The User-Agent: {}",user_agent);
    println!("------------------------------");
    
    // Opening the file
    let mut urls: Vec<String> = Vec::new(); 
    let file = File::open(wordlist).expect("Failed to open file");
    let file = BufReader::new(file);
    for i in file.lines() {
        // Look this up noob
       if let Ok(s) = i{
            urls.push(s);
       }
    }

    // Making the request
    for path in urls {
        let user_agent = format!("User-Agent: {}",user_agent);
        let target = format!("{}/{}", &target_host, &path);

        //let response = isahc::head(&target).body(ua).send();

        if let Err(err) = request(&target,&user_agent) {
            println!("Error with {}: {}",&target,err);
        }
        // Look up the "Ok(())"
        //Ok(());
    }
}

fn request (t: &str,ua: &str ) -> Result<(), isahc::Error> {
    // Send a HEAD request and wait for the response.
    // We use the HEAD method because we don't need the body in this case
    // Must be `mut` so we can read the response body.

    let response = Request::head(t).body(ua)?.send()?;
    // Handling the response by checking Status Code
    let status_response = response.status();


    // Handling the Response Code
    if status_response.is_success(){
        println!("[{}] Success | {} | Status Code: {}",chrono::Local::now().format("%T"),t,status_response);
    } else if status_response.is_redirection() {
        println!("[{}] Redirect | {} | Status Code: {}",chrono::Local::now().format("%T"),t,status_response);
    }

    Ok(())
}
