use std::io;


fn convert_to_int(input: String) -> i32 {
    let input: i32 = input.trim().parse::<i32>().unwrap();
    return input;
}

fn main() {
    let mut soma: i32 = 0;
    let mut number: String = String::new();
    io::stdin().read_line( &mut number).expect("error");
    for (i, char) in number.chars().enumerate() {
        if i < number.len() -1 {
            soma += convert_to_int(char.to_string());
        }
    }
    println!("Soma: {}", soma);
}
