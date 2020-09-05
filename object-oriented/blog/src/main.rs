extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a whole chicken for lunch today!");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a whole chicken for lunch today!", post.get_content());
}
