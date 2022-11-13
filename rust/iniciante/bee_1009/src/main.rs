use std::io;

fn main() {
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("");

    let mut sal_fixo = String::new();
    io::stdin()
        .read_line(&mut sal_fixo)
        .expect("");
    let sal_fixo: f64 = sal_fixo.trim().parse().unwrap(); 

    let mut montante = String::new();
    io::stdin()
        .read_line(&mut montante)
        .expect("");
    let montante: f64 = montante.trim().parse().unwrap();

    let salario = sal_fixo + 0.15 * montante;
    println!("TOTAL = R$ {:.2}", salario);
}
