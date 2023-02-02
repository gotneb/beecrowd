use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("");

    let input:f64 = input
    .trim()
    .parse()
    .unwrap();

    println!("{} minutos", input * 2.0);
}
