use std::io;

fn convert_to_int(input: String) -> i32 {
    let input: i32 = input.trim().parse::<i32>().unwrap();
    return input;
}

fn main() {
    println!("Insira o número positivo para calcular o fatorial: ");
    let mut number: String = String::new();
    io::stdin().read_line( &mut number).expect("error");
    let number: i32 = convert_to_int(number);
    if number <= 0 {
        println!("Número inválido");
    } else {
        let mut result: i32 = 1;
        for i in 1..number+1 {
            result *= i;
        }
        println!("Fatorial de {} é: {}", number, result);
    }
}