use std;

type Byte = u8;
type Word = u16;

struct CPU{
    
    PC: Word, //accumulator
    SP: Word, //stack pointer

    //8bit registers
    A: Byte,
    X: Byte,
    Y: Byte,

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
    fn Reset (&mut self){
        self.PC = 0xFFFC;
        self.SP = 0x0100;

        self.A = 0;
        self.X = 0;
        self.Y = 0;

        self.flags = 0b0000_0000;
    }

    fn SetN(&mut self){ self.flags |= 0b1000_0000;}
    fn SetV(&mut self){ self.flags |= 0b0100_0000;}
    fn SetB(&mut self){ self.flags |= 0b0001_0000;}
    fn SetD(&mut self){ self.flags |= 0b0000_1000;}
    fn SetI(&mut self){ self.flags |= 0b0000_0100;}
    fn SetZ(&mut self){ self.flags |= 0b0000_0010;}
    fn SetC(&mut self){ self.flags |= 0b0000_0001;}

    fn ResetN(&mut self){ self.flags &= 0b0111_1111;}
    fn ResetV(&mut self){ self.flags &= 0b1011_1111;}
    fn ResetB(&mut self){ self.flags &= 0b1110_1111;}
    fn ResetD(&mut self){ self.flags &= 0b1111_0111;}
    fn ResetI(&mut self){ self.flags &= 0b1111_1011;}
    fn ResetZ(&mut self){ self.flags &= 0b1111_1101;}
    fn ResetC(&mut self){ self.flags &= 0b1111_1110;}

}
fn main() {
    let mut cpu = CPU {
        PC : 0xFFFC,
        SP : 0x0100,

        A: 0,
        X: 0,
        Y: 0,

        flags: 0b0010_0000,
    };
    cpu.SetV();
    cpu.ResetV();
    cpu.Reset();
    println!("Hello, world!");
}
