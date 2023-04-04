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
    mem.data[0xFFCC as usize] = cpu::INS_JSR;
    mem.data[0xFFCD as usize] = 0x42;
    mem.data[0xFFCE as usize] = 0xED;
    mem.data[0x0042 as usize] = 0x84;
    cpu.execute(5, &mut mem);
}
