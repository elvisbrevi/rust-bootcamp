const MAX_PLAYERS: u8 = 10;
static CASINO_NAME: &str = "Casino Royale";

fn main() {
    /* CONSTANTS */
    // two space of memory
    let a: u8 = MAX_PLAYERS;
    let a: u8 = MAX_PLAYERS;
    // one space of memory
    let b: &str = CASINO_NAME;
    let b: &str = CASINO_NAME;

    /* FUNCTIONS */
    let my_value = my_function(22);
    println!("my function returned: {}", my_value);
    //variables();
}

fn my_function(x: i32) -> i32 {
    println!("my function called with: {}", x);
    x + 22
}

// function variables
fn variables() {
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
