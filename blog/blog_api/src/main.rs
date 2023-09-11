use blog_shared::Post;

fn main() {
    let post = Post::new(
        "My first post".to_owned(),
        "This is my first blog post".to_owned(),
    );

    println!("{:?}", post);
}
