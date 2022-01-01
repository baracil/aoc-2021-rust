use std::fmt::{Display, Formatter};
use crate::{AOCResult, parse_input};
use crate::days::day_24::block::Block::{PushOrNope, ReplaceOrPop};
use crate::days::day_24::stack::Stack;

pub enum Block {
    PushOrNope(i32,u32),
    ReplaceOrPop(i32,u32),
}

impl Block {
    pub(crate) fn find_forced_digit(&self, stack:&Stack) -> Option<u32> {
        match self {
            PushOrNope(mod_offset, _) | ReplaceOrPop(mod_offset, _) => {
                let current = (stack.current_value() as i64) + (*mod_offset as i64);
                if current<=0 || current>9 {
                    None
                } else {
                    Some(current as u32)
                }
            }
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (type_name,mo,o) = match self {
            Block::PushOrNope(mo, o) => ("PushOrNopre",mo,o),
            Block::ReplaceOrPop(mo, o) => ("ReplaceOrPor",mo,o),
        };
        f.write_fmt(format_args!("z[i] = inp {: <+#3},  type:{:#10},  offset: {}", mo,type_name,o))
    }
}

impl Block {

    pub fn handle_digit(&self, stack:&mut Stack, digit:u32) {
        let condition_met = self.condition_met(stack,digit);
        match (self,condition_met) {
            (Block::PushOrNope(_, offset),true) => stack.push(digit+offset),
            (Block::PushOrNope(_, _),false) => {},
            (Block::ReplaceOrPop(_, offset),true) => stack.replace(digit+offset),
            (Block::ReplaceOrPop(_, _),false) => stack.pop(),
        }
    }

    fn condition_met(&self, stack:&Stack, digit:u32) -> bool {
        let prev = stack.current_value();
        match self {
            PushOrNope(test_offset, _) | ReplaceOrPop(test_offset, _) => {
                if *test_offset<0 {
                    prev != digit + ((-test_offset) as u32)
                } else {
                    prev + (*test_offset as u32) != digit
                }
            }
        }
    }

    pub fn parse(lines: &[String]) -> AOCResult<Self> {
        let mut acc = Acc::default();

        lines.iter().fold(&mut acc, |a, l| a.handle_line(l));

        acc.build_block()
    }
}

pub struct Acc {
    idx: i32,
    mod_offset:i32,
    offset:u32,
    div:u32,
}

impl Default for Acc {
    fn default() -> Self {
        Acc { idx: -1, mod_offset:0,offset:0,div:0 }
    }
}

impl Acc {
    fn handle_line(&mut self, line: &str) -> &mut Acc {
        if line == "mod x 26" {
            self.idx = 0
        }
        let arg = if self.idx > 0 {
            line.split(' ').nth(2).unwrap()
        } else {
            ""
        };

        match self.idx {
            1 => self.div = parse_input!(arg,u32),
            2 => self.mod_offset = parse_input!(arg,i32),
            12 => self.offset = parse_input!(arg,u32),
            _ => {}
        }

        if self.idx >= 0 {
            self.idx += 1;
        }

        self
    }

    fn build_block(self) -> AOCResult<Block> {
        match self.div {
            1 => Ok(PushOrNope(self.mod_offset, self.offset)),
            26 => Ok(ReplaceOrPop(self.mod_offset, self.offset)),
            d => Err(format!("Cannot create block with div {}",d))
        }
    }
}