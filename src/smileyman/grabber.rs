pub struct Grabber {
    pub index: usize,
    pub chars: Vec<char>,
}

impl Grabber {
    pub get_tag_content(&self, tag: &str, content: &str) {
        
    }

    pub fn peek_range(&self, len: usize) -> String {
        self.chars[self.index .. self.index + len].to_owned()
    }
}