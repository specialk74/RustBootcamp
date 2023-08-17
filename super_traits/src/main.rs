// Something is missing with the definition of Comparable trait. Fix it.

trait Numeric {
    fn convert_to_num(&self) -> u8;
}

trait Printable {
    fn convert_to_str(&self) -> String;
}

trait Comparable: Numeric + Printable {
    fn print_greater(a: &Self, b: &Self) {
        let num1 = a.convert_to_num();
        let num2 = b.convert_to_num();
        if num1 > num2 {
            println!(
                "{} is greater than {}",
                a.convert_to_str(),
                b.convert_to_str()
            );
        } else if num2 > num1 {
            println!(
                "{} is greater than {}",
                b.convert_to_str(),
                a.convert_to_str()
            );
        } else {
            println!("Both sizes are {}", a.convert_to_str());
        }
    }
}

enum Size {
    Small,
    Medium,
    Large,
}

impl Numeric for Size {
    fn convert_to_num(&self) -> u8 {
        match self {
            Self::Small => 0,
            Self::Medium => 1,
            Self::Large => 2,
        }
    }
}

impl Printable for Size {
    fn convert_to_str(&self) -> String {
        match self {
            Self::Small => "Small size".to_string(),
            Self::Medium => "Medium size".to_string(),
            Self::Large => "Large size".to_string(),
        }
    }
}

impl Comparable for Size {}

fn main() {
    let (size1, size2) = (Size::Small, Size::Medium);
    Comparable::print_greater(&size1, &size2);
}
