fn main() {
    
    // int, float, bool, char

    // i8 i16 i32 i64 (can assign both positives & negatives to signed integers)
    let negativeNumber : i32 = -47;
    println!("negativeNumber:{}", negativeNumber);

    // u8 u16 u32 u64 (can't assign negatives to unsigned integers)
    // let anotherNegativeNumber : u32 = -72;
    // println!("anotherNegativeNumber:{}", anotherNegativeNumber); // error

    let positiveNumber : u32 = 86;
    println!("positiveNumber:{}", positiveNumber);

    // f32 f64 (floating values )
    let floatNumber : f32 = 3.14;
    println!("floatNumber:{}", floatNumber);

    // bool
    let boolValue : bool = true;
    println!("boolValue:{}", boolValue);

    // char
    let charValue : char = 'A';
    println!("charValue:{}", charValue);

}
