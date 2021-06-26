pub struct Node {
    inner: Inner,
}

struct Inner {}

impl Node {
    pub fn new() -> Self {
        Self { inner: Inner {} }
    }

    pub fn start(&self) {}

    pub fn stop(&self) {}
}