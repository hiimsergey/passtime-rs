use colored::Colorize;
use rand::Rng;
use std::{
    env,
    fs::OpenOptions,
    io::{self, Write}
};

const ASK_TYPES: [&str; 5] = [
    "lowercase letters",
    "capital letters",
    "numbers",
    "special characters",
    "other characters"
];

fn main() {
    // TODO print exe if on windows, not if otherwise
    let file_name = match env::args().nth(1) {
        Some(arg) => arg,
        None => return eprintln!("{}\n\n{}{}{}\n",
        "Didn't get any file to write into!".red(),
        "Tip: execute the program like that: ".green(),
        "rng_passgen.exe ",
        "file_name.txt".yellow().italic()
    )
    };
    println!("{}", format!(
" ____  _   _  ____                                             {}
|  _ \\| \\ | |/ ___|  _ __   __ _ ___ ___  __ _  ___ _ __       {}
| |_) |  \\| | |  _  | '_ \\ / _` / __/ __|/ _` |/ _ \\ '_ \\
|  _ <| |\\  | |_| | | |_) | (_| \\__ \\__ \\ (_| |  __/ | | |     {}hiimsergey/rng-passgen-rs
|_| \\_\\_| \\_|\\____| | .__/ \\__,_|___/___/\\__, |\\___|_| |_|
                    |_|                  |___/
",
"rng-passgen-rs".white().bold(),
"a kinda useless problem".white().italic(),
"github.com/".white()
).magenta());

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
            x if ask_type == "other characters" => {
                    for _ in 0..(match symbols.len() {
                        0 => 1,
                        _ => symbols.len()
                    }) { symbols.extend(get_symbols(x)); }
                },
            "" if ask_type == "other characters" => {},
            "y" | "Y" | "" => symbols.extend(get_symbols(ask_type)),
            "n" | "N" => {},
            _ => { println!("{}", "Invalid choice!".red()); }, // TODO how to reask the question?
        }
    }

    if symbols.is_empty() {
        println!("{}", "Hey, you chose nothing!".red());
        main();
        return;
    }

    let password = gen_password(&symbols, ask_length());

    println!("{}\n{}",
        format!(
            "Do you want {} printed?",
            "additional information".cyan()
        ).bold(),
        "Just type whatever you need below or leave blank to skip:".italic()
    );
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect(
            format!("{}",
            "Failed to get answer!".red())
                .as_str()
        );

    // TODO make it create files if not there
    // TODO error handle this (Result)
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();

    if answer.trim() != "" {
        write!(file, "{} ", answer.trim());
    }
    // TODO make it print different password
    // TODO fundamentally change the program so at first, the settings get constructed (lib.rs)
    for _ in 1..=password_number() {
        for c in &password { write!(file, "{c}"); }
        writeln!(file);
    }
}

fn print_dialogue(ask_type: &str) {
    print!("{}",
        format!(
            "Do you want to include {}?",
            ask_type.cyan()
        ).bold()
    );
    if ask_type != "other characters" {
        println!(" {}",
            format!("[{}/{}]",
                "Y".green(),
                "n".red()
            ).bold()
        );
    } else {
        print!("\n");
    }

    match ask_type {
        "special characters" => println!("{}{}",
            "These: ".italic(),
            "!  \"  #  $  %  &  '  (  )  *  +  ,  -  .  /".yellow()
        ),
        "other characters" => println!("{}",
            "Type them below (one after another) or leave blank to skip:".italic()
        ),
        _ => {}
    }
}

fn get_symbols(ask_type: &str) -> Vec<String> {
    match ask_type {
        "lowercase letters"  => 'a'..='z',
        "capital letters"    => 'A'..='Z',
        "numbers"            => '0'..='9',
        "special characters" => '!'..='/',
        x                    => return x.chars().map(String::from).collect::<Vec<String>>()
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

fn password_number() -> u8 {
    println!("{}",
        format!(
            "{} passwords to generate? [{}/{}]",
            "How many".cyan(),
            "1".green(),
            "number".red().italic()
        )
    );

    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect(
            format!("{}",
            "Failed to get answer!".red())
                .as_str()
        );

    if answer.trim() == "" { return 1 }
    match answer.trim().parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            println!("{}", "Invalid number!".red());
            password_number()
        }
    }
}
