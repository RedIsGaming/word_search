use std::{env, error};

use colored::Colorize;
use word_game::{field::Field, reset::Reset};

#[derive(Debug)]
struct Arg;

impl Arg {
    fn option(args: &Args) {
        let arg = args.other.as_str();

        match arg {
            "-g" | "--game" => Field::generate(),
            "-p" | "--prank" => webbrowser::open("https://www.youtube.com/watch?v=cvh0nX08nRw").ok().unwrap(),
            "-h" | "--help" => {
                Reset::clear();
                Args::print(args);
            },
            "-v" | "--version" => println!("version = {}", env!("CARGO_PKG_VERSION")),
            _ => {
                Reset::clear();
                eprintln!("{}", "The command you've entered doesn't exist. Do you need help?\nTry cargo run -- -h instead.".bold());
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Args {
    first: Option<String>,
    other: String,
}

impl Args {
    fn new(args: &[String]) -> Result<Self, String> {
        if args.len().eq(&1) || args.len().ge(&3) {
            return Err(String::from("No arguments or more then 1 where passed!"));
        }

        Ok(Self {
            first: args.first().cloned(),
            other: args[1].to_owned(),
        })
    }

    fn print(&self) {
        println!("{} {} [OPTIONS] {} <GAME> {} <PRANK>\n\n{}{} <GAME>    Play Reddy word_search\n{} <PRANK>  Open video prank\n{}           Print help\n{}        Print version\n",
            "Reddy word_search Usage:".bold().underline(), self.first.as_ref().unwrap().replace(r"target\debug\", "").bold(), 
            "--game".bold(), 
            "--prank".bold(),
    
            "Options:\n".bold().underline(),
            "-g, --game".bold(),    
            "-p, --prank".bold(),
            "-h, --help".bold(),
            "-v, --version".bold(),
        );
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    Reset::clear();
    
    let args: Vec<_> = env::args().collect();
    let env_args = Args::new(&args);
    
    Args::print(env_args.as_ref().expect("Couldn't parse or find arguments"));
    Arg::option(&env_args.unwrap());

    Ok(())
}
