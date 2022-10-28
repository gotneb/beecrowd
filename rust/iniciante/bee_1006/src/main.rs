use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("");

    io::stdin()
        .read_line(&mut b)
        .expect("");
    
    io::stdin()
        .read_line(&mut c)
        .expect("");
    
    let a:f64 = a.trim().parse().expect("");
    let b:f64 = b.trim().parse().expect("");
    let c:f64 = c.trim().parse().expect("");

    let media = (a*2.0 + b*3.0 + c*5.0) / (2.0 + 3.0 + 5.0);

    println!("MEDIA = {:.1}", media);
}