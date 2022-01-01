use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Write};

use itertools::Itertools;

use crate::days::day_24::block::Block;
use crate::days::day_24::program::Program;
use crate::days::day_24::stack::Stack;

struct Finder<'a> {
    program: &'a Program,
    digits: [u32; 14],
    block_idx: usize,
    max_stack_size: [usize; 14],
    reversed:bool,
}

impl Display for Finder<'_> {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.block_idx {
            f.write_char(char::from_digit(self.digits[i],10).unwrap_or('?'))?
        }
        Ok(())
    }
}

impl<'a> Finder<'a> {

    pub fn find(&mut self, stack:&Stack) -> Option<String> {
        let block = self.program.block_at(self.block_idx);

        match block {
            None => self.handle_all_digits_set(stack),
            Some(block) => {
                let max_stack_size = self.max_stack_size[self.block_idx];
                match (block,stack.size().cmp(&max_stack_size)) {
                    (_,Ordering::Less) => self.find_test_all_digits(stack,block),
                    (_,Ordering::Greater) => self.handle_stack_size_to_large(),
                    (_,Ordering::Equal) => self.find_for_forced_digit(stack, block),

                }
            }
        }
    }

    fn handle_stack_size_to_large(&self) -> Option<String> {
        None
    }

    fn handle_all_digits_set(&self, stack:&Stack) -> Option<String> {
        if stack.size() == 1 {
            Some(self.digits.iter().map(|d| char::from_digit(*d, 10).unwrap_or('?')).join(""))
        } else {
            None
        }
    }

    fn find_for_forced_digit(&mut self, stack:&Stack, block:&Block) -> Option<String> {
        if let Some(digit) = block.find_forced_digit(stack) {
            let mut buffer = stack.clone();
            self.find_for_one_digit(&mut buffer,block,digit)
        } else {
            None
        }
    }

    fn find_test_all_digits(&mut self, stack:&Stack, block:&Block) -> Option<String> {
        let mut buffer = Stack::default();

        for index in 1u32..=9 {
            let digit = if self.reversed {
                10 - index
            } else {
                index
            };

            buffer.set_to(stack);

            let result = self.find_for_one_digit(&mut buffer, block, digit);

            if result.is_some() {
                return result;
            }
        }
        None
    }


    fn find_for_one_digit(&mut self, stack:&mut Stack, block:&Block, digit:u32) -> Option<String> {
        self.digits[self.block_idx] = digit;

        block.handle_digit(stack,digit);

        if stack.is_invalid() || stack.size() > self.max_stack_size[self.block_idx]{
            return None
        }

        self.block_idx+=1;
        let result = self.find(stack);
        self.block_idx-=1;

        result
    }
}

impl<'a> Finder<'a> {

    fn new(program: &'a Program, reversed:bool) -> Self {
        let mut max_stack_size = [0; 14];

        let mut cumulate = 1;
        for i in (0..14).rev() {
            if matches!(program.block_at(i), Some(Block::ReplaceOrPop(_,_))) {
                cumulate += 1;
            }
            max_stack_size[i] = cumulate
        }

        Finder { program, digits:[0;14], block_idx: 0, max_stack_size, reversed }
    }


}


pub fn find_largest_model_number(program: &Program) -> Option<String> {
    let mut finder = Finder::new(program,true);

    let stack = Stack::default();
    finder.find(&stack)
}

pub fn find_lowest_model_number(program: &Program) -> Option<String> {
    let mut finder = Finder::new(program,false);

    let stack = Stack::default();
    finder.find(&stack)
}