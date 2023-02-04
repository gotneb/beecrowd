use std::io;

fn main() {
    let mut input = String::from("");

    io::stdin()
    .read_line(&mut input)
    .expect("");

    let input = input.split_whitespace();

    let mut inputs = [0; 4];
    for (index, value) in input.enumerate() {
        inputs[index] = String::from(value).parse().unwrap();
    }

    check(inputs[0], inputs[1], inputs[2], inputs[3]);
}
fn check(a:i32, b:i32, c:i32, d:i32) {
    if b > c && d > a && (c+d) > (a+b) && c > 0 && d > 0 && a%2 == 0 {
        println!("Valores aceitos")
    } else {
        println!("Valores nao aceitos")
    }
}