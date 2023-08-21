use std::{collections::HashMap, io, num::ParseIntError};

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParseIntError> {
    let numbers = card
        .split_whitespace()
        .into_iter()
        .map(|s| {
            s.parse()
        })
        .collect::<Result<Vec<u32>,_>>()?;

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

fn parse_card(card: &str) -> Result<Card, String> {
    let mut numbers = parse_card_numbers(card)
        .map_err(|e| e.to_string())?;

    let len = numbers.len();
    let expected_len = 4;
    if len!= expected_len {
        return Err(format!("Expected {} numbers, got {}. Elements: {numbers:?}", expected_len, len));
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

fn get_credit_card_info (
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, String> {
    let card_string = credit_cards.get(name)
        .ok_or(format!("No credit card was found for {name}"))?;

    let card = parse_card(card_string)?;

    Ok(card)
}

 

fn main() {
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
        Err(e) => {
            println!("Error: {e}");
        },
        Ok(card) => {
            println!("\nCredit Card Info: {card:?}");
        },
    }
}
