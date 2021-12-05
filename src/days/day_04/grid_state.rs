#[derive(Clone,Copy)]
pub enum GridState {
    NotWinning,
    Winning(u32),
}
