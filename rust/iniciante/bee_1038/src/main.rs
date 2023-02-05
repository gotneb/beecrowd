use std::io;

fn read_input() -> (i32, i32) {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("");

    let mut input = input.split_whitespace();

    // Idk if this kind of syntax is beauty...
    (
    input.next()
    .unwrap()
    .parse()
    .unwrap(),
    input.next()
    .unwrap()
    .parse()
    .unwrap(),
    )
}

fn main() {
   let (code, quantity) = read_input();

    let price = match code {
        1 => 4.00,
        2 => 4.50,
        3 => 5.00,
        4 => 2.00,
        5 => 1.50,
        _ => 0.0,
    };

    let value = price * f64::from(quantity);
    println!("Total: R$ {:.2}", value);
}
