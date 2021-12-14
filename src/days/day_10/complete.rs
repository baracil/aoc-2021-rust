use crate::days::day_10::stack::Stack;


pub fn complete_score(line: &str) -> Option<usize> {
    let mut stack = Stack::new(line.len());


    for c in line.chars() {
        let valid = match c {
            '(' => push_part2(&mut stack, ')'),
            '[' => push_part2(&mut stack, ']'),
            '{' => push_part2(&mut stack, '}'),
            '<' => push_part2(&mut stack, '>'),
            ')' | ']' | '}' | '>' => depop_part2(&mut stack, c),
            _ => panic!("Unknown char '{}'", c)
        };
        if !valid {
            return None
        }
    };

    let mut score = 0usize;
    loop {
        let c = stack.pop();
        if let Some(c) = c {
            let point = match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Unknown char '{}'", c)
            };

            score = score*5 + point;
        } else {
            return Some(score)
        }
    }

}

fn push_part2(stack: &mut Stack<char>, c: char) -> bool {
    stack.push(c);
    true
}

fn depop_part2(stack: &mut Stack<char>, expected: char) -> bool {
    let depoped = stack.pop();

    depoped == Some(expected)
}