use crate::opcodes::Opcode;
use crate::types::BoundedUsize;

#[derive(Debug, Default)]
pub struct Vm {
    pub registers: [i32; 8],
    pc: usize,
    pub program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl Vm {
    pub fn new(
        fill_registers: Option<Vec<(usize, i32)>>,
        pc: Option<usize>,
        program: Vec<u8>,
        remainder: Option<u32>,
        equal_flag: Option<bool>,
    ) -> Vm {
        let mut registers: [i32; 8] = [0; 8];
        if let Some(fill_registers) = fill_registers {
            for (reg_index, value) in &fill_registers {
                let filled_register =
                    BoundedUsize::<0, 8>::fallible_new(*reg_index).unwrap();
                registers[*filled_register] = *value;
            }
        }
        let pc = pc.unwrap_or_default();
        let remainder = remainder.unwrap_or_default();
        let equal_flag = equal_flag.unwrap_or_default();
        Vm {
            registers,
            pc,
            program,
            remainder,
            equal_flag,
        }
    }

    pub fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    pub fn push_byte(&mut self, value: u8) {
        self.program.push(value);
    }

    pub fn push_bytes(&mut self, values: Vec<u8>) {
        for value in values {
            self.program.push(value);
        }
    }

    pub fn run(&mut self) {
        loop {
            let is_done = self.execute_instruction();
            if is_done {
                // Debug print!
                println!("Terminated. VM state: {:#?}", self);
                std::process::exit(0);
            }
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
            Opcode::SKIP => {
                self.next_8bits();
            }
            Opcode::HALT => {
                println!("HALT encountered");
                result = true;
            }
            Opcode::LOAD => {
                let register = self.next_8bits();
                let number = self.next_16bits();
                self.registers[register as usize] = number as i32;
            }
            Opcode::ADD => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.registers[self.next_8bits() as usize] =
                    register0 + register1;
            }
            Opcode::SUB => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.registers[self.next_8bits() as usize] =
                    register0 - register1;
            }
            Opcode::MUL => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.registers[self.next_8bits() as usize] =
                    register0 * register1;
            }
            Opcode::DIV => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.registers[self.next_8bits() as usize] =
                    register0 / register1;
                self.remainder = (register0 % register1) as u32;
            }
            Opcode::JUMP => {
                let target = self.next_register();
                self.next_16bits();
                self.pc = target as usize;
            }
            Opcode::JF => {
                let value = self.next_register();
                self.next_16bits();
                self.pc += value as usize;
            }
            Opcode::JB => {
                let value = self.next_register();
                self.next_16bits();
                self.pc -= value as usize;
            }
            Opcode::EQ => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.equal_flag = register0 == register1;
                self.next_8bits();
            }
            Opcode::NE => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.equal_flag = register0 != register1;
                self.next_8bits();
            }
            Opcode::GT => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.equal_flag = register0 > register1;
                self.next_8bits();
            }
            Opcode::LT => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.equal_flag = register0 < register1;
                self.next_8bits();
            }
            Opcode::GTE => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.equal_flag = register0 >= register1;
                self.next_8bits();
            }
            Opcode::LTE => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.equal_flag = register0 <= register1;
                self.next_8bits();
            }
            Opcode::JE => {
                let target = self.next_register();
                self.next_16bits();
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::JNE => {
                let target = self.next_register();
                self.next_16bits();
                if !self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::INC => {
                let register = self.next_8bits();
                self.next_16bits();
                self.registers[register as usize] += 1;
            }
            Opcode::DEC => {
                let register = self.next_8bits();
                self.next_16bits();
                self.registers[register as usize] -= 1;
            }
            Opcode::ILL => {
                println!("Unrecognized opcode found");
                result = true;
            }
        }
        result
    }

    fn next_register(&mut self) -> i32 {
        self.registers[self.next_8bits() as usize]
    }

    fn next_8bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8)
            | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm(
        fill_registers: Option<Vec<(usize, i32)>>,
        pc: Option<usize>,
        program: Vec<u8>,
    ) -> Vm {
        Vm::new(fill_registers, pc, program, None, None)
    }

    #[test]
    fn test_create_vm() {
        let test_vm = Vm::default();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_halt() {
        let program = vec![1, 0, 0, 0];
        let mut test_vm = get_test_vm(None, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_ill() {
        let program = vec![200, 0, 0, 0];
        let mut test_vm = get_test_vm(None, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        // Remember, this is how we represent `500` using two `u8`s in little
        // endian format: `1, 244`
        let program = vec![2, 0, 1, 244];
        let mut test_vm = get_test_vm(None, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let fill_registers = Some(vec![(1, 15), (2, 10)]);
        let program = vec![3, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 25);
    }

    #[test]
    fn test_opcode_sub() {
        let fill_registers = Some(vec![(1, 15), (2, 10)]);
        let program = vec![4, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 5);
    }

    #[test]
    fn test_opcode_mul() {
        let fill_registers = Some(vec![(1, 4), (2, 6)]);
        let program = vec![5, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 24);
    }

    #[test]
    fn test_opcode_div() {
        let fill_registers = Some(vec![(1, 8), (2, 5)]);
        let program = vec![6, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 1);
        assert_eq!(test_vm.remainder, 3)
    }

    #[test]
    fn test_opcode_jump() {
        let fill_registers = Some(vec![(0, 4)]);
        let program = vec![7, 0, 0, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jf() {
        let fill_registers = Some(vec![(0, 4)]);
        let program = vec![8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 8);
    }

    #[test]
    fn test_opcode_jb() {
        let fill_registers = Some(vec![(0, 8)]);
        let pc = Some(4);
        let program = vec![0, 0, 0, 0, 9, 0, 0, 0];
        let mut test_vm = get_test_vm(fill_registers, pc, program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_eq() {
        let fill_registers0 = Some(vec![(1, 32), (2, 32)]);
        let program0 = vec![10, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        test_vm0.run_once();
        assert!(test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 52), (2, 38)]);
        let program1 = vec![10, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        test_vm1.run_once();
        assert!(!test_vm1.equal_flag);
    }

    #[test]
    fn test_opcode_ne() {
        let fill_registers0 = Some(vec![(1, 32), (2, 32)]);
        let program0 = vec![11, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        test_vm0.run_once();
        assert!(!test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 52), (2, 38)]);
        let program1 = vec![11, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        test_vm1.run_once();
        assert!(test_vm1.equal_flag);
    }

    #[test]
    fn test_opcode_gt() {
        let fill_registers0 = Some(vec![(1, 8), (2, 5)]);
        let program0 = vec![12, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        test_vm0.run_once();
        assert!(test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 5), (2, 8)]);
        let program1 = vec![12, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        test_vm1.run_once();
        assert!(!test_vm1.equal_flag);
    }

    #[test]
    fn test_opcode_lt() {
        let fill_registers0 = Some(vec![(1, 8), (2, 5)]);
        let program0 = vec![13, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        test_vm0.run_once();
        assert!(!test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 5), (2, 8)]);
        let program1 = vec![13, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        test_vm1.run_once();
        assert!(test_vm1.equal_flag);
    }

    #[test]
    fn test_opcode_gte() {
        let fill_registers0 = Some(vec![(1, 77), (2, 64)]);
        let program0 = vec![14, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        test_vm0.run_once();
        assert!(test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 21), (2, 21)]);
        let program1 = vec![14, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        test_vm1.run_once();
        assert!(test_vm1.equal_flag);

        let fill_registers2 = Some(vec![(1, 8), (2, 64)]);
        let program2 = vec![14, 1, 2, 0];
        let mut test_vm2 = get_test_vm(fill_registers2, None, program2);
        test_vm2.run_once();
        assert!(!test_vm2.equal_flag);
    }

    #[test]
    fn test_opcode_lte() {
        let fill_registers0 = Some(vec![(1, 77), (2, 64)]);
        let program0 = vec![15, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        test_vm0.run_once();
        assert!(!test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 21), (2, 21)]);
        let program1 = vec![15, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        test_vm1.run_once();
        assert!(test_vm1.equal_flag);

        let fill_registers2 = Some(vec![(1, 8), (2, 64)]);
        let program2 = vec![15, 1, 2, 0];
        let mut test_vm2 = get_test_vm(fill_registers2, None, program2);
        test_vm2.run_once();
        assert!(test_vm2.equal_flag);
    }

    #[test]
    fn test_opcode_je() {
        let pc0 = Some(4);
        let program0 = vec![0, 0, 0, 0, 16, 0, 0, 0];
        let mut test_vm0 = get_test_vm(None, pc0, program0);
        test_vm0.equal_flag = true;
        test_vm0.run_once();
        assert_eq!(test_vm0.pc, 0);

        let pc1 = Some(4);
        let program1 = vec![0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0];
        let mut test_vm1 = get_test_vm(None, pc1, program1);
        test_vm1.run_once();
        assert_eq!(test_vm1.pc, 8);
    }

    #[test]
    fn test_opcode_jne() {
        let pc0 = Some(4);
        let program0 = vec![0, 0, 0, 0, 17, 0, 0, 0];
        let mut test_vm0 = get_test_vm(None, pc0, program0);
        test_vm0.run_once();
        assert_eq!(test_vm0.pc, 0);

        let pc1 = Some(4);
        let program1 = vec![0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0];
        let mut test_vm1 = get_test_vm(None, pc1, program1);
        test_vm1.equal_flag = true;
        test_vm1.run_once();
        assert_eq!(test_vm1.pc, 8);
    }
}
