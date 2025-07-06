// Structure of a function
// fn name
// (params to recive:type of data)
// "->" the type of value to return
fn add_number(my_number:i32) -> i32 {
    let final_number = my_number * 2;
    return final_number;
}

fn main() {
    let number = add_number(10);
    println!("{number}");
}
