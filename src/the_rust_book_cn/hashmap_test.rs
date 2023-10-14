// this is an interactive program to show the basic usage of std::collections::HashMap

use std::{collections::HashMap, io::Write};

fn print_help() {
    println!("help: get help\nadd <name> <score>: add a new record\nget <name> get a record\nset <name> <value> set a record value\nexit\nprint")
}

fn add_to_record(m: &mut HashMap<String, u32>, name: &str, value: &str) {
    let value = value.parse();
    if let Result::Err(_) = value {
        println!("error: can not parse to a int.");
        return;
    }
    let value: u32 = value.unwrap();
    match m.get(name) {
        Some(_) => {
            println!("error: already in record.");
            return;
        }
        None => {
            m.insert(name.to_string(), value);
        }
    }
    println!("Successfully added.");
}

fn get_from_record(m: &HashMap<String, u32>, name: &str) {
    match m.get(name) {
        Some(value) => println!("{}", value),
        None => println!("we don't have this record."),
    }
}

fn set_entry(m: &mut HashMap<String, u32>, name: &str, value: &str) {
    let value = value.parse();
    if let Result::Err(_) = value {
        println!("error: can not parse to a int.");
        return;
    }
    let value: u32 = value.unwrap();
    match m.get(name) {
        Some(_) => {
            m.insert(name.to_string(), value);
        }
        None => {
            println!("error: not in record");
            return;
        }
    }
    println!("Successfully changed.");
}

fn is_enough_argument(v: &Vec<&str>, n: usize) -> bool {
    if v.len() < n {
        println!("error: no enough arguments.");
        return false;
    }
    true
}

fn print(m: &HashMap<String, u32>) {
    for i in m {
        println!("{} {}", i.0, i.1);
    }
}

#[allow(dead_code)]
pub fn main7() {
    use std::io;

    let mut scores: HashMap<String, u32> = HashMap::new();

    println!("You are in a program that tracks scores of different teams. Type commands below. Type help to get help.");

    loop {
        print!("> ");
        io::stdout().flush().expect("error");

        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("Can not read from command line.");
        let s: Vec<&str> = s.trim().split_whitespace().collect();
        let command = s[0];

        match command {
            "help" => print_help(),
            "exit" => break,
            "add" => {
                if !is_enough_argument(&s, 3) {
                    continue;
                }
                add_to_record(&mut scores, s[1], s[2]);
            }
            "get" => {
                if !is_enough_argument(&s, 2) {
                    continue;
                }
                get_from_record(&scores, s[1]);
            }
            "print" => print(&scores),
            "set" => {
                if !is_enough_argument(&s, 3) {
                    continue;
                }
                set_entry(&mut scores, s[1], s[2]);
            }
            _ => println!("sorry, we don't have this command."),
        }
    }

    println!("Good bye!");
}
