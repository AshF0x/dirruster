use chrono;
use clap::App;
use http::Request;
use isahc::prelude::*;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use ansi_term::Colour::Fixed;
use indicatif::ProgressBar;
use console::Term;
use futures::StreamExt;
use futures::future::ok;

#[tokio::main]
async fn main() {
    let args = App::new("dirruster")
        .version("0.1")
        .about("Project about learning how to write a directory bruteforcer in Rust")
        .author("written by AshF0x")
        .args_from_usage(
            "
       -u, --url=[TARGET_URL] 'Sets your target URL(required)'
       -w, --wordlist=[PATH_TO_WORDLIST] 'Sets your wordlist file(required)'
       -a, --uagent=[USER_AGENT] 'Sets your desired User Agent in the request header (optional)'
       -e, --extension=[EXTENSION] 'Sets your desired URI extension (limited to 1 atm)'",
        )
        .get_matches();
    
    let term = Term::stdout();
    //println!(IulianSiPunct);
    let mut user_agent = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.138 Safari/537.36";
    //let mut user_agent = "Mozilla/5.0 (iPhone; CPU iPhone OS 10_3_1 like Mac OS X) AppleWebKit/603.1.30 (KHTML, like Gecko) Version/10.0 Mobile/14E304 Safari/602.1";
    let target_host = args.value_of("url").unwrap();
    let wordlist = args.value_of("wordlist").unwrap();
    let mut ext = "";
    match args.occurrences_of("uagent") {
        0 => user_agent = user_agent,
        1 => user_agent = args.value_of("uagent").unwrap(),
        _ => println!("Something went wrong with the User-Agent.\nPlease try again."),
    }

    match args.occurrences_of("extension") {
        0 => ext = ext,
        1 => ext = args.value_of("extension").unwrap(),
        _ => println!("Something went wrong witht the extension.\nPlease try again."),
    }


    let out_path = Path::new("output.txt");
    let mut out_file = match File::create(&out_path) {
        Err(err) => panic!("Could not create output file | Reason: {}",err),
        Ok(file) => file,
    }; 

    //Testing if we can read the host & user agent
    println!("------------------------------");
    println!("The url: {}", Fixed(172).paint(target_host));
    println!("The User-Agent: {}", Fixed(177).paint(user_agent));
    println!("------------------------------");

    // Opening the file
    //let mut urls: Vec<String> = Vec::new();
    let file = File::open(wordlist).expect("Failed to open file");
    let file = BufReader::new(file);
    let bar = ProgressBar::new(1000);
    prober(file, &target_host,&user_agent, &ext, &mut out_file).await;
    // tokio::fs::File
}

async fn prober(file: BufReader<File>,target_host: &str,user_agent: &str,ext: &str,out_file: &mut File){
    let requests = file.lines() //Result<String, Error>
        .filter_map(|line_result| line_result.ok())
        .map(|line|format!("{}/{}{}", &target_host,line,ext)) //String
        .filter_map(|fetch| Request::head(fetch).header("User-Agent", user_agent).body(()).ok()); // Request
        
        futures::stream::iter(requests) // Stream of Requests
        .map(|request|request.send_async())
        .buffer_unordered(256) //Response
        .filter_map(|response| async { response.ok()})
        .for_each(|response| {
            let uri = response.effective_uri()
            .map(|uri|uri.to_string())
            .unwrap_or("".to_string());

            println!("[{}] {:?}",response.status().as_u16(),uri);
            // match out_file.write_all(uri.as_bytes()) {
            //     Err(err) => panic!("Error while writing to file: {}", err),
            //     Ok(_) => (),
            // }
            futures::future::ready(())
        }).await;
}

fn request(t: &str, ua: &str,term: &Term, mut out_file: &File) -> Result<(), isahc::Error> {
    // Send a HEAD request and wait for the response.
    // We use the HEAD method because we don't want to handle a response body
    let response = Request::head(t).header("User-Agent", ua).body(())?.send()?;
    // Handling the response by checking Status Code
    let status_response = response.status();
    // Handling the Response Code
    let success = Fixed(76).paint("Success");
    let redirect = Fixed(178).paint("Redirect");
    let client_error = Fixed(9).paint("Client Error");
    let server_error = Fixed(57).paint("Server Error");
    let informal = Fixed(21).paint("Informal");

    // Clears the current Term line
    let _ = term.clear_line();

    if status_response.is_success() {
        let succ_response = format!(
            "[{}] {} [{}] | {} \n",
            chrono::Local::now().format("%T"),
            success,
            status_response,
            t);
        println!("{}",succ_response);
        match out_file.write_all(succ_response.as_bytes()) {
            Err(err) => panic!("Error while writing to file: {}", err),
            Ok(_) => (),
        }
    } else if status_response.is_redirection() {
        let red_response = format!(
            "[{}] {} [{}] | {} \n",
            chrono::Local::now().format("%T"),
            redirect,
            status_response,
            t
        );
        println!("{}",red_response);
        match out_file.write_all(red_response.as_bytes()) {
            Err(err) => panic!("Error while writing to file: {}", err),
            Ok(_) => (),
        }
    } else if status_response.is_client_error() {
        let cerr_response = format!(
            "[{}] {} [{}] | {} \n",
            chrono::Local::now().format("%T"),
            client_error,
            status_response,
            t
        );
        println!("{}",cerr_response);
        match out_file.write_all(cerr_response.as_bytes()) {
            Err(err) => panic!("Error while writing to file: {}", err),
            Ok(_) => (),
        }
    } else if status_response.is_server_error() {
        let serr_response = format!(
            "[{}] {} [{}] | {} \n",
            chrono::Local::now().format("%T"),
            server_error,
            status_response,
            t
        );
        println!("{}",serr_response);
        match out_file.write_all(serr_response.as_bytes()) {
            Err(err) => panic!("Error while writing to file: {}", err),
            Ok(_) => (),
        }
    } else if status_response.is_informational() {
        let inf_response = format!(
            "[{}] {} [{}] | {} \n",
            chrono::Local::now().format("%T"),
            informal,
            status_response,
            t
        );
        println!("{}",inf_response);
        match out_file.write_all(inf_response.as_bytes()) {
            Err(err) => panic!("Error while writing to file: {}", err),
            Ok(_) => (),
        }
    }
    Ok(())
}
