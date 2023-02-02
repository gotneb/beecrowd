use std::io;

fn main() {
    let time = read_input();
    let speed = read_input();
    let distance = speed * time;

    let fuel = f64::from(distance)/12.0;

    println!("{:.3}", fuel)
}

fn read_input() -> i32 {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("");

    input
    .trim()
    .parse()
    .unwrap()
}