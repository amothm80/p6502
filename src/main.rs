use std;

type Byte = u8;
type Word = u16;
const MAX_MEM: usize = 1024 * 64;

//operators
const INS_LDA_IM: Byte = 0xA9;

struct MEM{
    data: [ Byte ; MAX_MEM ],
}

impl MEM{
    fn init(&mut self){
        self.data = [0;MAX_MEM];
    }
}
struct CPU{
    
    pc: Word, //accumulator
    sp: Word, //stack pointer

    //8bit registers
    a: Byte,
    x: Byte,
    y: Byte,

    //flags
    flags: Byte, //NV_BDIZC
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
    fn reset (&mut self, mem: &mut MEM){
        self.pc = 0xFFFC;
        self.sp = 0x0100;

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.flags = 0b0000_0000;

        mem.init();
    }

    fn fetch_byte(&mut self, cycles:&mut u32, mem: &mut MEM) -> Byte{
        //assert!((self.pc as usize) >= 0);
        assert!((self.pc as usize) < MAX_MEM);
        assert!(*cycles >0);
        let d: Byte = mem.data[self.pc as usize];
        self.pc += 1;
        *cycles = *cycles - 1;
        return d;
    }

    fn execute(&mut self,mut cycles: u32, mem: &mut MEM){
        while cycles > 0{
            let ins:Byte = self.fetch_byte(&mut cycles, mem);
            match ins {
                //0_u8..=168_u8 => println!("others"),
                INS_LDA_IM => {
                    let value:Byte = self.fetch_byte(&mut cycles, mem);
                    self.a = value;
                    if self.a == 0 { self.set_z()};
                    if self.a & 0b1000_0000 > 0 {self.set_n()};
                },
                //170_u8..=u8::MAX => println!("others"),
                _ => println!("others"),
            }
        }
    }

    fn set_n(&mut self){ self.flags |= 0b1000_0000;}
    fn set_v(&mut self){ self.flags |= 0b0100_0000;}
    fn set_b(&mut self){ self.flags |= 0b0001_0000;}
    fn set_d(&mut self){ self.flags |= 0b0000_1000;}
    fn set_i(&mut self){ self.flags |= 0b0000_0100;}
    fn set_z(&mut self){ self.flags |= 0b0000_0010;}
    fn set_c(&mut self){ self.flags |= 0b0000_0001;}

    fn reset_n(&mut self){ self.flags &= 0b0111_1111;}
    fn reset_v(&mut self){ self.flags &= 0b1011_1111;}
    fn reset_b(&mut self){ self.flags &= 0b1110_1111;}
    fn reset_d(&mut self){ self.flags &= 0b1111_0111;}
    fn reset_i(&mut self){ self.flags &= 0b1111_1011;}
    fn reset_z(&mut self){ self.flags &= 0b1111_1101;}
    fn reset_c(&mut self){ self.flags &= 0b1111_1110;}

}
fn main() {
    let mut mem = MEM { data:[0;MAX_MEM] }; //clear memory for mem_size
    let mut cpu = CPU {
        pc : 0xFFFC,
        sp : 0x0100,

        a: 0,
        x: 0,
        y: 0,

        flags: 0b0000_0000,
    };
    cpu.reset(&mut mem);
    cpu.execute(2, &mut mem);
    cpu.execute(2, &mut mem);
    //println!("Hello, world!");
    //fetch video 1 at 20:17 line 50
}
