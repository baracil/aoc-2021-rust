
pub struct Map {
    heights: Vec<u32>,
    nb_cols: usize,
}

impl Map {
    pub fn parse(lines: Vec<String>) -> Self {
        let nb_rows = lines.len() + 2;
        let nb_cols = lines[0].len() + 2;

        let mut heights = Vec::new();
        heights.reserve(nb_rows * nb_cols);

        (0..nb_cols).for_each(|_| heights.push(9));
        for line in lines.iter() {
            heights.push(9);
            line.chars()
                .map(|c| c as u32 - '0' as u32)
                .for_each(|d| heights.push(d));
            heights.push(9)
        }
        (0..nb_cols).for_each(|_| heights.push(9));

        Map { heights, nb_cols }
    }

    fn compute_low_pos(&self) -> impl Iterator<Item=(usize, &u32)> {
        self.heights
            .iter()
            .enumerate()
            .filter(|(idx, h)| self.is_low(idx, h))
    }

    pub fn compute_part1(&self) -> u32 {
        self.compute_low_pos()
            .map(|(_, h)| h + 1)
            .sum()
    }

    pub fn compute_part2(&self) -> u32 {
        compute_basin_sizes(&self)
    }

    fn is_low(&self, pos: &usize, h: &u32) -> bool {
        if *h == 9 {
            return false;
        }
        let low = *h < self.heights[pos - 1]
            && *h < self.heights[pos + 1]
            && *h < self.heights[pos + self.nb_cols]
            && *h < self.heights[pos - self.nb_cols];
        return low;
    }
}


fn compute_basin_sizes(map: &Map) -> u32 {
    let nb_positions = map.heights.len();
    let mut sizes = Vec::new();

    let mut visited: Vec<bool> = map.heights.iter().map(|h| *h == 9).collect();
    let mut current = 0;
    loop {
        current = (current..nb_positions).find(|p| !visited[*p]).unwrap_or(nb_positions);

        if current >= nb_positions {
            break;
        }
        let size = fill_basin(&mut visited, current, map.nb_cols) as i32;
        sizes.push(-size);
    };

    sizes.sort();
    (0..3).map(|i| sizes[i]).fold(-1, |i1,i2| i1*i2).abs() as u32

}

fn fill_basin(visited: &mut Vec<bool>, position: usize, nb_cols: usize) -> u32 {
    if visited[position] {
        return 0;
    }

    visited[position] = true;

    return 1
        + fill_basin(visited, position - 1, nb_cols)
        + fill_basin(visited, position + 1, nb_cols)
        + fill_basin(visited, position - nb_cols, nb_cols)
        + fill_basin(visited, position + nb_cols, nb_cols);
}