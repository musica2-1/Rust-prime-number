use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("erro ao ler");

    let n = input.trim().parse::<i32>().expect("digite um número");

    let mut prime = true;

    if n < 2 || (n % 2 == 0 && n != 2) {
        prime = false;
    } else {
        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                prime = false;
                break;
            }
            i += 2;
        }
    }

    println!("{prime}");
}