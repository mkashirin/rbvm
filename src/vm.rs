use crate::instruction::Opcode;

#[derive(Default)]
pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM::default()
    }

    pub fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        let mut result = false;
        if self.pc >= self.program.len() {
            result = true;
        }
        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered");
                result = true;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits();
                let number = self.next_16_bits();
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let first_register = self.registers[self.next_8_bits()];
                let second_register = self.registers[self.next_8_bits()];
                self.registers[self.next_8_bits()] =
                    first_register + second_register;
            }
            Opcode::SUB => {
                let first_register = self.registers[self.next_8_bits()];
                let second_register = self.registers[self.next_8_bits()];
                self.registers[self.next_8_bits()] =
                    first_register - second_register;
            }
            Opcode::MUL => {
                let first_register = self.registers[self.next_8_bits()];
                let second_register = self.registers[self.next_8_bits()];
                self.registers[self.next_8_bits()] =
                    first_register * second_register;
            }
            Opcode::DIV => {
                let first_register = self.registers[self.next_8_bits()];
                let second_register = self.registers[self.next_8_bits()];
                self.registers[self.next_8_bits()] =
                    first_register / second_register;
                self.remainder = (first_register % second_register) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits()];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits()];
                self.pc += value as usize;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits()];
                self.pc -= value as usize;
            }
            Opcode::IGL => {
                println!("Unrecognized opcode found. Terminating...");
                result = true;
            }
        }
        result
    }

    fn next_8_bits(&mut self) -> usize {
        let result = self.program[self.pc];
        self.pc += 1;
        result as usize
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8)
            | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        test_vm.program = vec![200, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        // Remember, this is how we represent `500` using two `u8`s in little
        // endian format: `1, 244`
        test_vm.program = vec![1, 0, 1, 244];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 10;
        test_vm.registers[2] = 15;
        test_vm.program = vec![2, 1, 2, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 25);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 15;
        test_vm.registers[2] = 10;
        test_vm.program = vec![3, 1, 2, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 5);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 4;
        test_vm.registers[2] = 6;
        test_vm.program = vec![4, 1, 2, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 24);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 8;
        test_vm.registers[2] = 5;
        test_vm.program = vec![5, 1, 2, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 1);
        assert_eq!(test_vm.remainder, 3)
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 4;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::new();
        test_vm.pc = 2;
        test_vm.registers[0] = 4;
        test_vm.program = vec![0, 0, 8, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }
}
