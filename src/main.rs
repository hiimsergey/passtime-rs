// TODO
// lib.rs
// print_on_file (idk)
// add figlet banner
// Cargo.toml

use rand::Rng;
use std::io;
use colored::Colorize;

const ASK_TYPES: [&str; 6] = [
    "lowercase letters",
    "capital letters",
    "numbers",
    "spaces",
    "special characters",
    "other characters"
];

fn main() {
    let mut symbols = Vec::<String>::new();

    for ask_type in ASK_TYPES {
        print_dialogue(ask_type);

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect(
                format!("{}",
                "Failed to get answer!".red())
                    .as_str()
            );

        match answer.trim() {
            x if ask_type == "other characters"
                => {
                    for _ in 0..(match symbols.len() {
                        0 => 1,
                        _ => symbols.len()
                    }) {
                        symbols.extend(x
                            .chars()
                            .map(String::from)
                            .collect::<Vec<String>>()
                        )
                    }
                },
                // => symbols.extend(x // TODO mv to get_symbols
                //     .chars()
                //     .map(String::from)
                //     .collect::<Vec<String>>()),
            "" if ask_type == "other characters"
                => {},
            "y" | "Y" | ""
                => symbols.extend(get_symbols(ask_type)),
            "n" | "N" => {},
            _ => {
                println!("{}", "Invalid choice!".red()); // TODO how to reask the question?
            },
        }
    }

    if symbols.is_empty() {
        println!("{}", "Hey, you chose nothing!".red());
        main();
        return;
    }

    let password = gen_password(&symbols, ask_length());
    for c in &password { print!("{}", c.blue()); }
    println!();

    // TODO NEXT
    // ask for additional information
    // write password to disk
}

// TODO DOC Prints the yes/no question depending on the ASK_TYPE
fn print_dialogue(ask_type: &str) {
    print!("{}", format!("Do you want to include {}?", ask_type.cyan()).bold());

    match ask_type {
        "special characters" => println!("{}{}",
            format!(
                " [{}/{}]",
                "Y".green(),
                "n".red()
            ).bold(),
            format!("\n{}{}",
                "These: ".italic(),
                "!  \"  #  $  %  &  '  (  )  *  +  ,  -  .  /".yellow()
            )
        ),
        "spaces" => println!("{}{}",
            format!(
                " [{}/{}]",
                "Y".green(),
                "n".red()
            ).bold(),
            format!("\n{}",
                "You madlad!".italic()
            )
        ),
        "other characters" => println!("\n{}",
            "Type them below (one after another) or leave blank to skip:".italic()
        ),
        _ => println!("{}",
            format!(
                " [{}/{}]",
                "Y".green(),
                "n".red()
            ).bold()
        )
    }
}

// TODO DOC Returns a vector of all the symbols corresponding to the chosen ASK_TYPE
fn get_symbols(ask_type: &str) -> Vec<String> {
    if ask_type == "spaces" {
        return vec![String::from(" ")]
    }
    match ask_type {
        "lowercase letters" => 'a'..='z',
        "capital letters"   => 'A'..='Z',
        "numbers"           => '0'..='9',
        _                   => '!'..='/'
    }.map(|c| c.to_string()).collect::<Vec<_>>()
}

fn ask_length() -> u8 {
    println!("{}", format!("{} should the password be?", "How long".cyan()).bold());

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect(
            format!("{}",
            "Failed to get answer!".red())
                .as_str()
        );

    match answer.trim().parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            println!("{}", "Invalid number!".red());
            ask_length()
        }
    }
}

fn gen_password(symbols: &Vec<String>, pass_len: u8) -> Vec<&str> {
    let mut password = Vec::<&str>::new();

    for _ in 1..=pass_len {
        password.push(&symbols[
            rand::thread_rng().gen_range(0..symbols.len())
        ])
    }

    loop {
        println!();
        for c in &password { print!("{}", c.blue() ) }
        println!("{}",
            format!(
                "\nAre you satisfied with this password? [{}/{}]",
                "Y".green(),
                "n".red()
            ).bold()
        );

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect(
                format!("{}",
                "Failed to get answer!".red())
                    .as_str()
            );

        match answer.trim() {
            "y" | "Y" | "" => return password,
            "n" | "N" => {
                return gen_password(symbols, pass_len)
            },
            _ => {
                println!("{}", "Invalid choice!".red());
                continue;
            }
        }
    }
}
