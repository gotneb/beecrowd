use std::io;

fn main() {
    let (x1, y1) = read_input();
    let (x2, y2) = read_input();
    
    println!("{:.4}", distance(x1, y1, x2, y2));
}

fn read_input() -> (f64, f64) {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("");

    let mut input = input.split_whitespace();
    let x = input
    .next()
    .unwrap()
    .trim()
    .parse()
    .unwrap();
    
    let y = input
    .next()
    .unwrap()
    .trim()
    .parse()
    .unwrap();

    (x, y)
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}