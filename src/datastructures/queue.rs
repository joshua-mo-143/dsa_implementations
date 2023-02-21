pub struct Queue<T> {
    queue: VecDeque<T>
}

impl Queue<T> {
    fn new() -> Self {
        Self {queue: VecDeque::new()}
    }

    fn push(&mut self, item: T) -> Self {
        self.queue.push_back(item)
    }

    fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    fn front(&self) -> Option<&T> {
        self.queue.front()
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn back(&self) -> Option<&T> {
        self.queue.get(self.length - 1)
    }
}