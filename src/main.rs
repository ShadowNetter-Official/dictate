use reqwest;
use serde_json::Value;
use std::env;
use colored::*;

fn help() {
    println!("dictate | A pocket cli dictionary utility in Rust 🦀 ");
    println!("");
    println!("usage:");
    println!("");
    println!("dictate <word> <number of definitions>");
}

fn main() {
    // initial API and collection of user arguments
    let api = "https://api.dictionaryapi.dev/api/v2/entries/en/";
    let args: Vec<String> = env::args().collect();
    // check argument length (minimum 2 arguments)
    if args.len() < 2 {
        help();
    } else {
        // takes user word and makes new API link
        let word = args[1].clone();
        let link = format!("{}{}", api, word);
        
        // fetches and parses API data
        let res = reqwest::blocking::get(link)
            .expect("Request Failed")
            .text()
            .expect("Failed to read response");
        
        let data: Value = serde_json::from_str(&res).expect("Invalid JSON");
        let val = &data[0];
        // number of definitions to show
        let numstr = if args.len() < 3 {
            "1"
        } else {
            &args[2]
        };
        let num: usize = match numstr.parse() {
            Ok(n) => n,
            Err(_) => 1
        };
        let mut count = 0;
        // counter for definition numbering
        let mut i: i32 = 1;
        // colors for styling
        let num_yellow = Color::TrueColor { r:255, g:242, b:102 };
        let d_yellow = Color::TrueColor {r:250, g:246, b:202 };
        // gets user word
        let word = val["word"].as_str().unwrap();
        // gets list of meanings
        let meanings = &val["meanings"];
        // print intro (word, definitions)
        println!("{} {}\n", "word:".color(num_yellow), word.color(d_yellow));
        println!("{} \n", "definitions:".color(num_yellow));
        // gets meanings
        for meaning in meanings.as_array().unwrap() {
            let definitions = &meaning["definitions"];
            // for every definition in array display definition with colors and numbering
            for def in definitions.as_array().unwrap().iter() {
                if count >= num {
                    break;
                }
                let d = def["definition"].as_str().unwrap();
                println!("{}. {}\n", format!("{}", i).color(num_yellow), d.color(d_yellow));
                i += 1;
                count += 1;
            }
            if count >= num {
                break;
            }
        }

    }
}
