use itertools::Itertools;
use crate::AOCResult;
use crate::days::day_24::block::Block;
use crate::days::day_24::stack::Stack;

pub struct Program {
    blocks:Vec<Block>,
}

impl Program {

    #[allow(dead_code)]
    pub(crate) fn nb_blocks(&self) -> usize {
        self.blocks.len()
    }

    #[allow(dead_code)]
    pub(crate) fn blocks(&self) -> impl Iterator<Item=&Block> {
        self.blocks.iter()
    }

    pub fn block_at(&self, index:usize) -> Option<&Block> {
        self.blocks.get(index)
    }

}


impl Program {

    #[allow(dead_code)]
    pub fn evaluate(&self, input:&[u32;14]) -> String {
        let mut stack = Stack::default();
        for (i,block) in self.blocks.iter().enumerate() {
            block.handle_digit(&mut stack, input[i])
        };
        stack.to_string()
    }

    pub fn parse(lines: &[String]) -> AOCResult<Program> {
        let blocks:AOCResult<Vec<Block>> = lines.iter()
            .enumerate()
            .filter(|(_, l)| *l == "inp w")
            .map(|(idx, _)| idx)
            .chain(std::iter::once(lines.len()))
            .tuple_windows::<(_, _)>()
            .map(|(s, e)| &lines[s..e])
            .map(Block::parse)
            .collect();
        Ok(Program { blocks:blocks? })
    }

}