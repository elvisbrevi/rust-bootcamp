struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand {
            name: name,
            payload,
        }
    }
}

fn main() {
    let cmd1 = BrowserCommand {
        name: "navigate".to_string(), // or String::from("navigate"),
        payload: String::from("https://www.rust-lang.org"),
    };

    let cmd2 = BrowserCommand {
        name: "zoom".to_string(),
        payload: 200,
    };
}
