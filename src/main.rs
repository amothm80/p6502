use std;
mod mem;
mod cpu;
use crate::mem::MEM;
use crate::cpu::CPU;

type Byte = u8;
type Word = u16;
const MAX_MEM: usize = 1024 * 64;

fn main() {
    //let mut mem = MEM { data:[0;MAX_MEM] }; //clear memory for mem_size
    let mut mem = MEM::new(); //clear memory for mem_size
    let mut cpu = CPU::new();
    cpu.reset(&mut mem);
    mem.data[0xFFFC as usize] = cpu::INS_LDA_IM;
    mem.data[0xFFFD as usize] = 0x42;
    cpu.execute(2, &mut mem);
}
