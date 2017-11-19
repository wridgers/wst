pub struct Buffer {
    buffer: Vec<String>,
    header: usize,
    length: usize,
}

impl Buffer {
    pub fn new(header: usize, length: usize) -> Buffer {
        Buffer {
            buffer: Vec::new(),
            header: header,
            length: length,
        }
    }

    pub fn add_line(&mut self, line: String) {
        if self.header > self.buffer.len() || self.length > 0 {
            self.buffer.push(line);
        }

        if self.length > 0 {
            if self.buffer.len() > self.header + self.length {
                self.buffer.remove(self.header);
            }
        }
    }

    pub fn get_buffer(&mut self) -> &Vec<String> {
        &self.buffer
    }
}
