#[macro_export]
macro_rules! parse_input { ($x:expr, $t:ident) => ($x.trim().parse::<$t>().expect(&format!("Cannot parse '{}'",$x))) }

#[macro_export]
macro_rules! try_parse_input { ($x:expr, $t:ident) => ($x.trim().parse::<$t>().map_err(|o| format!("Cannot parse {} : {:?}",$x,o))) }
