use std::io;

fn main() {
    let inputs = read_input();
    println!(
        "TRIANGULO: {:.3}\nCIRCULO: {:.3}\nTRAPEZIO: {:.3}\nQUADRADO: {:.3}\nRETANGULO: {:.3}",
        triangle(inputs.0, inputs.2),
        circle(inputs.2),
        trapezium(inputs.0, inputs.1, inputs.2),
        square(inputs.1),
        rectangle(inputs.0, inputs.1)
    );
}

fn read_input() -> (f64, f64, f64) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let mut x = input.split_whitespace();
    
    let a = x.next().unwrap();
    let b = x.next().unwrap();
    let c = x.next().unwrap();

    let a: f64 = a.trim().parse().unwrap();
    let b: f64 = b.trim().parse().unwrap();
    let c: f64 = c.trim().parse().unwrap();

    (a, b, c)
}

fn triangle(a: f64, c: f64) -> f64 {
    a * c / 2.0
}

fn circle(c: f64) -> f64 {
    const PI: f64 = 3.14159;
    PI * c.powi(2)
}

fn trapezium(a: f64, b: f64, c: f64) -> f64 {
    (a + b) * c / 2.0
}

fn square(b: f64) -> f64 {
    b.powi(2)
}

fn rectangle(a: f64, b: f64) -> f64 {
    a * b
}
