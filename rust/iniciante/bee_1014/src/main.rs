use std::io;

fn main() {
    let mut input = String::new();

    read_input(&mut input);
    let distance: i32 = input.trim().parse().unwrap();

    input.clear();
    read_input(&mut input);
    let fuel:f64 = input.trim().parse().unwrap();

    let gradient = f64::from(distance)/ fuel;

    println!("{:.3} km/l", gradient);
}

fn read_input(input: &mut String) {
    io::stdin()
    .read_line(input)
    .expect("error");
}