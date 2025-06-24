
pub struct ALU;

pub trait ALUExecution {
    type Return;
    fn and(&mut self, a: u16, b: u16) -> Self::Return;
    fn or(&mut self, a: u16, b: u16) -> Self::Return;
    fn xor(&mut self, a: u16, b: u16) -> Self::Return;
    fn adda(&mut self, a: u16, b: u16) -> Self::Return;
    fn suba(&mut self, a: u16, b: u16) -> Self::Return;
    fn addl(&mut self, a: u16, b: u16) -> Self::Return;
    fn subl(&mut self, a: u16, b: u16) -> Self::Return;
    fn sla(&mut self, a: u16, b: u16) -> Self::Return;
    fn sra(&mut self, a: u16, b: u16) -> Self::Return;
    fn sll(&mut self, a: u16, b: u16) -> Self::Return;
    fn srl(&mut self, a: u16, b: u16) -> Self::Return;
    fn cpa(&mut self, a: u16, b: u16) -> Self::Return;
    fn cpl(&mut self, a: u16, b: u16) -> Self::Return;
}

pub struct Return {
    /// 演算結果
    pub result: u16,
    /// フラグ
    /// 0: Overflow flag
    /// 1: Sign flag
    /// 2: Zero flag
    pub flags: [bool; 3],
}

/// ALUの実装
/// ALUは、算術論理演算を行うユニットです。
impl ALUExecution for ALU {
    type Return = Return;
    /// 論理積演算 (AND)
    fn and(&mut self, a: u16, b: u16) -> Self::Return {
        let result = a & b;
        let overflow = false;
        let sign = (result & 0x8000) != 0;
        let zero = result == 0;
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 論理和演算 (OR)
    fn or(&mut self, a: u16, b: u16) -> Self::Return {
        let result = a | b;
        let overflow = false;
        let sign = (result & 0x8000) != 0;
        let zero = result == 0;
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 排他的論理和演算 (XOR)
    fn xor(&mut self, a: u16, b: u16) -> Self::Return {
        let result = a ^ b;
        let overflow = false;
        let sign = (result & 0x8000) != 0;
        let zero = result == 0;
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 算術加算演算 (ADDA)
    fn adda(&mut self, a: u16, b: u16) -> Self::Return {
        let a_s = a as i16;
        let b_s = b as i16;
        let result_of = a_s.overflowing_add(b_s);
        let result = result_of.0 as u16; // Convert back to u16
        let overflow = result_of.1; // Overflow occurs if the addition overflows
        let sign = (result & 0x8000) != 0;
        let zero = result == 0;
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 算術減算演算 (SUBA)
    fn suba(&mut self, a: u16, b: u16) -> Self::Return {
        let a_s = a as i16;
        let b_s = b as i16;
        let result_of = a_s.overflowing_sub(b_s);
        let result = result_of.0 as u16; // Convert back to u16
        let overflow = result_of.1; // Overflow occurs if the subtraction overflows
        let sign = (result & 0x8000) != 0;
        let zero = result == 0;
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 論理加算演算 (ADDL)
    fn addl(&mut self, a: u16, b: u16) -> Self::Return {
        let result_of = a.overflowing_add(b);
        let result = result_of.0 as u16; // Convert back to u16
        let overflow = result_of.1; // Overflow occurs if the addition overflows
        let sign = (result & 0x8000) != 0;
        let zero = result == 0;
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 論理減算演算 (SUBL)
    fn subl(&mut self, a: u16, b: u16) -> Self::Return {
        let result_of = a.overflowing_sub(b);
        let result = result_of.0 as u16; // Convert back to u16
        let overflow = result_of.1; // Overflow occurs if the subtraction overflows
        let sign = (result & 0x8000) != 0;
        let zero = result == 0;
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 算術左シフト演算 (SLA)
    fn sla(&mut self, a: u16, b: u16) -> Self::Return {
        let a_s = a as i16;
        let b_s = b as i16;
        let result_of = a_s.overflowing_shl(b_s as u32);
        let result = result_of.0 as u16; // Convert back to u16
        let overflow = if (0..=14).contains(&b_s) {
            result_of.1 // Overflow occurs if the shift overflows
        } else {
            false // No overflow for shifts greater than 14
        };
        let sign = (result & 0x8000) != 0;
        let zero = result == 0;
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 算術右シフト演算 (SRA)
    fn sra(&mut self, a: u16, b: u16) -> Self::Return {
        let a_s = a as i16;
        let b_s = b as i16;
        let result_of = a_s.overflowing_shr(b_s as u32);
        let result = result_of.0 as u16; // Convert back to u16
        let overflow = if (0..=15).contains(&b_s) {
            result_of.1 // Overflow occurs if the shift overflows
        } else {
            false // No overflow for shifts greater than 15
        };
        let sign = (result & 0x8000) != 0;
        let zero = result == 0;
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 論理左シフト演算 (SLL)
    fn sll(&mut self, a: u16, b: u16) -> Self::Return {
        let result_of = a.overflowing_shl(b as u32);
        let result = result_of.0 as u16; // Convert back to u16
        let overflow = if (0..=16).contains(&b) {
            result_of.1 // Overflow occurs if the shift overflows
        } else {
            false // No overflow for shifts greater than 16
        };
        let sign = (result & 0x8000) != 0;
        let zero = result == 0;
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 論理右シフト演算 (SRL)
    fn srl(&mut self, a: u16, b: u16) -> Self::Return {
        let result_of = a.overflowing_shr(b as u32);
        let result = result_of.0 as u16; // Convert back to u16
        let overflow = if (0..=16).contains(&b) {
            result_of.1 // Overflow occurs if the shift overflows
        } else {
            false // No overflow for shifts greater than 16
        };
        let sign = (result & 0x8000) != 0;
        let zero = result == 0;
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 算術比較演算 (CPA)
    /// return はありません
    /// 常に 0 を返します
    fn cpa(&mut self, a: u16, b: u16) -> Self::Return {
        let a_s = a as i16;
        let b_s = b as i16;
        let result = 0;
        let overflow = false; // CPA does not produce an overflow
        let sign = a_s < b_s; // Sign flag is set if A < B
        let zero = a_s == b_s; // Zero flag is set if A == B
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }

    /// 論理比較演算 (CPL)
    /// return はありません
    /// 常に 0 を返します
    fn cpl(&mut self, a: u16, b: u16) -> Self::Return {
        let result = 0;
        let overflow = false; // CPL does not produce an overflow
        let sign = a < b; // Sign flag is set if A < B
        let zero = a == b; // Zero flag is set if A == B
        Return {
            result,
            flags: [overflow, sign, zero],
        }
    }
}
