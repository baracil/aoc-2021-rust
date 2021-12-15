use std::collections::VecDeque;
use crate::days::day_15::map::Map;

pub fn find_lowest_risk(map: &Map) -> u32 {
    let nb_columns = map.nb_columns();

    let mut lowest_risk_so_far = vec![u32::MAX; map.nb_elements()];
    let mut position_to_visit = VecDeque::<(usize,u32)>::new();
    position_to_visit.reserve(map.nb_elements());

    position_to_visit.push_back((map.start_position(), 0));

    while !position_to_visit.is_empty() {

        let (position,risk) = position_to_visit.pop_front().unwrap();

        if lowest_risk_so_far[position] < risk {
            continue
        }

        lowest_risk_so_far[position] = risk;


        for &neighbor in &[position-1,position+1,position-nb_columns,position+nb_columns] {
            if map.is_outside(neighbor) {
                continue
            }
            let new_risk = risk+map.risk_at(neighbor);

            if lowest_risk_so_far[neighbor] <= new_risk {
                continue
            }

            lowest_risk_so_far[neighbor] = new_risk;

            position_to_visit.push_back((neighbor,new_risk))
        }
    }

    lowest_risk_so_far[map.end_position()]


}

