// This library illustrates how to use a dyn types to create
// a state machine.  A dyn type can be used in place of a
// concreted or generic type.  Whenever a state transition happens
// we point to a different struct.  Each struct implements the
// trait State which is used to control the transitions.

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        let modifyable = self.state.as_ref().unwrap().modify_flag();

        if modifyable {
            self.content.push_str(text);
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn content(&self) -> &str {
        //as_ref is used since we want a refrence to the
        //value inside the option.
        //returns Option<&Box<dyn State>>
        //without this, we can't move `state` out of the borrowed &self
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn request_review(&mut self) {
        // take will _take_ ownership, leaving a None
        // value in place of Some(s)
        if let Some(s) = self.state.take() {
            // Since `state` is a dyn type, the new result
            // will depend on what struct box is pionting to via Box.
            // This struct can be anything that implements the `State` trait.
            self.state = Some(s.request_review());
        }
    }
}

// different traits determine the state transitions
trait State {
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn modify_flag(&self) -> bool {
        false
    }

    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn modify_flag(&self) -> bool {
        true
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approvals: 0 })
    }
}

struct PendingReview {
    approvals: u8,
}

impl State for PendingReview {
    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approvals = self.approvals + 1;

        if self.approvals >= 2 {
            Box::new(Published {})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Published {}

impl State for Published {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
