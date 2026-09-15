use std::env;
/**
Simple HTTP directory scanner for the command line.

Inputs:
- URL of the website to scan
- Path to a wordlist/dictionary

Output:
- All discovered paths returning HTTP status 200
*/
use std::fs::read_to_string;

static OK_RESPONSE: u16 = 200;

#[tokio::main]
async fn main()  {
    // get parameters from commandline
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    let dictionary_path = &args[2];

    // read all paths from file
    let mut dictionary: Vec<&str> = Vec::new();
    let binding = read_to_string(dictionary_path).unwrap();
    for line in binding.lines() {
        dictionary.push(line);
    }

    // send request(s)
    let mut found_paths: Vec<&str> = Vec::new();
    for path in dictionary{
        let target_url = format!("{}{}", url, path);
        let status = get_status(&target_url).await;
        match status {
            Ok(status) => {
                let code = status.as_u16();
                if code == OK_RESPONSE{
                    found_paths.push(path);
                }
            },
            Err(err) => println!("{err}")
        }
    }

    // print results
    println!("found paths for {url}:");
    for result in found_paths{
        println!("{result}")
    }

}

async fn get_status(url: &str) -> Result<reqwest::StatusCode, reqwest::Error>{
    let client = reqwest::Client::new();
    let res = client.post(url)
        .body("the exact body that is sent")
        .send()
        .await;

    match res {
        Ok(response) => Ok(response.status()),
        Err(err) => Err(err)
    }
}

