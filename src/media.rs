use std::io;

fn convert_to_int(input: String) -> i32 {
    let input: i32 = input.trim().parse::<i32>().unwrap();
    return input;
}

fn main() {
    let mut input_corte: String = String::new();
    let mut prova1: String = String::new();
    let mut prova2: String = String::new();
    let mut prova3: String = String::new();
    println!("Digite a nota da prova 1: ");
    io::stdin().read_line( &mut prova1).expect("error");
    println!("Digite a nota da prova 2: ");
    io::stdin().read_line( &mut prova2).expect("error");
    println!("Digite a nota da prova 3: ");
    io::stdin().read_line( &mut prova3).expect("error");
    println!("Qual a nota de corte?");
    io::stdin().read_line( &mut input_corte).expect("error");
    let media: i32 = (convert_to_int(prova1) + convert_to_int(prova2) + convert_to_int(prova3)) / 3;    
    if media >= convert_to_int(input_corte) {
        println!("Aprovado com média: {}", media);
    } else {
        println!("Reprovado com média: {}", media);
    }

}