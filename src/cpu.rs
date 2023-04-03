use crate::MEM;


const MAX_MEM: usize = 1024 * 64;
//operators
pub const INS_LDA_IM: Byte = 0xA9; //load accumulator immediate
pub const INS_LDA_ZP: Byte = 0xA5; //load accumulator zero page
type Byte = u8;
type Word = u16;
pub struct CPU{
    
    pub pc: Word, //accumulator
    pub sp: Word, //stack pointer

    //8bit registers
    pub a: Byte,
    pub x: Byte,
    pub y: Byte,

    //flags
    pub flags: Byte, //NV_BDIZC
    //N Negative flag 0 positive 1 negative
    //V Overflow flag 0 false 1 true
    //
    //B Break Command Flag 0 No break 1 Break
    //D Decimal Mode Flag 0 false 1 true
    //I IRQ disable flag 0 enable 1 disable
    //Z Zero flag 0 result not zero 1 result zero
    //C Carry flag 0 false 1 true
}

impl CPU{
    pub fn new ()->CPU{
        CPU{pc :0xFFFC,
        sp : 0x0100,
        a: 0,
        x: 0,
        y: 0,
        flags: 0b0000_0000}        
    }
    pub fn reset (&mut self, mem: &mut MEM){
        self.pc = 0xFFFC;
        self.sp = 0x0100;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.flags = 0b0000_0000;

        mem.init();
    }

    fn fetch_byte(&mut self, cycles:&mut u32, mem: &MEM) -> Byte{
        let d: Byte = mem.data[self.pc as usize];
        self.pc += 1;
        *cycles = *cycles - 1;
        return d;
    }

    fn read_byte(&mut self, cycles:&mut u32, mem: &MEM, zpa: Byte) -> Byte{
        let d: Byte = mem.data[zpa as usize];
        *cycles = *cycles - 1;
        return d;
    }    

    pub fn execute(&mut self,mut cycles: u32, mem: &mut MEM){
        while cycles > 0{
            let ins:Byte = self.fetch_byte(&mut cycles, mem);
            match ins {
                INS_LDA_IM => {
                    let value:Byte = self.fetch_byte(&mut cycles, mem);
                    self.a = value;
                    self.lda_set_status();
                },
                INS_LDA_ZP =>{
                    let zpa:Byte = self.fetch_byte(&mut cycles, mem);
                    self.a = self.read_byte(&mut cycles, mem, zpa);
                    self.lda_set_status();
                }
                _ => println!("Instr. {:x} at add. {:x} not handled", ins, self.pc-1),
            }
        }
    }

    pub fn lda_set_status(&mut self){
        if self.a == 0 { self.set_z()};
        if self.a & 0b1000_0000 > 0 {self.set_n()};
    }

    pub fn set_n(&mut self){ self.flags |= 0b1000_0000;}
    pub fn set_v(&mut self){ self.flags |= 0b0100_0000;}
    pub fn set_b(&mut self){ self.flags |= 0b0001_0000;}
    pub fn set_d(&mut self){ self.flags |= 0b0000_1000;}
    pub fn set_i(&mut self){ self.flags |= 0b0000_0100;}
    pub fn set_z(&mut self){ self.flags |= 0b0000_0010;}
    pub fn set_c(&mut self){ self.flags |= 0b0000_0001;}

    pub fn reset_n(&mut self){ self.flags &= 0b0111_1111;}
    pub fn reset_v(&mut self){ self.flags &= 0b1011_1111;}
    pub fn reset_b(&mut self){ self.flags &= 0b1110_1111;}
    pub fn reset_d(&mut self){ self.flags &= 0b1111_0111;}
    pub fn reset_i(&mut self){ self.flags &= 0b1111_1011;}
    pub fn reset_z(&mut self){ self.flags &= 0b1111_1101;}
    pub fn reset_c(&mut self){ self.flags &= 0b1111_1110;}

}