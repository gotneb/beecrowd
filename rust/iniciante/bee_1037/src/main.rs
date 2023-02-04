use std::io;

fn read_input() -> f64 {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("");

    let input:f64 = input
    .trim()
    .parse()
    .unwrap();
    
    input
}

fn main() {
    let input = read_input();

    if input < 0.0 || input > 100.0 {
        println!("Fora de intervalo");
    } else if input <= 25.0 {
        println!("Intervalo [0,25]");
    } else if input <= 50.0 {
        println!("Intervalo (25,50]");
    } else if input <= 75.0 {
        println!("Intervalo (50,75]");
    } else if input <= 100.0 {
        println!("Intervalo (75,100]");
    }
}
