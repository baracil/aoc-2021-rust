pub struct Stack<T> {
    values:Vec<T>,
}

impl <T> Stack<T> {

    pub fn new(capacity:usize) -> Self {
        let mut values = Vec::new();
        values.reserve(capacity);

        Stack{values}
    }


    pub fn push(&mut self, value:T) where T:Clone {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.values.len();
        if len == 0 {
            None
        } else {
            Some(self.values.remove(len - 1))
        }
    }
}