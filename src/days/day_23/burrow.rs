use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::days::day_23::amphipod::Amphipod;
use crate::days::day_23::consts::{EXTRA_LINES, HALLWAY_Y, is_not_in_front_of_a_room, ROOM_DEPTH_PART1, ROOM_DEPTH_PART2, STRIDE};
use crate::days::day_23::displacement::Displacement;
use crate::days::day_23::position::Position;

#[derive(Debug, Clone)]
pub struct Burrow {
    map: String,
    room_depth: i32,
    amphipods: HashMap<usize, Amphipod>,
}

impl Burrow {
    pub(crate) fn map(&self) -> &String {
        &self.map
    }
}

impl Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.map))
    }
}

impl Burrow {
    pub(crate) fn for_part1(map: &[String]) -> Burrow {
        Burrow::parse_map(map, ROOM_DEPTH_PART1)
    }

    pub(crate) fn for_part2(map: &[String]) -> Burrow {
        let new_map: Vec<String> = map.iter().take(3)
            .chain(EXTRA_LINES.iter())
            .chain(map.iter().skip(3))
            .cloned()
            .collect();
        Burrow::parse_map(&new_map, ROOM_DEPTH_PART2)
    }

    pub fn parse_map(map: &[String], room_depth: i32) -> Burrow {
        let map = map.iter().join("\n");

        let amphipods = map.chars()
            .map(|c| c as u8)
            .enumerate()
            .filter(|(_, c)| *c >= b'A' && *c <= b'D')
            .map(|(i, c)| (i, Amphipod::from(i, c - b'A', Position::from_linear_coordinate(i))))
            .collect();

        Burrow { amphipods, map, room_depth }
    }


    pub(crate) fn is_organized(&self) -> bool {
        self.amphipods.iter().all(|(_, a)| a.right_x())
    }

    pub fn with_moved_amphipod(&self, displacement: &Displacement) -> Self {
        let moved_char = self.amphipods.get(&displacement.amphipod_id()).unwrap().char();

        let amphipods = self.amphipods
            .values()
            .map(|amphipode| displacement.displace(amphipode))
            .map(|a| (a.id(), a))
            .collect();

        let idx_start = displacement.start().linear_coordinate();
        let idx_end = displacement.end().linear_coordinate();

        let map = self.map.chars().enumerate().map(|(idx, c)| {
            if idx == idx_start {
                '.'
            } else if idx == idx_end {
                moved_char
            } else {
                c
            }
        }).join("");


        Burrow { amphipods, map, room_depth: self.room_depth }
    }
}

impl Burrow {
    fn char_at(&self, position: &Position) -> char {
        self.char_at_xy(position.x, position.y)
    }

    fn char_at_xy(&self, x: i32, y: i32) -> char {
        let idx = x as usize + y as usize * STRIDE;
        self.map[idx..=idx].chars().next().unwrap()
    }

    fn is_empty(&self, position: &Position) -> bool {
        self.char_at(position) == '.'
    }

    fn is_empty_xy(&self, x: i32, y: i32) -> bool {
        self.char_at_xy(x, y) == '.'
    }

    fn is_possible(&self, displacement: &Displacement) -> bool {
        displacement.positions().skip(1).all(|p| self.is_empty(&p))
    }

    pub fn allowed_displacements(&self) -> Vec<Displacement> {
        let move_to_go_to_room = self.amphipods
            .values()
            .map(|a| self.go_to_room(a))
            .flatten()
            .find(|d| self.is_possible(d));

        if let Some(d) = move_to_go_to_room {
            return vec![d];
        }

        self.amphipods
            .values()
            .flat_map(|a| self.move_to_hallway(a))
            .collect()
    }

    fn is_at_right_position(&self, amphipod: &Amphipod) -> bool {
        let last_room_y = HALLWAY_Y + self.room_depth;
        if !amphipod.right_x() {
            false
        } else if amphipod.y() == last_room_y {
            true
        } else {
            (amphipod.y() + 1..=(HALLWAY_Y + self.room_depth))
                .map(|y| Position::of(amphipod.room_x(), y))
                .map(|p| self.char_at(&p))
                .all(|c| c == amphipod.char())
        }
    }

    fn cannot_get_out_of_room(&self, amphipod: &Amphipod) -> bool {
        !(HALLWAY_Y..amphipod.y()).all(|y| self.is_empty_xy(amphipod.x(), y))
    }

    fn move_to_hallway<'a>(&'a self, amphipod: &'a Amphipod) -> Box<dyn Iterator<Item=Displacement> + 'a> {
        let in_hallway = amphipod.is_in_hallway();
        let at_right_position = self.is_at_right_position(amphipod);
        let cannot_get_out_of_room = self.cannot_get_out_of_room(amphipod);
        if in_hallway || at_right_position || cannot_get_out_of_room {
            return Box::new(std::iter::empty());
        }


        let left = (1..).map(|dx| amphipod.x() - dx)
            .filter(|x| is_not_in_front_of_a_room(*x))
            .map(|x| Position::of(x, HALLWAY_Y))
            .take_while(|p| self.is_empty(p));

        let right = (1..).map(|x| amphipod.x() + x)
            .filter(|x| is_not_in_front_of_a_room(*x))
            .map(|x| Position::of(x, HALLWAY_Y))
            .take_while(|p| self.is_empty(p));

        Box::new(left.chain(right).map(|p| Displacement::create(amphipod, p)))
    }

    fn go_to_room(&self, amphipod: &Amphipod) -> Option<Displacement> {
        if amphipod.right_x() {
            None
        } else {
            let first_room_y = HALLWAY_Y + 1;
            let last_room_y = HALLWAY_Y + self.room_depth;

            let first_vacant = {
                let mut last = None;
                for y in first_room_y..=last_room_y {
                    let position = Position::of(amphipod.room_x(), y);
                    if self.is_empty(&position) {
                        last = Some(position)
                    } else {
                        break;
                    }
                }
                last
            }?;

            let all_friend_below = {
                let mut result = true;
                for y in first_vacant.y + 1..=last_room_y {
                    let c = self.char_at_xy(amphipod.room_x(), y);
                    if c != amphipod.char() {
                        result = false;
                        break;
                    }
                }
                result
            };

            if all_friend_below {
                Some(Displacement::create(amphipod, first_vacant))
            } else {
                None
            }
        }
    }
}
