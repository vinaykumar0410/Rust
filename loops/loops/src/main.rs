fn main() {
    
    loop{
        println!("Hello Rust");
        break // without break loop will be infinite
    }

    let mut counter = 1;

    // basic loop in rust
    let result = loop{
        counter += 1;
        if counter == 5 {
            break counter*2;
        }
    };

    println!("Counter is : {}", result);

    // loop that prints evens in range 3 to 17
    evens_in_range(3,17);

    let full_name: String = String::from("Virat Kohli");
    // get_first_name will return Virat
    let first_name: String = get_first_name(full_name);

    println!("\nFirst Name is {}", first_name);

}

fn evens_in_range(start: i32, end: i32){
    
    for number in start..end {
        if number % 2 == 0 {
            print!("{} ",number);
        }
    }
}

fn get_first_name(name: String) -> String {

    let mut first_name: String = String::from("");
    for ch in name.chars() {
        if ch == ' ' {
            break;
        }
        first_name.push(ch);
    }
    return first_name;
}
