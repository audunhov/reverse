use std::env;

fn main() {
    for argument in env::args().skip(1) {
        let reversed = reverse(argument);
        println!("{}", reversed);
    }
}

fn reverse(string: String) -> String {
    let reverse = string.chars().rev().collect();
    return reverse;
}
