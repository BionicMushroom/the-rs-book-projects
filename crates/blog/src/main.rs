use blog::{ApproveResult, Post};

fn main() {
    let mut post = Post::new_draft();
    post.add_text("I ate a salad for lunch today.");

    let post = post.request_review();

    let mut post = post.reject();
    post.add_text(" It was good.");

    let post = post.request_review();

    let ApproveResult::Pending(post) = post.approve() else {
        unreachable!("We need two approvals for the post to be considered approved.")
    };

    let ApproveResult::Approved(post) = post.approve() else {
        unreachable!("The post didn't get approved after two approvals.");
    };

    assert_eq!(
        "I ate a salad for lunch today. It was good.",
        post.content()
    );
}
