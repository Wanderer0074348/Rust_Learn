fn main() {
    // Definition
    let x:i8 = 10;
    println!("{x}");
    // Mutability
    let mut y:i16 = 1500;
    println!("{y}");
    y = 41;
    println!("{y}");

    //scope
    {
        let z:i32 = 5600000;
        println!("{z}");
    }

    //shadowing
    let t = 10;
    println!("{t}");
    let t = t+10;
    println!("{t}");
}
