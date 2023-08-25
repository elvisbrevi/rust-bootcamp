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

    // Ownership
    let s1 = String::from("hello"); // heap allocated string value
                                    // let s2 = s1; // s1 is moved to s2
                                    // print!("s2 is: {}", s2); // s1 is dropped before main function ends

    function_ownwership(s1); // s1 is moved to function
                             //print!("s2 is: {}", s1);

    // cloned by default for primitive types (copy trait)
    let x = 5;
    let y = x;
    println!("y is: {}", x);

    // solution: clone non-primitive types
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s2 is: {}", s2);

    // solution: reference
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s2 is: {}", s2);

    let mut y = 30;
    let x = &mut y;
    println!("x and y: {}", x);
    *x = 40;
    println!("y and y: {}", x);

    // slices
    let tweet = String::from("This is a tweet message");
    let trimmed_tweet = &tweet[..10]; // string slice
    println!("trimmed tweet: {}", trimmed_tweet);
    println!("trimmed tweet: {}", trim_tweet(&tweet));

    let tweet2 = "other tweet but literal string";
    println!("trimmed tweet: {}", trim_tweet(&tweet2));

    let a = [1, 2, 3, 4, 5];
    let a_slice: &[i32] = &a[1..3];
    println!("a slice: {:?}", a_slice);
}

fn function_ownwership(s: String) {
    println!("s is: {}", s);
} // s is dropped here

fn my_function(x: i32) -> i32 {
    println!("my function called with: {}", x);
    x + 22
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..10]
}
