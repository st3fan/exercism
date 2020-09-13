class BufferFullException(Exception):
    pass


class BufferEmptyException(Exception):
    pass


class CircularBuffer:
    def __init__(self, capacity):
        self.capacity = capacity
        self.clear()

    def read(self):
        if self.length == 0:
            raise BufferEmptyException("buffer is empty")
        value = self.data[self.read_index]
        self.read_index = (self.read_index + 1) % self.capacity
        self.length -= 1
        return value

    def write(self, value):
        if self.length == self.capacity:
            raise BufferFullException("buffer is full")
        self.data[self.write_index] = value
        self.length += 1
        self.write_index = (self.write_index + 1) % self.capacity

    def overwrite(self, value):
        if self.length == self.capacity:
            self.data[self.write_index] = value
            self.write_index = (self.write_index + 1) % self.capacity
            self.read_index = (self.read_index + 1) % self.capacity
        else:
            self.write(value)
            
    def clear(self):
        self.data = [None] * self.capacity
        self.read_index = 0
        self.write_index = 0
        self.length = 0
