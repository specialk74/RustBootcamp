// Fix this code with shadowing

fn main() {
    let x = "three"; // don't change this line
    println!("Spell a Number : {}", x);
    let x = 3; // don't rename this variable
    println!("Number plus two is : {}", x + 2);

    let s1 = generate_string();
    println!("{s1}");
    let s2 = generate_string();
    println!("{s2}");
}

fn generate_string() -> String {
    String::from("hello")
}