
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let message = &args[1];
    let key = &args[2];
    let final_message = xor(message, key);

    println!("{:?}", final_message);

}

fn xor(input_string: &str, xor_key: &str) -> String{

    let length = input_string.len();
    let mut altered_string = "".to_string();
    let mut xor_operation;

    for i in 0..length {
        xor_operation = ((input_string.as_bytes()[i] as u8) ^ (xor_key.as_bytes()[i % (xor_key.len())] as u8)) as char;
        altered_string += &xor_operation.to_string();
    }

    return altered_string

}