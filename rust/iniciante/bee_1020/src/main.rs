use std::io;

fn main() {
    let mut days = read_input();

    const YEAR_IN_DAYS:i32 = 365;
    const MONTH_IN_DAYS:i32 = 30;

    let years = days/YEAR_IN_DAYS;
    days -= years * YEAR_IN_DAYS;

    let months = days/MONTH_IN_DAYS;
    days -= months * MONTH_IN_DAYS;

    println!("{} ano(s)\n{} mes(es)\n{} dia(s)", years, months, days);
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
