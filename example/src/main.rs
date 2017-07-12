#[macro_use]
extern crate dbc;

#[derive(Debug)]
struct AA(i32);

#[derive(Debug)]
struct BB(AA);


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
    require!(false, msg, a);
}
