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

    // Strings
    let s1 = "Hola Mundo";
    let s2 = String::from("hello world");
    let s3 = "Hola Hola ".to_string();
    let s4 = "Hola Hola ".to_owned();
    let s5 = &s4[0..2];
    println!("s5 is: {}", s5);

    // Manipulating strings
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is: {}", s);
    s.replace_range(0..2, "replace_with");
    println!("s is: {}", s);

    // string concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used
    println!("s3 is: {}", s3);

    // string formatting
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    // less efficient than concatenation, because it allocates new memory (s4) and copies the values into it. but more readable
    println!("s4 is: {}", s4);

    // Concats
    let a1 = ["first", "second"].concat();
    println!("a1 is: {}", a1);
    let a2 = format!("{}-{}", "first", "second"); // more efficient than concat, because it doesn't allocate new memory
    println!("a2 is: {}", a2);
    let a3 = ["first", "second"].join("-"); // more efficient than concat, because it doesn't allocate new memory
    print!("a3 is: {}", a3);

    let s1 = "🦀🦀🦀🦀🦀";
    let s2 = &s1[0..4];
    println!("s2 is: {}", s2);

    for b in s1.bytes() {
        println!("b is: {}", b);
    }

    for b in s1.chars() {
        println!("b is: {}", b);
    }

    println!("s2 is: {}", s1.chars().count());

    let s1 = String::from("hello");
    let s2 = "hello";
    my_string_function(&s1);
    my_string_function(s2);

    /* STRUCTS */
    let mut book = Product::new(String::from("The Lord of the Rings"), 10.99, true);
    let price = book.price;
    book.in_stock = false;
    let s_tax = book.calculate_sales_tax();
    println!("sales tax is: {}", s_tax);
    book.set_price(1.0);
    book.buy();

    // Tuple Structs
    struct RGB(i32, i32, i32);
    struct CMYK(i32, i32, i32, i32);
    struct Point(i32, i32, i32);

    let rgb = RGB(0, 0, 0);
    let cmyk = CMYK(0, 0, 0, 0);
    let point = Point(0, 0, 0);

    // Unit-like Structs
    struct UnitLikeStruct;

    // Enums
    let cmd = Command::Undo;
    let cmd = Command::AddText(String::from("Hello"));
    let cmd = Command::MoveCursor(10, 20);
    let cmd = Command::Replace {
        from: ("hola").to_ascii_uppercase(),
        to: ("hello".to_lowercase()),
    };
    let json_string = cmd.serialize();
    println!("json string is: {}", json_string);

    // Match
    let age = 43;
    match age {
        1 => println!("age is 1"),
        13..=19 => println!("age is between 13 and 19"),
        x => println!("You are {} years old", x),
    }

    // Option
    let username = get_username(1);
    match username {
        Some(name) => println!("username is: {}", name),
        None => {}
    }
    // or with if let, which is more concise, but less flexible (ownership error)
    // if let Some(name) = username {
    //     println!("username is: {}", name);
    // }

    // Vector
    let v1: Vec<String> = Vec::new();
    let mut v2 = Vec::new();
    v2.push(String::from("Hello"));
    v2.push(String::from("World"));

    let v3 = vec![1, 2, 3, 4, 5];
    let ele2 = v3[0]; // can panic, but not in this case, because it's a primitive type

    //let ele1 = &v2[0]; // can panic, because it's a non-primitive type
    //let ele1 = v2.remove(0); // safe way to move the value out of the vector, but it's slower and all elements after the removed one have to be moved
    let ele1 = v2.get(0); // returns an Option<&T>, so it's safe, but slower

    if let Some(e) = ele1 {
        println!("ele is: {}", e);
    }

    // iterating over a vector
    for s in &mut v2 {
        s.push_str("!"); // s is a mutable reference
    }

    for s in &v2 {
        println!("s is: {}", s);
    }

    let mut v4 = vec![];
    for s in v2 {
        v4.push(s);
    }

    // v2 ownership is moved to v4, so v2 is no longer available
    //let i = v2.get(0);
}

fn get_username(user_id: u32) -> Option<String> {
    let query = format!("SELECT name FROM users WHERE id = {}", user_id);
    let db_result = query_db(query);
    db_result.ok()
}

// Result
fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query is empty"))
    } else {
        Ok(String::from("John Doe"))
    }
}

struct Product {
    name: String,
    price: f32,
    in_stock: bool,
    category: Category,
}

// Enums
enum Category {
    Books,
    Clothes,
    Electronics,
}

enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        let json_string = match self {
            Command::Undo => String::from("{ \"cmd\": \"undo\" }"),
            Command::Redo => String::from("{ \"cmd\": \"redo\" }"),
            Command::AddText(text) => {
                format!(
                    "{{
                        \"cmd\": \"add_text\", \
                        \"text\": {text}
                    }}"
                )
            }
            Command::MoveCursor(x, y) => {
                format!(
                    "{{
                        \"cmd\": \"move_cursor\", \
                        \"x\": {x}, \
                        \"y\": {y}
                    }}"
                )
            }
            Command::Replace { from, to } => {
                format!(
                    "{{ 
                        \"cmd\": \"replace\", \
                        \"from\": {from}, \
                        \"to\": {to}
                    }}"
                )
            }
        };
        json_string
    }
}

impl Product {
    fn new(name: String, price: f32, in_stock: bool) -> Product {
        Product {
            name,
            price,
            in_stock,
            category: Category::Books,
        }
    }

    fn get_default_sales_tax() -> f32 {
        0.1
    }

    fn calculate_sales_tax(&self) -> f32 {
        self.price * Product::get_default_sales_tax()
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bought");
        123
    }
}

fn my_string_function(a: &str) -> String {
    return format!("{}-{}", a, "other string");
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
