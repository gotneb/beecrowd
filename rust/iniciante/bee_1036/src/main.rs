use std::io;

fn main() {
    let mut input = String::from("");

    io::stdin()
    .read_line(&mut input)
    .expect("");

    let input = input.split_whitespace();

    let mut inputs = [0.0; 3];

    for (index, value) in input.enumerate() {
        inputs[index] = String::from(value).parse().unwrap();
    }

    solve_eq(inputs[0], inputs[1], inputs[2]);
}

fn solve_eq(a:f64, b:f64, c:f64) {
    let delta = b.powi(2) - 4.0*a*c;

    if delta < 0.0 || a == 0.0 {
        println!("Impossivel calcular");
    } else {
        let r1 = (-b + delta.sqrt()) / (2.0*a);
        let r2 = (-b - delta.sqrt()) / (2.0*a);

        println!("R1 = {:.5}\nR2 = {:.5}", r1, r2);
    }
}