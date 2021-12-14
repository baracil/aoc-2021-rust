use std::collections::HashMap;
use std::str::FromStr;
use crate::days::day_12::caves::Node::{BigCave, End, SmallCave, Start};
use crate::parse_input;

#[derive(Copy, Clone)]
enum Node {
    Start,
    End,
    SmallCave,
    BigCave,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(match name {
            "start" => Start,
            "end" => End,
            _ => if name.chars().next().unwrap().is_uppercase() { BigCave } else { SmallCave }
        })
    }
}

pub struct Caves {
    start_idx: usize,
    nodes: Vec<Node>,
    graph: HashMap<usize, Vec<usize>>,
}


impl Caves {

    pub fn parse(input:&str) -> Self {
        input.split('\n').fold(CavesAcc::default(), |mut acc, line| {
            acc.push_one_line(line);
            acc
        }).build()
    }

    pub(crate) fn is_end(&self, idx: usize) -> bool {
        matches!(self.nodes[idx], End)
    }

    pub fn number_of_nodes(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_start(&self) -> usize {
        self.start_idx
    }

    pub fn get_connected_caves(&self, idx: usize) -> &Vec<usize> {
        self.graph.get(&idx).expect("Invalid index")
    }

    pub(crate) fn is_start_or_small_cave(&self, idx: usize) -> bool {
        matches!(self.nodes[idx],Start|SmallCave)
    }

    pub(crate) fn is_small_cave(&self, idx: usize) -> bool {
        matches!(self.nodes[idx],SmallCave)
    }
}

#[derive(Default)]
pub struct CavesAcc<'a> {
    start_idx: usize,
    nodes: Vec<Node>,
    node_idx: HashMap<&'a str, usize>,
    graph: HashMap<usize, Vec<usize>>,
}


impl<'a> CavesAcc<'a> {
    pub fn push_one_line(&mut self, line: &'a str) {
        let node_names: Vec<&str> = line.split('-').collect();

        let node0 = self.handle_name_and_get_index(node_names[0]);
        let node1 = self.handle_name_and_get_index(node_names[1]);

        self.graph.entry(node0).or_insert_with(Vec::new).push(node1);
        self.graph.entry(node1).or_insert_with(Vec::new).push(node0);
    }

    pub fn build(self) -> Caves {
        Caves { start_idx: self.start_idx, nodes: self.nodes, graph: self.graph }
    }

    fn handle_name_and_get_index(&mut self, node_name: &'a str) -> usize {
        let existing = self.node_idx.get(node_name);
        if let Some(idx) = existing {
            return *idx;
        };
        let idx = self.nodes.len();


        let node = parse_input!(node_name,Node);

        self.node_idx.insert(node_name, idx);
        self.nodes.push(node);

        if let Start = node {
            self.start_idx = idx;
        }
        idx
    }
}