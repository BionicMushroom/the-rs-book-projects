pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
    needed_approvals: u32,
}

pub enum ApproveResult {
    Pending(PendingReviewPost),
    Approved(Post),
}

impl Post {
    #[must_use]
    pub fn new_draft() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    #[must_use]
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost::new(self.content)
    }
}

impl PendingReviewPost {
    #[must_use]
    pub fn approve(self) -> ApproveResult {
        let mut mut_self = self;
        mut_self.needed_approvals -= 1;

        if mut_self.needed_approvals == 0 {
            ApproveResult::Approved(Post {
                content: mut_self.content,
            })
        } else {
            ApproveResult::Pending(mut_self)
        }
    }

    #[must_use]
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    fn new(content: String) -> PendingReviewPost {
        PendingReviewPost {
            content,
            needed_approvals: 2,
        }
    }
}
