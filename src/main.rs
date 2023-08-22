const MAX_PLAYERS: u8 = 10;
static CASINO_NAME: &str = "Casino Royale";

fn main() {
    /* VARIABLES */
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

    // if/else
    let x = 5;

    if x > 5 {
        println!("x is bigger than 5");
    } else if x > 3 {
        println!("x is not bigger than 5 but bigger than 3");
    } else {
        println!("x is not 5 and not 3");
    }

    let b = if x > 5 { 10 } else { 20 };
    println!("b is: {}", b);

    // loop
    'outer: loop {
        loop {
            println!("looping a");
            break 'outer;
        }
        println!("looping b");
        break;
    }

    let x = loop {
        break 5;
    };
    print!("x is: {}", x);

    // while
    let mut a = 0;
    while a < 5 {
        println!("a is: {}", a);
        a = a + 1;
    }

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}

fn my_function(x: i32) -> i32 {
    println!("my function called with: {}", x);
    x + 22
}
