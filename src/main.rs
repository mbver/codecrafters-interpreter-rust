#![allow(unused_variables)]
use std::env;
use std::fs;
use std::process;

use crate::interpreter::Interpreter;
use crate::interpreter::print_value;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::stmt::Stmt;

mod expr;
mod interpreter;
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
            if let Some(stmts) = parser.parse() {
                for stmt in stmts {
                    if let Stmt::Expression(expr) = stmt {
                        println!("{}", printer::ast_print(&expr));
                    }
                }
            } else {
                println!("");
            }
            if parser.has_error {
                process::exit(65);
            }
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
            let Some(stmts) = parser.parse() else {
                eprintln!("No expression to evaluate");
                return;
            };
            if parser.has_error {
                process::exit(65);
            }
            let mut interpreter = Interpreter::new();
            for stmt in stmts {
                if let Stmt::Expression(expr) = stmt {
                    match interpreter.evaluate(&expr) {
                        Ok(value) => print_value(&value),
                        Err(e) => {
                            eprintln!("{}\n[line {}]", e.msg, e.token.line);
                            interpreter.has_error = true;
                            break;
                        }
                    }
                }
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
            let Some(stmts) = parser.parse() else {
                eprintln!("No expression to evaluate");
                return;
            };
            if parser.has_error {
                process::exit(65);
            }
            let mut interpreter = Interpreter::new();
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
