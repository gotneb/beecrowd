use std::io;

fn main() {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("");
    let number:i32 = number
        .trim()
        .parse()
        .expect("");
    
    let mut horas = String::new();
    io::stdin()
        .read_line(&mut horas)
        .expect("");
    let horas:i32 = horas
        .trim()
        .parse()
        .expect("");
    
    let mut valor = String::new();
    io::stdin()
        .read_line(&mut valor)
        .expect("");
    let valor:f64 = valor.trim().parse().expect("");
    
    println!("NUMBER = {}\nSALARY = U$ {:.2}", number, (horas as f64) * valor);
}
