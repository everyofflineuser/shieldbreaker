use std::io;
use std::io::Write;

mod bsod;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test Malware on Rust");
    
    loop {
        print!("Введите команду: ");
        io::stdout().flush().unwrap(); // Очистка буфера вывода

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка чтения строки");

        // Удаляем символ новой строки из введенной строки
        let input = input.trim();

        println!("\nОбработка команды...");

        match input {
            "bsod" => bsod::bsod(),
            "exit" => break,
            _ => println!("Неизвестная команда"),
        }
    }

    Ok(())
}