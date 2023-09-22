const MAX_LEVEL: u32 = 100_000;

fn main() {
    let mut num = 22;
    num = 20;

    let mut x = 10;
    {
        let mut x = 20;
        print!("x = {}", x);
        x = 30;
        print!("x = {}", x);
    }
    print!("x = {}", x);

    let s1 = String::from("hello");
    let s2 = s1;
    print!("s2 = {}", s2);

    let s3 = String::from("hello");
    print(s3.clone());
    print!("s3 = {}", s3);

    let mut r1 = String::from("hello");
    let r2 = &r1;
    let r3 = &mut r1;
    print!("r1 = {}", r3);
}

fn print(s: String) {
    print!("{}", s);
}
