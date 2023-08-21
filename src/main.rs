fn main() {
    // creation
    let a: i16 = 5;
    // mutability
    let mut b: i16 = 5;
    b = 4;
    // shadowing
    let c: i16 = 110;
    let c: i16 = 130;

    println!("c is: {}", c);

    // scope
    let d: i16 = 5;
    {
        let d: i16 = d + 6;
        println!("d is: {}", d);
    }

    println!("d is: {}", d);
}
