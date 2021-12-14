use crate::days::day_10::stack::Stack;

pub fn syntax_score(line: &str) -> usize {
    let mut stack = Stack::new(line.len());


    for c in line.chars() {
        let score = match c {
            '(' => push(&mut stack, ')'),
            '[' => push(&mut stack, ']'),
            '{' => push(&mut stack, '}'),
            '<' => push(&mut stack, '>'),
            ')' | ']' | '}' | '>' => depop(&mut stack, c),
            _ => panic!("Unknown char '{}'", c)
        };
        if let Some(score) = score {
            return score;
        }
    };
    0
}

fn push(stack: &mut Stack<char>, c: char) -> Option<usize> {
    stack.push(c);
    None
}

fn depop(stack: &mut Stack<char>, expected: char) -> Option<usize> {
    let depoped = stack.pop();

    if depoped == Some(expected) {
        None
    } else {
        let score = match expected {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Unknown char '{}'", expected)
        };
        Some(score)
    }
}