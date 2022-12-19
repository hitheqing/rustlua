pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}


impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn truncate(&mut self){
        self.data = &self.data[self.pos..];
        self.pos = 0;
    }

    fn read_2(&mut self) -> [u8; 2] {
        let arr = [self.data[self.pos], self.data[self.pos + 1]];
        self.pos += 2;
        self.truncate();
        arr
    }

    fn read_4(&mut self) -> [u8; 4] {
        let arr = [self.data[self.pos], self.data[self.pos + 1], self.data[self.pos + 2], self.data[self.pos + 3]];
        self.pos += 4;
        self.truncate();
        arr
    }

    fn read_8(&mut self) -> [u8; 8] {
        let arr = [self.data[self.pos + 0], self.data[self.pos + 1], self.data[self.pos + 2], self.data[self.pos + 3],
            self.data[self.pos + 4], self.data[self.pos + 5], self.data[self.pos + 6], self.data[self.pos + 7]];
        self.pos += 8;
        self.truncate();
        arr
    }

    pub fn read_byte(&mut self) -> u8 {
        let u = self.data[self.pos];
        self.pos += 1;
        self.truncate();
        u
    }

    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        let u = &self.data[self.pos..(self.pos + n)];
        self.pos += n;
        self.truncate();
        u.to_vec()
    }


    pub fn read_char(&mut self) -> char {
        let u = self.read_byte();
        self.truncate();
        char::from(u)
    }
    pub fn read_u16(&mut self) -> u16 {
        u16::from_le_bytes(self.read_2())
    }

    pub fn read_u32(&mut self) -> u32 {
        u32::from_le_bytes(self.read_4())
    }

    pub fn read_f64(&mut self) -> f64 {
        f64::from_le_bytes(self.read_8())
    }

    pub fn read_i64(&mut self) -> i64 {
        i64::from_le_bytes(self.read_8())
    }

    pub fn read_u64(&mut self) -> u64 {
        u64::from_le_bytes(self.read_8())
    }

    pub fn read_str(&mut self) -> String {
        let mut size: usize = self.read_byte() as usize;
        if size == 0 { return "".to_owned(); }
        if size == 0xff {
            size = self.read_u64() as usize;
        }
        let bytes = self.read_bytes(size - 1);
        self.truncate();
        String::from_utf8(bytes).unwrap().as_str().to_owned()
    }
}

