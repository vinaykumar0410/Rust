fn main() {
    
    let age: u32 = 17;
    let is_eligible: bool = is_eligible(age);

    if is_eligible {
        println!("Age {} is eligible", age);
    } else {
        println!("Age {} is not eligible", age);
    }

    let max_number: i32 = max_of_three_numbers(14,24,18);
    println!("Maximum number is {}",max_number);

    let is_sunday = true;
    let free_time = if is_sunday {"10h"} else {"4h"};

    println!("Free Time is {}", free_time);

}

fn is_eligible(age: u32) -> bool {
    age >= 18
}

fn max_of_three_numbers(num1: i32, num2: i32, num3: i32) -> i32 {

    if num1 > num2 && num1 > num3 {
        num1
    } else if num2 > num3 {
        num2
    } else {
        num3
    }
}