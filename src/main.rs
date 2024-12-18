use std::io::Error;
use sporky_checker::*;
use clap::{error::ErrorKind, Parser};
use cli::Cli;
use colored::Colorize;

pub mod cli;

fn main() { 

    //initialize app
    std::process::exit(match app() {
        Ok(_) => 0,
        Err(error) => {
            eprintln!("Error: {:?}", error);
            1
        }
    });
}

fn app() -> Result<(), Error>{
    let list = read_word_list("en.txt");
    let mut matches = Vec::new();
    let args = Cli::parse();

    let mut target = String::new();

    if args.target == None {
        let mut cmd = clap::Command::new("spork-checker [TARGET] [OPTIONS]");

        let error = cmd.error(ErrorKind::MissingRequiredArgument, 
            format!("{} {} {}", "Argument".red().bold(), target, "was not provided".red().bold())
        );

        clap::Error::exit(&error);
    }
    else {
        target = args.target.unwrap();
    }

    for word in list.iter() {
        let distance = levenshtein_distance(&target, word);
        if distance == 0 {
            println!("{}", "Your spelling checks out!".green().bold());
            std::process::exit(0);
        }
        else if distance <= 4 {
            matches.push((word.clone(), distance)); 
        }
    }

    matches.sort_by(|a, b| a.1.cmp(&b.1));
    let top_words = &matches[..matches.len().min(args.number)];

    if !args.verbose {
        println!("{} ", "Possible Matches:".purple().bold() );
        for (word, _distance) in top_words {
            println!(" {} '{}'", "-".blue().bold(), word);
        }

        std::process::exit(0);
    }

    if args.verbose {
        println!("{} ", "Possible Matches:".purple().bold());
        
        for (word, distance) in top_words {
            println!("{} '{}' {} '{}'","Word:".purple(), word, "with a distance of:".purple(), distance);
        }

        std::process::exit(0);
    }

    Ok(())
}



