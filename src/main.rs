use std::io;
fn main() {
    let variable1 = "Rust";
    let variable2 = "No";
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input = input.trim(); 
    match_input(input, variable1, variable2);

}

fn match_input(input: &str, var1: &str, var2: &str) {
    match input {
        var if var == var1 => println!("So you appreciate Rust? That's great! Thank you!"),
        var if var == var2 => println!("So you like nothing? Alright then... :)"),
        _ => println!("It seems that you like {}.", input),
    }
}
