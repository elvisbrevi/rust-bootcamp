struct Tweet {
    content: String,
}

impl Tweet {
    fn replace_content(&mut self, content: &str) -> &str {}
}

fn main() {
    let tweet = Tweet {
        content: "example".to_owned(),
    };
}
