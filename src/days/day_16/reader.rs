
//TODO use lifetime you lazy dev

pub struct Reader {
    message: String,
    end: usize,
    index: usize,
}

impl Reader {
    pub fn remaining_bits(&self) -> usize {
        self.end - self.index
    }

    pub fn extract(&mut self, length:u32) -> Reader {
        let pos = self.move_by(length as usize);
        Reader{message:self.message.clone(),end:self.index, index:pos}
    }

    pub fn read(&mut self, nb_bits: usize) -> u32 {
        let pos = self.move_by(nb_bits);

        u32::from_str_radix(&self.message[pos..self.index], 2)
            .unwrap_or_else(|_| panic!("Cannot convert str to u32"))
    }

    pub fn read_one_bit(&mut self) -> bool {
        self.read(1) == 1
    }

    fn move_by(&mut self, nb_bits:usize) -> usize {
        if self.index + nb_bits > self.end {
            panic!("Index out of bound")
        }

        let pos = self.index;
        self.index += nb_bits;
        pos
    }

    pub fn from_hexa(message: &str) -> Reader {
        let message = message.chars().map(to_binary).collect::<String>();
        let message_len = message.len();
        Reader{message,end:message_len,index:0}
    }
}

pub fn to_binary(hexa_digit: char) -> String {
    match hexa_digit {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Cannot convert '{}' to binary",hexa_digit)
    }.to_string()
}