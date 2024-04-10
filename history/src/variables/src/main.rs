fn main() {
    let input = read_line();
    println!("You entered: {}", input);
}

fn double(x: i32) -> i32 {
    x * 2
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect(
        "Failed to read line");
    input.trim().to_string()
}
