fn main() {
    // Declare a mutable variable type String
    let mut name : String = String::new(); 
    let mut age : String = String::new();
    // Print on screen
    println!("Write your name: ");
    // Read from input user and save in a variable
    std::io::stdin().read_line(&mut name).unwrap();
    println!("Write your age: ");
    std::io::stdin().read_line(&mut age).unwrap();
    // Remove newline
    let name = name.trim();
    // Remove newline, convert to int
    let age: u8 = age.trim().parse().expect("Please enter a valid number");
    //Print with placeholder
    println!("Welcome {name}, from {age} years old");
}
