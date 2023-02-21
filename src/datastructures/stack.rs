pub struct Stack<T> {
    stack: Vec<T>
}

impl Stack<T> {
    // init new
    fn new() -> Self {
        Stack {stack: Vec::new()}
    }

    // pop the last item off the end of the stack
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    // push an item to the front of the stack
    fn push(&mut self, item: T) -> Self {
        self.stack.push(item)
    }

    fn is_empty(&mut self) -> bool {
        self.stack.is_empty()
    }

    fn length(&mut self) -> usize {
        self.stack.length()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}