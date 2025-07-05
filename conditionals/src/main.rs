fn main() {
    let mut age : String = String::new();
    println!("Write your age");
    std::io::stdin().read_line(&mut age).unwrap();
    let age: u8 = age.trim().parse().expect("Please enter a valid number");
    // Conditional if
    if age >= 18 {
        println!("You are adult");
    }else{
        println!("You are not adult!");
    }
}
