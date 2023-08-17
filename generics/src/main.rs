struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new (name: String, payload: T) -> Self {
        Self {
            name: name,
            payload: payload,
        }
    }  

    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print_payload(&self) {
        println!("{}", self.payload);
    }
}

fn main() {
    let cmd1 = BrowserCommand::new(
        String::from("navigate"),
        String::from("https://www.google.com"),
    );

    let cmd2 = BrowserCommand::new(
        String::from("zoom"),
        200,
    );

    cmd1.print_payload();
    
    let p1 = cmd1.get_payload();
    let p2= cmd2.get_payload();

    serialize_payload(p1);
    serialize_payload(p2);
}

fn serialize_payload<T>(payload: &T) -> String{
    "hello".to_owned()
}