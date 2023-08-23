use std::{collections::HashMap, io, error::Error, fmt::Display};

use anyhow::Context;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{msg}")]
struct ParsePaymentInfoError {
    source: Option<anyhow::Error>,
    msg: String,
}


fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split_whitespace()
        .into_iter()
        .map(|s| {
            s.parse()
            .with_context(|| format!("{s:?} could not be parsed as u32"))
        })
        .collect::<Result<Vec<u32>,_>>()
        .map_err(|e| ParsePaymentInfoError { 
            source: Some(e), 
            msg: format!("Failed to parse input as numbers. Input: {card}") 
        })?;

    Ok(numbers)
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32,
}


#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;
    if len!= expected_len {
        return Err(
            ParsePaymentInfoError { 
                source: None,
                msg: format!("Expected {} numbers, got {}. Elements: {numbers:?}", expected_len, len),
            }
        );
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month},
        cvv
    })
}

#[derive(Error, Debug)]
enum CreditCardError {
    #[error("{0}")]
    InvalidInput(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

fn get_credit_card_info (
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name)
        .ok_or(CreditCardError::InvalidInput(format!("No credit card was found for {name}")))?;

    let card = parse_card(card_string)
        .with_context(|| format!("{name}'s card could not be parsed."))
        .map_err(|e| 
            CreditCardError::Other(e)
        )?;

    Ok(card)
}

 

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from ([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27"),
        ("Bob", "1234567 Dec 27 123"),
    ]);

    println!("Enter Name:");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());
    
    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            match &err {
                CreditCardError::InvalidInput(msg) => println!("Invalid input: {msg}"),
                CreditCardError::Other(_) => println!("Somenthing went wrong!"),
            }
            log::error!("\n{err:?}");
        },
    }
}
