#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process;

use crate::parser::Parser;
use crate::scanner::Scanner;

mod expr;
mod parser;
mod printer;
mod scanner;
mod stmt;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents =
                fs::read_to_string(filename).unwrap_or_else(|_| {
                    eprintln!("Failed to read file {}", filename);
                    String::new()
                });

            let mut scanner = Scanner::new(&file_contents);
            scanner.scan_tokens();

            for token in scanner.get_tokens() {
                println!("{}", token);
            }
            if scanner.has_error() {
                process::exit(65)
            }
        }
        "parse" => {
            let file_contents =
                fs::read_to_string(filename).unwrap_or_else(|_| {
                    eprintln!("Failed to read file {}", filename);
                    String::new()
                });

            let mut scanner = Scanner::new(&file_contents);
            scanner.scan_tokens();

            let mut parser = Parser::new(scanner.get_tokens());
            let expr = parser.parse();
            printer::ast_print(&expr);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
