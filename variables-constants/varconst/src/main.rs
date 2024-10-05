fn main() {
    
    let mut number: u32 = 16;
    println!("Number is {}", number);

    // number = 18; // error without mut keyword for variable
    // println!("Number is {}", number);

    number = 18; // completely fine with mut keyword for variable
    println!("Number is {}", number);

    // good practice is to use const name in upper_case & mention type
    const DAYS_IN_A_WEEK: u8 = 7; 
    println!("Days in a week : {}", DAYS_IN_A_WEEK);

    println!("Two hours in seconds : {}", TWO_HOURS_IN_SECONDS);

}

// const can be declared outside main function 
const TWO_HOURS_IN_SECONDS: u16 = 2 * 60 * 60;
