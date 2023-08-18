use std::rc::Rc;

struct Database { }

struct AuthService {
    db: Rc<Database>
}

struct ContentService {
    db: Rc<Database>
}

fn main() {
    let db = Rc::new(Database { });
    let auth = AuthService { db: Rc::clone(&db) };
    let content = ContentService { db: Rc::clone(&db) };
}
