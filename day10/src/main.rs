use std::collections::HashMap;
use std::error;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    //let point_values = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let point_values = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let corrupted_sum = 0;
    let mut valid_lines: Vec<Vec<char>> = Vec::new();

    for line in BufReader::new(std::io::stdin()).lines() {
        let mut stack: Vec<char> = Vec::new();
        let mut line_corrupted = false;
        for char in line.unwrap().chars() {
            match char {
                '[' | '(' | '{' | '<' => stack.push(char),
                curr => match (stack.pop().unwrap(), curr) {
                    ('{', '}') | ('(', ')') | ('[', ']') | ('<', '>') => {
                        continue;
                    }
                    (_, _curr) => {
                        // Corrupted line, skip it
                        //corrupted_sum += point_values[&curr];
                        line_corrupted = true;
                        break;
                    }
                },
            }
        }
        if !line_corrupted {
            valid_lines.push(stack);
        }
    }
    println!("Corrupted Sum {}", corrupted_sum);
    for line in valid_lines {
        let mut completed_sum: usize = 0;
        for char in line.iter().rev() {
            completed_sum *= 5;
            completed_sum += point_values[char];
            print!("{}", char);
        }
        println!();
        println!("Completed Sum {}", completed_sum);
    }
}
