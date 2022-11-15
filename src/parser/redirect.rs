#[derive(Debug, Clone)]
pub struct Redirect {
    path: String,
    is_over: bool,
}

impl Redirect {
    pub fn new(path: &str, is_over: bool) -> Self {
        Self {
            path: path.to_string(),
            is_over,
        }
    }
}
