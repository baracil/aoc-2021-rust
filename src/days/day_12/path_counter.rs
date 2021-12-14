use crate::days::day_12::caves::Caves;

pub struct PathCounter {
    visited: Vec<bool>,
    small_cave: Option<usize>,
    nb_paths: usize,
}

impl PathCounter {
    fn create(nb_nodes: usize) -> Self {
        let visited = vec![false; nb_nodes];
        PathCounter { visited, nb_paths: 0, small_cave: None }
    }

    pub fn count_pathes_part1(caves: &Caves) -> usize {
        let mut counter = PathCounter::create(caves.number_of_nodes());
        counter.count_part1(caves)
    }

    pub fn count_pathes_part2(caves: &Caves) -> usize {
        let mut counter = PathCounter::create(caves.number_of_nodes());
        counter.count_part2(caves)
    }

    fn count_part1(&mut self, caves: &Caves) -> usize {
        let start = caves.get_start();
        self.count_caves_part1(start, caves);
        self.nb_paths
    }

    fn count_caves_part1(&mut self, cave: usize, caves: &Caves) {
        // invariant: if start or small cave, it has not been visited yet, and this is not 'end'
        self.visited[cave] = caves.is_start_or_small_cave(cave);

        let connected_caves = caves.get_connected_caves(cave);

        for &connected_cave in connected_caves {
            if self.visited[connected_cave] {
                continue;
            }

            if caves.is_end(connected_cave) {
                self.nb_paths += 1;
            } else {
                self.count_caves_part1(connected_cave, caves)
            }
        };

        self.visited[cave] = false
    }

    fn count_part2(&mut self, caves: &Caves) -> usize {
        let start = caves.get_start();
        self.count_caves_part2(start, caves);
        self.nb_paths
    }

    fn count_caves_part2(&mut self, cave: usize, caves: &Caves) {
        // invariant: cave
        //  - if start, it has not been visited yet,
        //  - this is not 'end'
        //  - if it is a small cave, either not visited or small_cave is None

        let mut small_cave_set: bool = false;

        if caves.is_small_cave(cave) && self.visited[cave] {
            self.small_cave = Some(cave);
            small_cave_set = true;
        }

        self.visited[cave] = caves.is_start_or_small_cave(cave);

        let connected_caves = caves.get_connected_caves(cave);

        for &connected_cave in connected_caves {
            if self.visited[connected_cave] && (!caves.is_small_cave(connected_cave) || self.small_cave.is_some()) {
                continue;
            }

            if caves.is_end(connected_cave) {
                self.nb_paths += 1;
            } else {
                self.count_caves_part2(connected_cave, caves)
            }
        };

        if small_cave_set {
            self.small_cave = None
        } else {
            self.visited[cave] = false
        }
    }
}


