use blog_shared::Post;

fn main() {
    let post = Post::new("title".to_owned()
    , "body".to_owned());

    println!("blog_api {post:?}");
}
