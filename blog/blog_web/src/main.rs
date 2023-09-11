use blog_shared::Post;

fn main() {
    let post = Post::new(
        "My first post".to_owned(),
        "the content of the first post".to_owned(),
    );

    println!("{:?}", post);
}
