

type Byte = u8;
type Word = u16;
const MAX_MEM: usize = 1024 * 64;
pub struct MEM{
    pub data: [ Byte ; MAX_MEM ],
}

impl MEM{
    pub fn init(&mut self){
        self.data = [0;MAX_MEM];
    }
}
