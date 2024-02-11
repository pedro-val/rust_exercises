use std::io;

fn convert_to_int(input: String) -> Result<i32, &'static str> {
    match input.trim().parse::<i32>() {
        Ok(val) => Ok(val),
        Err(_) => Err("A entrada fornecida não é um número válido"),
    }
}


fn main() {
    loop {
        println!("Insira o número positivo para calcular o fatorial: ");
        let mut number: String = String::new();
        io::stdin().read_line(&mut number).expect("Erro ao ler a linha");
        match convert_to_int(number) {
            Ok(val) => {
                if val <= 0 {
                    println!("Número inválido");
                } else {
                    let mut result: i32 = 1;
                    for i in 1..val+1 {
                        result *= i;
                    }
                    println!("Fatorial de {} é: {}", val, result);
                    break;
                }
            },
            Err(err) => {
                println!("Erro: {}", err);
                continue;
            },
        }
    }
}