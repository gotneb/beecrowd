use std::io;

fn main() {
    const PI: f64 = 3.14159;
    let mut raio = String::new();

    io::stdin()
        .read_line(&mut raio)
        .expect("");
    let raio: f64 = raio
        .trim()
        .parse()
        .unwrap();
    

    let volume = 4.0/3.0*PI*raio.powf(3.0);
    println!("VOLUME = {:.3}", volume);
}
