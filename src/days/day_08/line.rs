use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;


//const LETTERS:[u32;7] = ["c","f","a","e","b","d","g"];

const LETTERS: [u32; 7] = [4, 32, 1, 16, 2, 8, 64];



fn bin_to_digit(bin: u32) -> u32 {
    match bin {
        36 => 1,
        93 => 2,
        109 => 3,
        46 => 4,
        107 => 5,
        123 => 6,
        37 => 7,
        127 => 8,
        111 => 9,
        119 => 0,
        _ => panic!("Cannot convert bin {}", bin)
    }
}


pub struct Line<T> {
    numbers:Vec<T>,
    digits:Vec<T>,
}

impl <T> Line<T> {
    pub fn digits(&self) -> &Vec<T> {
        &self.digits
    }
}

impl <T> Display for Line<T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?} | {:?}",self.numbers, self.digits)
    }
}

impl Line<String> {

    pub fn compute_histo(&self) -> [i32;7] {
        let a = 'a' as usize;

        let mut histo = [0;7];
        for number in &self.numbers {
            let weight = match number.len() {
                3 => -4,
                5|6 => 1,
                _ => 0
            };

            if weight == 0 {
                continue;
            }

            number.chars()
                .map(|c|  c as usize -a)
                .for_each(|i| histo[i] += weight)



        };

        histo
    }

    pub fn transform_to_u32(&self) -> Line<u32> {
        let histo = self.compute_histo();
        let mut bit :[u32;7] = [0;7];
        for i in 0..7 {
            bit[i] = LETTERS[histo[i] as usize]
        };

        let to_digit = |n:&str| {
            let bin = n.chars()
                .map(|c| c as usize - 'a' as usize)
                .map(|i| bit[i])
                .sum();
            bin_to_digit(bin)
        };

        let numbers:Vec<u32> = self.numbers.iter().map(|n| to_digit(n)).collect();
        let digits:Vec<u32> = self.digits.iter().map(|n| to_digit(n)).collect();

        Line{numbers,digits}
    }

}

impl Line<u32> {
    pub fn display_value(&self) -> u32 {
        self.digits[0]*1000+self.digits[1]*100+self.digits[2]*10+self.digits[3]
    }
}

impl FromStr for Line<String> {
    type Err = String;

    fn from_str(line_str: &str) -> Result<Self, Self::Err> {
        let tokens:Vec<&str> = line_str.split("|").collect();

        let numbers_str:Vec<&str> = tokens.get(0).ok_or(format!("Cannot parse '{}'",line_str))?
            .trim()
            .split(" ")
            .collect();
        let digits_str:Vec<&str> = tokens.get(1).ok_or(format!("Cannot parse '{}'",line_str))?
            .trim()
            .split(" ")
            .collect();

        let mut numbers = Vec::new();
        let mut digits = Vec::new();

        for i in 0..10 {
            numbers.push(numbers_str[i].to_string());
        };

        for i in 0..4 {
            digits.push(digits_str[i].to_string());
        };

        Ok(Line{numbers,digits})
    }
}