use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};

#[derive(Clone, Copy)]
enum Operator {
    None,
    Add,
    Multiply,
}

#[derive(Clone, Copy)]
enum Token {
    Add,
    Multiply,
    Value(usize),
    StartParen,
    EndParen,
}

fn tokenise(parts: Vec<String>) -> Vec<Token> {
    let mut tokens = Vec::new();

    for mut part in parts {
        let mut need_end_paren = 0;

        while part.starts_with('(') {
            tokens.push(Token::StartParen);
            part = part.strip_prefix('(').unwrap().to_string();
        }
        while part.ends_with(')') {
            part = part.strip_suffix(')').unwrap().to_string();
            need_end_paren += 1;
        }

        tokens.push(match part.as_str() {
            "+" => Token::Add,
            "*" => Token::Multiply,
            x => Token::Value(usize::from_str(x).unwrap()),
        });

        for _ in 0..need_end_paren {
            tokens.push(Token::EndParen);
        }
    }

    tokens
}

fn evaluate<I: Iterator<Item = Token>>(tokens: &mut I) -> usize {
    let mut accumulator = 0;
    let mut operator = Operator::None;

    while let Some(token) = tokens.next() {
        match token {
            Token::Add => operator = Operator::Add,
            Token::Multiply => operator = Operator::Multiply,
            Token::Value(x) => match operator {
                Operator::None => accumulator = x,
                Operator::Add => accumulator += x,
                Operator::Multiply => accumulator *= x,
            },
            Token::StartParen => {
                let x = evaluate(tokens);
                match operator {
                    Operator::None => accumulator = x,
                    Operator::Add => accumulator += x,
                    Operator::Multiply => accumulator *= x,
                }
            }
            Token::EndParen => break,
        }
    }

    accumulator
}

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 18")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let parts: Vec<String> = line.split(' ').map(|v| v.to_string()).collect();
        let result = evaluate(&mut tokenise(parts).iter().cloned());
        sum += result;

        println!("{} = {}", line, result);
    }

    println!();
    println!("Total: {}", sum);

    Ok(())
}
