use super::state::{CPU};

trait ALU {
    fn and(&mut self, a: u16, b: u16) -> u16;
    fn or(&mut self, a: u16, b: u16) -> u16;
    fn xor(&mut self, a: u16, b: u16) -> u16;
    fn adda(&mut self, a: u16, b: u16) -> u16;
    fn suba(&mut self, a: u16, b: u16) -> u16;
    fn addl(&mut self, a: u16, b: u16) -> u16;
    fn subl(&mut self, a: u16, b: u16) -> u16;
    fn sla(&mut self, a: u16, b: u16) -> u16;
    fn sra(&mut self, a: u16, b: u16) -> u16;
    fn sll(&mut self, a: u16, b: u16) -> u16;
    fn srl(&mut self, a: u16, b: u16) -> u16;
    fn cpa(&mut self, a: u16, b: u16) -> bool;
    fn cpl(&mut self, a: u16, b: u16) -> bool;
}

impl ALU for CPU {
    fn and(&mut self, a: u16, b: u16) -> u16 {
        todo!()
    }

    fn or(&mut self, a: u16, b: u16) -> u16 {
        todo!()
    }

    fn xor(&mut self, a: u16, b: u16) -> u16 {
        todo!()
    }

    fn adda(&mut self, a: u16, b: u16) -> u16 {
        todo!()
    }

    fn suba(&mut self, a: u16, b: u16) -> u16 {
        todo!()
    }

    fn addl(&mut self, a: u16, b: u16) -> u16 {
        todo!()
    }

    fn subl(&mut self, a: u16, b: u16) -> u16 {
        todo!()
    }

    fn sla(&mut self, a: u16, b: u16) -> u16 {
        todo!()
    }

    fn sra(&mut self, a: u16, b: u16) -> u16 {
        todo!()
    }

    fn sll(&mut self, a: u16, b: u16) -> u16 {
        todo!()
    }

    fn srl(&mut self, a: u16, b: u16) -> u16 {
        todo!()
    }

    fn cpa(&mut self, a: u16, b: u16) -> bool {
        todo!()
    }

    fn cpl(&mut self, a: u16, b: u16) -> bool {
        todo!()
    }
}
