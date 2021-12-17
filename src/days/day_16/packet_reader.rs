use crate::days::day_16::packet::Packet;
use crate::days::day_16::packet::Packet::{Literal, Operator};
use crate::days::day_16::reader::Reader;


const LITERAL_DIGIT_MASK:u32 = 15;

pub fn read_packet(reader:&mut Reader) -> Packet {

    let version = reader.read(3);
    let id = reader.read(3);

    match id {
        4 => read_literal(reader, version,id),
        _ => read_operator(reader, version, id),
    }
}

fn read_literal(reader: &mut Reader, version:u32, id:u32) -> Packet {
    let mut result = 0_u64;
    loop {
        let digit = reader.read(5);
        let value = digit&LITERAL_DIGIT_MASK;
        result = result*16+(value as u64);

        if digit == value {
            break
        }
    }
    Literal(version,id,result)
}

fn read_operator(reader: &mut Reader, version:u32, id:u32) -> Packet {
    let length_type_id = reader.read_one_bit();

    let mut sub_packets = Vec::new();

    if length_type_id {
        let number_of_subpackets = reader.read(11);

        for _ in 0..number_of_subpackets {
            sub_packets.push(read_packet(reader));
        }

    } else {
        let total_length_of_subpackets = reader.read(15);

        let mut sub_reader = reader.extract(total_length_of_subpackets);

        while sub_reader.remaining_bits() > 6 {
            sub_packets.push(read_packet(&mut sub_reader))
        }
    };

    Operator(version,id, sub_packets)

}