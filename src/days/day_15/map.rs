use std::fmt::{Display, Formatter, Write};

const SENTINEL:u32 = 0;

pub struct Map {
    nb_rows:usize,
    nb_columns:usize,
    start_position:usize,
    end_position:usize,
    risks:Vec<u32>,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for r in 1..self.nb_rows-1 {
            f.write_char('\n')?;
            for c in 1..self.nb_columns-1 {
                f.write_fmt(format_args!("{}",self.risks[c+r*self.nb_columns]))?;
            }
            f.write_char('#')?;
        }
        Ok(())
    }
}

impl Map {

    pub(crate) fn is_outside(&self, position:usize) -> bool {
        self.risks[position] == SENTINEL
    }

    pub fn end_position(&self) -> usize {
        self.end_position
    }
}

impl Map {
    pub(crate) fn start_position(&self) -> usize {
        self.start_position
    }


    pub fn nb_columns(&self) -> usize {
        self.nb_columns
    }
}


impl Map {

    pub fn risk_at(&self,position:usize) -> u32 {
        self.risks[position]
    }

    pub fn nb_elements(&self) -> usize {
        self.risks.len()
    }

    pub fn parse_part1(lines:&[String]) -> Map {
        let nb_rows = lines.len()+2;
        let nb_columns = lines[0].len()+2;

        let mut risks= vec![SENTINEL;nb_rows* nb_columns];

        for (row_idx,line) in lines.iter().enumerate() {
            for (col_idx,char) in line.chars().enumerate() {
                let idx = (1+row_idx)* nb_columns + (1+col_idx);
                let base_risk = ((char as u8) - b'0') as u32;
                risks[idx]= base_risk
            }
        }

        Map{nb_columns,nb_rows, start_position:nb_columns+1, end_position:nb_columns*(nb_rows-1)-2, risks}
    }

    pub fn parse_part2(lines:&[String]) -> Map {
        let nb_rows = lines.len();
        let nb_columns = lines[0].len();

        let nb_map_rows = nb_rows*5 + 2;
        let nb_map_columns = nb_columns*5 + 2;



        let mut risks= vec![SENTINEL; nb_map_rows * nb_map_columns];

        for i in 0..5 {
            for j in 0..5 {

                for (row_idx,line) in lines.iter().enumerate() {
                    for (col_idx,char) in line.chars().enumerate() {

                        let idx = (1+row_idx + j*nb_rows) * nb_map_columns + (1+col_idx+ i *nb_columns);
                        let base_risk = ((char as u8) - b'0') as u32;
                        let offset = (i +j) as u32;
                        let risk = ((base_risk+offset-1)%9)+1;

                        risks[idx]=risk;
                    }
                }

            }
        }


        Map{ nb_columns: nb_map_columns, nb_rows:nb_map_rows, start_position: nb_map_columns +1, end_position: nb_map_columns *(nb_map_rows -1)-2, risks}
    }
}