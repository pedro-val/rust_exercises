use std::io;

fn convert_to_int(input: String) -> i32 {
    let input: i32 = input.trim().parse::<i32>().unwrap();
    return input;
}

fn main() {
    let mut number1: String = String::new();
    println!("Digite o primeiro número: ");
    io::stdin().read_line( &mut number1).expect("error");
    let mut number2: String = String::new();
    println!("Digite o segundo número: ");
    io::stdin().read_line( &mut number2).expect("error");
    let number1: i32 = convert_to_int(number1);
    let number2: i32 = convert_to_int(number2);
    let mut while_conditional: bool = true;
    let mut counter: i32 = std::cmp::min(number1, number2);
    while while_conditional {
        if number1 % counter == 0 && number2 % counter == 0 {
            println!("MDC: {}", counter);
            while_conditional = false;
        }
        counter -= 1;
    }

}