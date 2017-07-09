#[macro_use]
extern crate dbc;

fn main() {
    println!("Starting...");
    require!(true);
    let msg = "This is a test";
    let a = 3;
    require!(false, msg, a);
}
