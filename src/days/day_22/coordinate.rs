

pub struct Coordinate {
    pub value:i32,
    pub size:usize,
}

impl Coordinate {
    pub fn new_with_min_max(vmin:i32, vmax:i32) -> Self {
        Coordinate{value:vmin,size:(vmax-vmin) as usize}
    }
}
