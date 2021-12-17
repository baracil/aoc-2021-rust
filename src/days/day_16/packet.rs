
#[derive(Debug)]
pub enum Packet {
    Literal(u32,u32, u64),
    Operator(u32,u32, Vec<Packet>)
}


impl Packet {

    pub fn sum_of_versions(&self) -> u32 {
        match self {
            Packet::Literal(version, _, _) => *version,
            Packet::Operator(version, _, sub_packets) => {
                let sub_packet_sum:u32 = sub_packets.iter().map(|p| p.sum_of_versions()).sum();
                *version + sub_packet_sum
            }
        }
    }

    pub fn value(&self) -> u64 {
        match self {
            Packet::Literal(_, _, value) => *value,
            Packet::Operator(_, id, sub_packets) => {
                match id {
                    0 => sub_packets.iter().map(|p| p.value()).sum(),
                    1 => sub_packets.iter().map(|p| p.value()).product(),
                    2 => sub_packets.iter().map(|p|p.value()).min().unwrap(),
                    3 => sub_packets.iter().map(|p|p.value()).max().unwrap(),
                    5 => if sub_packets[0].value() > sub_packets[1].value() {1} else {0}
                    6 => if sub_packets[0].value() < sub_packets[1].value() {1} else {0}
                    7 => if sub_packets[0].value() == sub_packets[1].value() {1} else {0}
                    _ => panic!("Invalid Id {}",id)
                }
            }
        }
    }
}
