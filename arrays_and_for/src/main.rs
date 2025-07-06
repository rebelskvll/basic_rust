fn main() {

    // Create a vector
    let mut names: Vec<String> = Vec::new();

    for i in 1..4 {
        let mut name = String::new();
        println!("Enter name #{i}:");
        std::io::stdin().read_line(&mut name).unwrap();
        name = name.trim().to_string();
        names.push(name);
    }

    // print the names one after the other
    for name in names {
        println!("Hello, {}!", name);
    }
}
