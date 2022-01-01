use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Stack {
    values:[u32;15],
    size:usize,
    invalid:bool,
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 1..self.size {
            f.write_fmt(format_args!("{},",self.values[i]))?;
        }
        Ok(())
    }
}

impl Stack {
    pub(crate) fn is_invalid(&self) -> bool {
        self.invalid
    }

    pub fn current_value(&self) -> u32 {
        self.values[self.size-1]
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Stack {
    pub fn set_to(&mut self, other:&Stack) {
        for i in 0..other.size {
            self.values[i] = other.values[i]
        }
        self.size = other.size;
        self.invalid = other.invalid;
    }


    pub fn push(&mut self, value:u32) {
        self.values[self.size] = value;
        self.size += 1;
//        println!("Push    {:#2}  [{:#2}] ({})",value,self.size,self.current_value());
    }

    pub fn replace(&mut self, value:u32) {
        self.values[self.size-1] = value;
//        println!("Replace {:#2}  [{:#2}] ({})",value,self.size, self.current_value());
    }

    pub fn pop(&mut self) {
        if self.size == 0 {
            self.invalid = true
        } else {
            self.size -= 1;
 //           println!("Pop         [{:#2}] ({})",self.size,self.current_value());
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Stack{values:[0;15],size:1,invalid:false}
    }
}

