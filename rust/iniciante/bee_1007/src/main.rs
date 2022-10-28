use std::io;

fn main() {
    let a = read_value();
    let b = read_value();
    let c = read_value();
    let d = read_value();

    let dif = a*b - c*d;

    println!("DIFERENCA = {}", dif);
}

fn read_value() -> i32 {
    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("");
    
    let value:i32 = value.trim().parse().expect("");
    value
}
