use std::io;

fn main() {
    let mut seconds = read_input();

    const HOURS_IN_SECS:i32 = 3600;
    const MINUTES_IN_SECS:i32 = 60;

    let hours = seconds/HOURS_IN_SECS;
    seconds -= hours * HOURS_IN_SECS;

    let minutes = seconds/MINUTES_IN_SECS;
    seconds -= minutes * MINUTES_IN_SECS;

    println!("{}:{}:{}", hours, minutes, seconds);
}

fn read_input() -> i32 {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("");

    let input:i32 = input
    .trim()
    .parse()
    .unwrap();

    input
}