use std::io;

fn read_input() -> (f64, f64) {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("");

    let mut input = input.split_whitespace();

    let x = input.next().unwrap();
    let y = input.next().unwrap();

    (x.parse().unwrap(), y.parse().unwrap())
}

fn main() {
    let (x, y) = read_input();

    if x == 0.0 && y == 0.0 {
        println!("Origem");
    } else if x == 0.0 {
        println!("Eixo Y");
    } else if y == 0.0 {
        println!("Eixo X");
    } else if x > 0.0 && y > 0.0 {
        println!("Q1");
    } else if x < 0.0 && y > 0.0 {
        println!("Q2");
    } else if x < 0.0 && y < 0.0 {
        println!("Q3");
    } else if x > 0.0 && y < 0.0 {
        println!("Q4")
    }
}
