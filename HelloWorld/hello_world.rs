fn main() {
    println!("Hello World");
    println!("Am I a rustacean?");
    println!(
        "{first} and {second} formatted print",
        second = "second",
        first = "First",
    );
    println!("Binary number 1023 = {:b}", 1023);
    println!("Test{number:>5}", number = 2345);
}
