/**
Simple HTTP directory scanner for the command line.

Inputs:
- URL of the website to scan
- Path to a wordlist/dictionary

Output:
- All discovered paths returning HTTP status 200

Future
- delay (request -> delay -> request -> delay -> ...)
- port (change port)
- threads (run with multiple threads)
*/
use std::fs::read_to_string;

fn main() {
    let _url = "youtube.com";
    let _port = 80;
    let dictionary_path = "./test.txt";

    // read all paths from file
    let mut dictionary: Vec<&str> = Vec::new();
    let binding = read_to_string(dictionary_path).unwrap();
    for line in binding.lines() {
        dictionary.push(line);
    }

}
