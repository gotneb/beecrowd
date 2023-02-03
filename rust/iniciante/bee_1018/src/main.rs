use std::io;

fn main() {
    let mut total = read_input();
    println!("{}", total);

    let money = [100, 50, 20, 10, 5, 2, 1];

    for v in money.iter() {
        let amount = total/v;
        total -= amount * v;
        println!("{} nota(s) de R$ {},00", amount, v);
    }

}

fn read_input() -> i32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("");

    input
    .trim()
    .parse()
    .unwrap()
}
