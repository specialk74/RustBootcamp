fn main() {
    let player1 = String::from("player 1");
    let result;
    {    
        let player2 = String::from("player 2");

        result = first_turn(player1.as_str(), player2.as_str());
    }
    println!("Player going first is: {}", result);
}

fn first_turn<'a> (player1: &'a str, player2: &'a str) -> &'a str {
    if rand::random() {
        player1
    }
    else {
        player2
    }
}

fn second_turn(player1: &str, player2: &str) -> &'static str {
    let s: &'static str = "Let's Get Rusty!";
    s
}