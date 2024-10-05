fn main() {
    
    // owner of ExampleText is s1
    let s1 = String::from("ExampleText");
    // Here passing reference of s1 to calculate_length
    let len = calculate_length(&s1);

    println!("Length of {} is {}", s1, len);

    // now the owner of ExampleText is s2
    let s2 = s1;
    // println!("{}",s1); // error as s1 is no longer owned
    println!("{}",s2);
}


fn calculate_length(s: &String) -> usize {
    s.len()
}

