use std::{env, error, process::{self, Command, ExitStatus}};
use colored::Colorize;

#[derive(Debug)]
struct Arg;

impl Arg {
    fn option(args: &Args) {
        let arg = args.other.as_str();

        match arg {
            "-g" | "--game" => Arg::print(arg),
            "-p" | "--prank" => Arg::print(arg),
            "-e" | "--exit" => Arg::quit(arg),
            "-h" | "--help" => Arg::print(arg),
            "-v" | "--version" => Arg::print(arg),
            _ => Arg::print(arg),
        }
    }

    fn quit(arg: &str) {
        println!("Bye!!{:?}", arg);
        process::exit(0)
    }

    fn print(arg: &str) {
        println!("Test{:?}", arg);
    }
}

#[derive(Debug)]
struct Args {
    first: Option<String>,
    other: String,
}

impl Args {
    fn new(args: &[String]) -> Result<Self, String> {
        if args.len().eq(&1) || args.len() >= 3 {
            return Err(String::from("No arguments or more then 2 passed!"));
        }

        Ok(Self {
            first: args.first().cloned(),
            other: args[1].to_owned(),
        })
    }

    fn print(&self) {
        println!("{} {} [OPTIONS] {} <GAME>\n\n{}{} <GAME>\n{} <PRANK>  A video prank\n{}           Exit this menu\n{}           Print help\n{}        Print version\n{}",
            "Reddy word_search Usage:".bold().underline(), self.first.as_ref().unwrap().replace(r"target\debug\", "").bold(), "--game".bold(),
    
            "Options:\n".bold().underline(),
            "-g, --game".bold(),    
            "-p, --prank".bold(),
            "-e, --exit".bold(),
            "-h, --help".bold(),
            "-V, --version".bold(),
            self.other
        );

        Arg::option(self)
    }

    fn clear() -> Option<ExitStatus> {
        Command::new("cmd").args(["/c", "cls"]).status().ok()
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    Args::clear();
    let args: Vec<_> = env::args().collect();
    let env_args = Args::new(&args);
    Args::print(&env_args.unwrap());

    Ok(())
}
