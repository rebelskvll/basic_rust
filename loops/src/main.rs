fn main() {
    let mut counter = 1;

    loop {
        println!("Counter: {counter}");

        if counter == 5 {
            break; // Break the loop when counter equals 5
        }

        counter += 1;
    }

    println!("Finished");
}
