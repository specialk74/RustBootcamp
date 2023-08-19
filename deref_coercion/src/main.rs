use std::ops::{Deref, DerefMut};

mod employee;
use employee::Employee;


struct MySmartPointer<T> {
    value: T
}

impl <T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}

impl <T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl <T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

fn main() {
    let s = MySmartPointer::new(Box::new("hello world".to_owned()));
    let s = &(***s);
    print(s);

    employee::prova();
}

fn print(s: &str) {
    println!("{}", s);
}
