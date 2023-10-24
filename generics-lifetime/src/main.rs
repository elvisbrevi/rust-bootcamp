use rand;

fn main() {
    let player1 = String::from("player 1");
    let result: &str;
    {
        let player2 = String::from("player 2");
        result = first_turn_static(&player1, &player2);
    }

    println!("the player going first is {}", result);
    let p3 = result;
    println!("{}", p3);
    println!("{}", result);

    let player3 = String::from("player 3");
    let result2 = first_turn(&player1, &player3);
    println!("{}", result2);
    println!("{}", result2);
}

fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}

fn first_turn_static<'a>(p1: &'a str, p2: &'a str) -> &'static str {
    let s1 = "player 1 static";
    s1
}
