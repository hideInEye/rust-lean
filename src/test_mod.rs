#[derive(Debug)]
pub struct Rectangle {
    height: usize,
    width: usize,
}

 impl Rectangle {
    pub fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_height(&mut self, height: usize) -> usize {
        self.height = height;
        self.height()
    }

    pub fn parts(self) -> (usize, usize) {
        (self.height, self.width)
    }
}
