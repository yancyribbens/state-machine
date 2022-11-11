use state_machine::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    // Post can not be seen until it's been approved.
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

     // Reviewer rejected, so it goes back to Draft state.
    post.reject();
    assert_eq!("", post.content());

    // Ready for review again.
    post.request_review();

    // The content is still not visible.
    assert_eq!("", post.content());

    // Content can only be added while it's in the Draft state.
    post.add_text("I ate pizza for lunch today");

    // Two approvals are required to move it to published.
    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
