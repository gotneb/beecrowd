use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("");

    io::stdin()
        .read_line(&mut b)
        .expect("");
    
    let a:f64 = a.trim().parse().expect("");
    let b:f64 = b.trim().parse().expect("");

    let media = (a*3.5 + b*7.5) / (3.5 + 7.5);

    println!("MEDIA = {:.5}", media);
}
