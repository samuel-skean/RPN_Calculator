pub struct Lexer<T> {
    // TODO: Consider making it so that the lexer holds a window into itself (an
    // &str), that starts at what is currently s.start, instead of anything
    // else. It would be great if this could somehow hold both a String and an
    // immutable reference into that string, so that it didn't have a lifetime
    // parameter. This would even be almost poetically pleasing - mirroring how
    // the channels had to be hidden from the interface to the lexer in Go.
    // Also, I think it would make debugging way easier. Maybe there's a way to
    // just do this in-place to a string?
    input: String,
    start: usize,
    pos: usize,
    items: Vec<Item<T>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Item<T> {
    pub token: T,
    pub pos: usize,
}

impl<T> Lexer<T> {
    pub fn new(input: String) -> Lexer<T> {
        Lexer {
            input,
            start: 0,
            pos: 0,
            items: Vec::new(),
        }
    }

    pub fn peek(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Ignore the current window of the input. It's not a token.
    pub fn ignore(&mut self) {
        self.start = self.pos;
    }

    pub fn emit(&mut self, token: T) {
        self.items.push(Item {
            token,
            pos: self.start,
        });
        self.start = self.pos;
    }

    pub fn accept_any(&mut self, chars_set: &str) -> bool {
        if self.end() {
            return false;
        }
        let next = self.peek();
        if chars_set.contains(next) {
            self.pos += next.len_utf8();
            true
        } else {
            false
        }
    }

    pub fn accept_run(&mut self, chars_set: &str) {
        while self.accept_any(chars_set) {}
    }

    pub fn accept_prefix(&mut self, prefix: &str) -> bool {
        if self.input[self.pos..].starts_with(prefix) {
            self.pos += prefix.len();
            return true;
        }
        false
    }

    pub fn accept_first_prefix(&mut self, prefix_set: &[&str]) -> bool {
        for prefix in prefix_set {
            if self.accept_prefix(prefix) {
                return true;
            }
        }
        return false;
    }

    pub fn current_match(&self) -> &str {
        &self.input[self.start..self.pos]
    }

    pub fn end(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn to_vec(self) -> Vec<Item<T>> {
        self.items
    }
}
