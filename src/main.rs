use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    let mut first_number = String::new();
    println!("İlk sayıyı girin:");
    io::stdin().read_line(&mut first_number).expect("Giriş okunamadı.");
    let first_number: f64 = first_number.trim().parse().expect("Geçerli bir sayı girin.");

    let mut operation = String::new();
    println!("Yapılacak işlemi girin (+, -, *, /):");
    io::stdin().read_line(&mut operation).expect("Giriş okunamadı.");
    let operation = operation.trim();

    let mut second_number = String::new();
    println!("İkinci sayıyı girin:");
    io::stdin().read_line(&mut second_number).expect("Giriş okunamadı.");
    let second_number: f64 = second_number.trim().parse().expect("Geçerli bir sayı girin.");

    let operation = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Geçersiz işlem.");
            return;
        }
    };

    let result = calculate(operation);
    println!("Sonuç: {}", result);
}
