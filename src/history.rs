pub struct History {
    size: usize,
    index: usize,
    history: Vec<String>,
}

impl History {
    pub fn new(size: usize) -> History {
        History {
            size,
            index: 0,
            history: Vec::new()
        }
    }

    pub fn add(&mut self, word: String) {
        if self.history.len() == self.size {
            self.history.remove(0);
        }

        self.history.push(word);
    }

    pub fn get(&self) -> Option<&String> {
        if self.history.len() != 0 {
            Some(&self.history[self.index])
        } else {
            None
        }
    }

    pub fn back(&mut self) -> Option<&String> {
        if self.index != 0 {
            self.index = self.index - 1;
        }

        self.get()
    }

    pub fn next(&mut self) -> Option<&String> {
        if self.history.len() != 0 && self.index + 1 < self.history.len() {
            self.index = self.index + 1;
        }

        self.get()
    }

    pub fn reset_index(&mut self) {
        if self.history.len() != 0 {
            self.index = self.history.len() - 1;
        } else {
            self.index = 0;
        }
    }
}