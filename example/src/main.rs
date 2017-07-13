#[macro_use]
extern crate dbc;

use dbc::Invariant;

#[derive(Debug)]
struct AA(i32);

#[derive(Debug)]
struct BB(AA);

#[derive(Debug)]
struct Rectangle {
    length: i32,
    width: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        invariant!(self);

        self.length * self.width
    }
}

impl Invariant for Rectangle {
    fn invariant(&self) -> bool {
        self.length > 0 && self.width > 0
    }
}

fn main() {
    println!("Starting...");
    let a = 34;
    let b = BB(AA(234));
    let msg = "My message";

    println!("{}", formatvar!(a));
    println!("{}", formatvar!(b));
    println!("{}", formatvar!(msg,a,b));

    require!(true);
    let msg = "This is a test";
    let a = 3;
    require!(true, msg, a);

    let r = Rectangle{
        length: 100,
        width: 0,
    };
    println!("Area is {:?}", r.area());
}
