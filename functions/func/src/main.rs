fn main() {
    
    say_hello();

    greet_user("Vinay");

    user_details("Krishna", 24, 164.6);

    let number: u32 = 36;
    let plus_ten: u32 = add_ten(number);
    println!("Sum of {} + 10 is {}", number, plus_ten);

    let num1: i32 = -54;
    let num2: i32 = 26;
    
    println!("{} is positive number : {}", num1, is_positive(num1));
    println!("{} is positive number : {}", num2, is_positive(num2));

}

fn say_hello(){
    println!("Hello 👀")
}

fn greet_user(name: &str){
    println!("Hello {}", name);
}

fn user_details(name: &str, age: u32, height: f64){
    println!("Name : {}\nAge : {}\nHeight : {}", name, age, height);
}

fn add_ten(number: u32) -> u32{
    return number + 10;
}

fn is_positive(number: i32) -> bool {
    if number > 0 {
        return true;
    }
    return false;
}
