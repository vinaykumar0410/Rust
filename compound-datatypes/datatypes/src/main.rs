fn main() {
    
    // * Arrays (must be homogenius elements)

    // let numbers = [1, 2, 3, 4, 5];
    let numbers: [i32; 5] = [4, 5, 6, 7, 8];
    println!("Numbers : {:?}", numbers);

    // let heights = [156.6, 162.4, 148.2, 152.8];
    let heights: [f32; 4] = [156.6, 162.4, 148.2, 152.8];
    println!("Heights : {:?}", heights);

    // ! let mixture = ["Vinay", 23, true];
    // ! println!("Mixture : {:?}", mixture); // error (must be homogenius)

    // * Tuples (can be heterogenius elements)

    // let mixture = ("Vinay", 23, true, [1, 2, 3]);
    let mixture: (String, i32, bool) = ("Vinay".to_string(), 23, true);
    println!("Mixture : {:?}", mixture);

    // * Slices 

    let ages: &[i32] = &[17, 21, 16, 18, 19];
    println!("Ages : {:?}", ages);

    let friends: &[&str] = &["Sai", "Ram", "Krishna"];
    println!("Friends : {:?}", friends);

    let languages: &[&String] = &[&"Rust".to_string(), &"Go".to_string(), &"Java".to_string()];
    println!("Languages : {:?}", languages);

    // * String Slice 

    let greeting: String = String::from("Hello Bro!");
    let greet: &str = &greeting[0..5];
    println!("{} Welcome to Rust!", greet);

}
