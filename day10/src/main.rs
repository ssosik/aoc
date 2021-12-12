use std::error;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    println!("Hello, world!");
    for line in BufReader::new(std::io::stdin()).lines() {
        let mut stack: Vec<char> = Vec::new();
        for char in line.unwrap().chars() {
            match char {
                '[' | '(' | '{' | '<' => stack.push(char),
                curr => {
                    let last = stack.pop().unwrap();
                    match (last, curr) {
                        ('{', '}') | ('(', ')') | ('[', ']') | ('<', '>') => {
                            println!("OK: {} {}", last, curr)
                        }
                        _ => println!("NOT OK {} {}", last, curr),
                    }
                }
            }
            println!("{}", char);
        }
    }
}
