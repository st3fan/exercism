pub struct CircularBuffer<T: PartialEq> {
    capacity: usize,
    elements: Vec<Option<T>>,
    read_index: usize,
    write_index: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: PartialEq> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut elements = Vec::new();
        elements.resize_with(capacity, Default::default);
        Self {
            capacity,
            elements,
            read_index: 0,
            write_index: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.elements[self.write_index] != Option::None {
            return Err(Error::FullBuffer);
        }

        self.elements[self.write_index] = Some(element);
        self.write_index = (self.write_index + 1) % self.capacity;

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if let Some(v) = self.elements[self.read_index].take() {
            self.read_index = (self.read_index + 1) % self.capacity;
            return Ok(v);
        }
        return Err(Error::EmptyBuffer);
    }

    pub fn clear(&mut self) {
        self.read_index = 0;
        self.write_index = 0;
        // Can't use fill() because T then needs to be Clone'able
        for i in 0..self.capacity {
            self.elements[i] = Option::None;
        }
    }

    pub fn overwrite(&mut self, element: T) {
        if self.elements[self.write_index] == Option::None {
            self.elements[self.write_index] = Some(element);
            self.write_index = (self.write_index + 1) % self.capacity;
        } else {
            self.elements[self.read_index] = Some(element);
            self.read_index = (self.read_index + 1) % self.capacity;
            self.write_index = self.read_index;
        }
    }
}
