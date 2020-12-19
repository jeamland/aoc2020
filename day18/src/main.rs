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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

fn evaluate2(tokens: Vec<Token>) -> usize {
    let mut tokens = tokens;
    let mut new_tokens = Vec::new();

    while let Some(start) = tokens.iter().position(|t| *t == Token::StartParen) {
        let mut end = start;
        let mut paren_depth = 0;
        for (pos, token) in tokens.iter().enumerate().skip(start + 1) {
            match token {
                Token::StartParen => paren_depth += 1,
                Token::EndParen => {
                    if paren_depth == 0 {
                        end = pos;
                        break;
                    } else {
                        paren_depth -= 1;
                    }
                }
                _ => (),
            }
        }

        let paren_span: Vec<Token> = tokens
            .iter()
            .skip(start + 1)
            .take(end - start - 1)
            .copied()
            .collect();

        let result = evaluate2(paren_span);

        new_tokens.extend(tokens.iter().take(start).copied());
        new_tokens.push(Token::Value(result));
        new_tokens.extend(tokens.iter().skip(end + 1).copied());
        tokens = new_tokens;
        new_tokens = Vec::new();
    }

    let mut in_add = false;
    let mut lhs = 0;

    for token in tokens.iter() {
        match token {
            Token::Value(value) => {
                if in_add {
                    lhs += value;
                    new_tokens.pop();
                    new_tokens.push(Token::Value(lhs));
                    in_add = false;
                } else {
                    lhs = *value;
                    new_tokens.push(Token::Value(*value));
                }
            }
            Token::Add => in_add = true,
            t => new_tokens.push(*t),
        }
    }

    new_tokens
        .iter()
        .filter_map(|t| match t {
            Token::Value(x) => Some(x),
            _ => None,
        })
        .product()
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

    let mut sum1 = 0;
    let mut sum2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let parts: Vec<String> = line.split(' ').map(|v| v.to_string()).collect();
        let tokens = tokenise(parts);

        let result1 = evaluate(&mut tokens.iter().cloned());
        sum1 += result1;

        let result2 = evaluate2(tokens);
        sum2 += result2;

        println!("{} = {}", line, result1);
        println!("{} = {}", line, result2);
        println!();
    }

    println!("Total 1: {}", sum1);
    println!("Total 2: {}", sum2);

    Ok(())
}
