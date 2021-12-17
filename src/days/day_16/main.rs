use crate::days::day_16::packet_reader::read_packet;
use crate::days::day_16::reader::Reader;
use crate::Part;
use crate::problem::{AOCResult, Problem};

#[allow(dead_code)]
pub fn day16_launch(part: Part) -> AOCResult<String> {
    let hexa_message = parse_input(false)?;
    match part {
        Part::Part1 => part1(&hexa_message),
        Part::Part2 => part2(&hexa_message)
    }
}

fn part1(hexa_message:&str) -> AOCResult<String> {
    let mut reader = Reader::from_hexa(hexa_message);
    let packet = read_packet(&mut reader);
    Ok(packet.sum_of_versions().to_string())
}

fn part2(hexa_message:&str) -> AOCResult<String> {
    let mut reader = Reader::from_hexa(hexa_message);
    let packet = read_packet(&mut reader);
    Ok(packet.value().to_string())
}

#[allow(dead_code)]
fn parse_input(for_test:bool) -> AOCResult<String> {
    Problem::factory(for_test)(16).read_input()
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::days::day_16::main::{parse_input, part1, part2};
    use crate::days::day_16::packet_reader::read_packet;
    use crate::days::day_16::reader::Reader;

    #[test]
    fn day16_part1_test_01()  {
        let mut reader = Reader::from_hexa("8A004A801A8002F478");
        let result = read_packet(&mut reader).sum_of_versions();
        assert_eq!(result,16)
    }
    #[test]
    fn day16_part1_test_02()  {
        let mut reader = Reader::from_hexa("620080001611562C8802118E34");
        let result = read_packet(&mut reader).sum_of_versions();
        assert_eq!(result,12)
    }
    #[test]
    fn day16_part1_test_03()  {
        let mut reader = Reader::from_hexa("C0015000016115A2E0802F182340");
        let result = read_packet(&mut reader).sum_of_versions();
        assert_eq!(result,23)
    }
    #[test]
    fn day16_part1_test_04()  {
        let mut reader = Reader::from_hexa("A0016C880162017C3686B18A3D4780");
        let result = read_packet(&mut reader).sum_of_versions();
        assert_eq!(result,31)
    }

    #[test]
    fn day16_part2_test_01()  {
        let mut reader = Reader::from_hexa("C200B40A82");
        let result = read_packet(&mut reader).value();
        assert_eq!(result,3)
    }
    #[test]
    fn day16_part2_test_02()  {
        let mut reader = Reader::from_hexa("880086C3E88112");
        let result = read_packet(&mut reader).value();
        assert_eq!(result,7)
    }
    #[test]
    fn day16_part2_test_03()  {
        let mut reader = Reader::from_hexa("9C0141080250320F1802104A08");
        let result = read_packet(&mut reader).value();
        assert_eq!(result,1)
    }
    #[test]
    fn day16_part2_test_04()  {
        let mut reader = Reader::from_hexa("F600BC2D8F");
        let result = read_packet(&mut reader).value();
        assert_eq!(result,0)
    }

    #[test]
    #[ignore]
    fn day16_part2_test()  {
//        assert_eq!(result,"")
    }
}