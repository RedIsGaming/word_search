use std::{env, error, process::{self, Command, ExitStatus}};
use colored::Colorize;

#[derive(Debug)]
struct Arg;

impl Arg {
    fn option(args: &Args) {
        let arg = args.other.as_str();

        match arg {
            "-g" | "--game" => Arg::print(arg),
            "-p" | "--prank" => Arg::prank(),
            "-e" | "--exit" => Arg::exit(),
            "-h" | "--help" => Arg::help(args),
            "-V" | "--Version" => Arg::version(),
            _ => Arg::unknown(args),
        }
    }

    fn print(arg: &str) {
        println!("Test{:?}", arg);
    }

    fn prank() {
        webbrowser::open("https://www.youtube.com/watch?v=cvh0nX08nRw").ok();
    }

    fn exit() {
        process::exit(0);
    }

    fn help(args: &Args) {
        Args::clear();
        Args::print(args);
    }

    fn version() {
        println!("version = {}", env!("CARGO_PKG_VERSION"));
    }

    fn unknown(args: &Args) {
        Arg::help(args);
        eprintln!("The command you've entered doesn't exist. Do you need help?");
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
            return Err(String::from("No arguments or more then 1 passed!"));
        }

        Ok(Self {
            first: args.first().cloned(),
            other: args[1].to_owned(),
        })
    }

    fn print(&self) {
        println!("{} {} [OPTIONS] {} <GAME> {} <PRANK>\n\n{}{} <GAME>\n{} <PRANK>  Open video prank\n{}           Exit this menu\n{}           Print help\n{}        Print version\n\n{}",
            "Reddy word_search Usage:".bold().underline(), self.first.as_ref().unwrap().replace(r"target\debug\", "").bold(), "--game".bold(), "--prank".bold(),
    
            "Options:\n".bold().underline(),
            "-g, --game".bold(),    
            "-p, --prank".bold(),
            "-e, --exit".bold(),
            "-h, --help".bold(),
            "-V, --version".bold(),
            self.other
        );
    }

    fn clear() -> Option<ExitStatus> {
        Command::new("cmd").args(["/c", "cls"]).status().ok()
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    Args::clear();
    
    let args: Vec<_> = env::args().collect();
    let env_args = Args::new(&args);
    Args::print(env_args.as_ref().expect("Couldn't parse/find arguments"));
    Arg::option(&env_args.unwrap());

    Ok(())
}
