use std::io;

fn main() {
    let values = read_input();
    let mut greatest = maior_ab(values.0, values.1); 
    greatest = maior_ab(greatest, values.2);
    
    println!("{} eh o maior", greatest); 
}

fn maior_ab(a: i32, b:i32) -> i32 {
    (a + b + (a - b).abs()) / 2
}

// I know I'm repeating my code..
// But I'm learning Rust and I'm going to improve and write better code! :)
fn read_input() -> (i32, i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut x = input.split_whitespace();
    
    let a = x.next().unwrap();
    let b = x.next().unwrap();
    let c = x.next().unwrap();

    let a: i32 = a.trim().parse().unwrap();
    let b: i32 = b.trim().parse().unwrap();
    let c: i32 = c.trim().parse().unwrap();

    (a, b, c)
}