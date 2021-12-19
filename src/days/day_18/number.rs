use std::fmt::{Display, Formatter, Write};
use std::ops::Add;
use std::str::FromStr;
use crate::days::day_18::number::Node::{Literal, Pair};
use crate::parse_input;

#[derive(Debug,Clone)]
pub enum Node {
    Literal(usize),
    Pair
}

#[derive(Clone)]
pub struct Number {
    tree:Vec<Option<Node>>,
}

impl Number {
    fn insert(&mut self, my_pos: usize, source: &Number, its_pos:usize) {
        if my_pos >= self.tree.len() {
            return;
        }
        self.tree[my_pos] = source.tree[its_pos].to_owned();
        self.insert(left_child_index(my_pos),source, left_child_index(its_pos));
        self.insert(right_child_index(my_pos),source, right_child_index(its_pos));
    }

    pub fn magnitude(&self) -> usize {
        self.magnitude_rec(0)
    }

    fn magnitude_rec(&self,idx:usize) -> usize {
        match self.tree[idx] {
            Some(Literal(v)) => v,
            Some(Pair) => 3*self.magnitude_rec(left_child_index(idx))+2*self.magnitude_rec(right_child_index(idx)),
            None => 0usize
        }
    }


}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result:Number = Default::default();

        result.tree[0] = Some(Pair);
        result.insert(1,&self,0);
        result.insert(2,&rhs,0);

        result.reduce();
        result
    }
}

impl Default for Number {
    fn default() -> Self {
        Number{tree:vec![None;63]}
    }
}

impl Number {


    pub fn reduce(&mut self) {
        loop {
            loop {
                if !self.explode() {
                    break;
                }
            }
            if !self.split() {
                break
            }
        }
    }

    pub(crate) fn split(&mut self) -> bool {
        let to_split = self.find_index_to_split(0);

        if let Some(idx) = to_split {
            let value = self.get_value(idx);
            let left_value =value/2;
            let right_value = value - left_value;
            self.tree[idx] = Some(Pair);
            self.tree[left_child_index(idx)] = Some(Literal(left_value));
            self.tree[right_child_index(idx)] = Some(Literal(right_value));
        }

        to_split.is_some()
    }

    pub(crate) fn explode(&mut self) -> bool {
        let to_explode = self.find_node_to_explode();
        if let Some(idx) = to_explode {
            let parent = parent_index(idx);

            if let Some(first_left) = self.find_first_literal_on_left(parent) {
                let left_child_value = self.get_value(parent *2+1);
                self.add_value(first_left,left_child_value)
            }

            if let Some(first_right) = self.find_first_literal_on_right(parent) {
                let right_child_value = self.get_value(parent *2+2);
                self.add_value(first_right,right_child_value);
            }

            self.replace_pair_at_by_zero(parent);
        }
        to_explode.is_some()
    }

    fn add_value(&mut self, idx:usize, value:usize) {
        match self.tree[idx] {
            Some(Literal(v)) => self.tree[idx] = Some(Literal(v+value)),
            _ => panic!("Bug")
        };
    }

    fn get_value(&self, idx:usize) -> usize {
        match self.tree[idx] {
            Some(Literal(value)) => value,
            _ => panic!("Bug")
        }
    }

    fn find_node_to_explode(&self) -> Option<usize> {
        self.tree
            .iter()
            .enumerate()
            .skip(31)
            .find(|(_,n)| n.is_some())
            .map(|(idx,_)| idx)
    }


    fn find_first_literal_on_left(&self, start: usize) -> Option<usize> {
        let mut current = start;
        while current > 0 {
            let parent = parent_index(current);
            let left = left_child_index(parent);
            if left != current {
                let result = self.find_right_most_literal(left);
                if result.is_some() {
                    return result
                }
            }
            current = parent;
        }
        None
    }

    fn find_first_literal_on_right(&self, start: usize) -> Option<usize> {
        let mut current = start;
        while current > 0 {
            let parent = parent_index(current);
            let right = right_child_index(parent);
            if right != current {
                let result = self.find_left_most_literal(right);
                if result.is_some() {
                    return result
                }
            }
            current = parent;
        }
        None
    }


    fn replace_pair_at_by_zero(&mut self, pair: usize) {
        self.tree[left_child_index(pair)] = None;
        self.tree[right_child_index(pair)] = None;
        self.tree[pair] = Some(Literal(0));
    }

    fn find_right_most_literal(&self, sub_root: usize) -> Option<usize> {
        self.find_first_literal_on_a_side(sub_root,right_child_index, left_child_index)
    }

    fn find_left_most_literal(&self, sub_root: usize) -> Option<usize> {
        self.find_first_literal_on_a_side(sub_root,left_child_index,right_child_index)
    }

    fn find_first_literal_on_a_side(&self, sub_root: usize, first_to_check:fn(usize) -> usize, second_to_check:fn(usize) -> usize) -> Option<usize> {
        match self.tree[sub_root] {
            Some(Literal(_)) => Some(sub_root),
            Some(Pair) => {
                let result = self.find_first_literal_on_a_side(first_to_check(sub_root),first_to_check,second_to_check);
                if result.is_some() {
                    return result;
                }
                self.find_first_literal_on_a_side(second_to_check(sub_root),first_to_check,second_to_check)
            },
            None => None,
        }
    }
    fn find_index_to_split(&self, start: usize) -> Option<usize> {
        match self.tree[start] {
            Some(Literal(v)) if v >= 10 => Some(start),
            Some(Pair) => {
                let result = self.find_index_to_split(left_child_index(start));
                if result.is_some() {
                    result
                } else {
                    self.find_index_to_split(right_child_index(start))
                }
            },
            _ => None
        }
    }
}

fn parent_index(child:usize) -> usize {
    (child-1)/2
}


fn left_child_index(parent:usize) -> usize {
    parent*2+1
}

fn right_child_index(parent:usize) -> usize {
    parent*2+2
}


impl FromStr for Number {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = Number::default();
        parse(0, input,&mut result.tree);

        Ok(result)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(0,f)
    }
}

impl Number {
    fn format(&self, parent:usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.tree[parent] {
            Some(Literal(value)) => f.write_fmt(format_args!("{}",value))?,
            Some(Pair) => {
                f.write_char('[')?;
                self.format(parent*2+1, f)?;
                f.write_char(',')?;
                self.format(parent*2+2, f)?;
                f.write_char(']')?;
            },
            None=> {},
        }
        Ok(())
    }
}


fn parse(parent:usize, input:&str, tree:&mut Vec<Option<Node>>) {
    let splitted = split(input);
    if let Some((left,right)) = splitted {
        tree[parent] = Some(Pair);
        parse(parent*2+1, left, tree);
        parse(parent*2+2, right, tree);
    } else {
        tree[parent] = Some(Literal(parse_input!(input,usize)));
    }
}


fn split(input: &str) -> Option<(&str, &str)> {
    if !input.starts_with('[') {
        return None;
    }

    let mut count_opened = 0;
    let mut comma_pos = 0;
    let mut end = 0;
    for (pos, c) in input.chars().enumerate() {
        match c {
            '[' => count_opened += 1,
            ']' => count_opened -= 1,
            ',' => if count_opened == 1 { comma_pos = pos }
            _ => ()
        }

        if count_opened == 0 {
            end = pos;
            break;
        }
    }

    Some((&input[1..comma_pos], &input[comma_pos + 1..end]))
}