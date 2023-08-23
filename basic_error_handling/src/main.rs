use std::{collections::HashMap, io, error::Error, fmt::Display};


struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: String,
}

impl std::fmt::Debug for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self}\n\t{}", self.msg));

        if let Some(e) = self.source.as_ref() {
            write!(f, "\n\nCaused by\n\t{e:?}")?;
        }
        Ok(())
    }
}

impl Error for ParsePaymentInfoError {
    fn source(&self) -> Option<&(dyn Error +'static)> {
        self.source.as_deref()
    }
}

impl Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Parsing payment error: invalid payment info")
    }
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split_whitespace()
        .into_iter()
        .map(|s| {
            s.parse()
            .map_err(|_| 
                ParsePaymentInfoError 
                {
                    source: None,
                    msg: format!("{s:?} could not be parsed as u32")
                }
            )
        })
        .collect::<Result<Vec<u32>,_>>()
        .map_err(|e| ParsePaymentInfoError { 
            source: Some(Box::new(e)), 
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


enum CreditCardError {
    InvalidInput(String),
    Other(Box<dyn Error>, String),
}

impl std::fmt::Debug for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreditCardError::InvalidInput(s) => write!(f, "{self}\nInvalid input: {s}"),
            CreditCardError::Other(e, s) => write!(f, "{self}\nOther error: {e:?}: {s}"),
        }
    }
}

impl Error for CreditCardError {
    fn source(&self) -> Option<&(dyn Error +'static)> {
        match self {
            CreditCardError::InvalidInput(_) => None,
            CreditCardError::Other(e, _) => Some(e.as_ref()),
        }
    }
}

impl Display for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreditCardError::InvalidInput(msg) => write!(f, "{}", msg),
            CreditCardError::Other(e, msg) => write!(f, "{}", msg),
        }
    }
}

fn get_credit_card_info (
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name)
        .ok_or(CreditCardError::InvalidInput(format!("No credit card was found for {name}")))?;

    let card = parse_card(card_string)
        .map_err(|e| 
            CreditCardError::Other(Box::new(e), format!("{name}'s card could not be parsed."))
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
                CreditCardError::Other(_, _) => println!("Somenthing went wrong!"),
            }
            log::error!("\n{err:?}");
        },
    }
}
