#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 5, y: 6 };

    print!("{:?}", p1); // error because dont implement std::fmt::Display trait by default
    print!("{}", p1 == p2);
    print!("{}", p1 == p3);
}
