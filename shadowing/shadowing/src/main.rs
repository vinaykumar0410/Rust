fn main() {
    
    let number = 4;

    // here number is shadowing 
    let number = number + 1; 

    println!("Number is {}", number); // 5

    // number = number + 1;
    // println!("Number is {}", number); // error as variable in rust is immutable

    {
        // here number is shadowing but only for this block
        let number = number * 10; 
        println!("Number is {} inside block", number); // 50
    }

    // last shadowing is limited to that block only
    println!("Number is {} outside block", number); // 5
    
}
