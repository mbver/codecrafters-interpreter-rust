#![allow(unused_variables)]
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

use crate::interpreter::Interpreter;
use crate::interpreter::print_value;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::runtime_error::RuntimeError;
use crate::scanner::Scanner;

mod callable;
mod environment;
mod expr;
mod interpreter;
mod parser;
mod printer;
mod resolver;
mod runtime_error;
mod runtime_value;
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
            let expr = parser.parse_expr();
            if parser.has_error {
                process::exit(65);
            }
            println!("{}", printer::ast_print(&expr.unwrap()));
        }
        "evaluate" => {
            let file_contents =
                fs::read_to_string(filename).unwrap_or_else(|_| {
                    eprintln!("Failed to read file {}", filename);
                    String::new()
                });

            let mut scanner = Scanner::new(&file_contents);
            scanner.scan_tokens();
            if scanner.has_error() {
                process::exit(65)
            }
            let mut parser = Parser::new(scanner.get_tokens());
            let expr = parser.parse_expr();
            if parser.has_error {
                process::exit(65);
            }
            let expr = expr.unwrap();
            let mut interpreter = Interpreter::new(HashMap::new());
            match interpreter.evaluate(&expr) {
                Ok(value) => print_value(&value),
                Err(e) => match e {
                    RuntimeError::Error(e) => {
                        eprintln!("{}\n[line {}]", e.msg, e.token.line);
                        interpreter.has_error = true;
                    }
                    _ => unreachable!(),
                },
            }
            if interpreter.has_error {
                process::exit(70);
            }
        }
        "run" => {
            let file_contents =
                fs::read_to_string(filename).unwrap_or_else(|_| {
                    eprintln!("Failed to read file {}", filename);
                    String::new()
                });

            let mut scanner = Scanner::new(&file_contents);
            scanner.scan_tokens();
            if scanner.has_error() {
                process::exit(65)
            }
            let mut parser = Parser::new(scanner.get_tokens());
            let stmts = parser.parse();
            if parser.has_error {
                process::exit(65);
            }
            let mut resolver = Resolver::new();
            if let Err(e) = resolver.resolve_stmts(&stmts) {
                eprintln!("{}\n [line {}]\n", e.msg, e.token.line);
                process::exit(65);
            }
            let mut interpreter = Interpreter::new(resolver.into_locals());
            interpreter.interpret(&stmts);
            if interpreter.has_error {
                process::exit(70);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
