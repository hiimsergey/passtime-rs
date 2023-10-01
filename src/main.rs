use clap::Parser;
use colored::Colorize;
use fastrand;
use std::iter::repeat_with;

#[derive(Parser)]
#[command(author, version, about)]
#[command(arg_required_else_help(true))]
struct Args {
    /// Password length
    #[arg(value_parser = clap::value_parser!(u8).range(1..))]
    #[arg(value_name = "LENGTH")]
    len: u8,

    /// Include lowercase letters
    #[arg(short = 'a')]
    lowercase: bool,

    /// Include capital letters
    #[arg(short = 'A')]
    capital: bool,

    /// Include numbers
    #[arg(short = 'n')]
    numbers: bool,

    /// Include custom characters
    #[arg(short = 'c', value_name = "CHARACTERS")]
    custom_characters: Option<String>,

    /// Print text before the password
    #[arg(short = 'i', value_name = "TEXT")]
    additional_info: Option<String>,
}

fn main() {
    let args = Args::parse();

    let symbols = get_symbols(&args);
    let info = args.additional_info;

    let password: Vec<char> = repeat_with(|| symbols[fastrand::usize(..symbols.len())])
        .take(args.len as usize)
        .collect();

    println!(
        "{}",
        match info {
            Some(info) => format!("{info}: {}", password.iter().collect::<String>()),
            _ => password.iter().collect::<String>(),
        }
    );
}

fn get_symbols(args: &Args) -> Vec<char> {
    check_flags(args);

    let mut symbols = Vec::<char>::new();

    if args.lowercase {
        symbols.extend('a'..='z');
    }
    if args.capital {
        symbols.extend('A'..='Z');
    }
    if args.numbers {
        symbols.extend('0'..='9');
    }
    if let Some(chars) = &args.custom_characters {
        for _ in 0..=(symbols.len() / chars.len()) {
            symbols.extend(chars.chars());
        }
    }

    symbols
}

fn check_flags(args: &Args) {
    if (args.lowercase, args.capital, args.numbers) == (false, false, false) {
        eprintln!(
            "{} At least one of these flags must be given: {}

{} {} [OPTIONS] {}

For more information, try '{}'.",
            "error:".bold().red(),
            "-a -A -n".yellow(),
            "Usage:".bold().underline(),
            "passtime".bold(),
            args.len,
            "--help".bold()
        );
        std::process::exit(1);
    }
}
