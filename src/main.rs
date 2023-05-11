use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    string: String,
}

fn main() {
    let args = Args::parse();
    let reversed = reverse(args.string);
    println!("{}", reversed);
}

fn reverse(string: String) -> String {
    let reverse = string.chars().rev().collect();
    return reverse;
}

#[cfg(test)]
mod tests {
    use crate::reverse;

    #[test]
    fn ascii() {
        let case = "hello world".to_owned();
        assert_eq!(reverse(case), "dlrow olleh");
    }

    #[test]
    fn utf() {
        let case = "piña".to_owned();
        assert_eq!(reverse(case), "añip");
    }
}
