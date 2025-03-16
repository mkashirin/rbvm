use crate::instruction::Opcode;
use crate::types::BoundedUsize;

#[derive(Default)]
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
        let pc = pc.unwrap_or(0);
        let remainder = remainder.unwrap_or(0);
        let equal_flag = equal_flag.unwrap_or(false);
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

    pub fn run(&mut self) {
        loop {
            let is_done = self.execute_instruction();
            if is_done {
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
            Opcode::HLT => {
                println!("HLT encountered");
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
            Opcode::JMP => {
                let target = self.next_register();
                self.next_16bits();
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.next_register();
                self.next_16bits();
                self.pc += value as usize;
            }
            Opcode::JMPB => {
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
            Opcode::NEQ => {
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
            Opcode::GTQ => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.equal_flag = register0 >= register1;
                self.next_8bits();
            }
            Opcode::LTQ => {
                let (register0, register1) =
                    (self.next_register(), self.next_register());
                self.equal_flag = register0 <= register1;
                self.next_8bits();
            }
            Opcode::JEQ => {
                let target = self.next_register();
                self.next_16bits();
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::JNEQ => {
                let target = self.next_register();
                self.next_16bits();
                if !self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::IGL => {
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
    fn test_opcode_hlt() {
        let program = vec![1, 0, 0, 0];
        let mut test_vm = get_test_vm(None, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
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
    fn test_opcode_jmp() {
        let fill_registers = Some(vec![(0, 4)]);
        let program = vec![7, 0, 0, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpf() {
        let fill_registers = Some(vec![(0, 4)]);
        let program = vec![8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 8);
    }

    #[test]
    fn test_opcode_jmpb() {
        let fill_registers = Some(vec![(0, 8)]);
        let pc = Some(4);
        let program = vec![0, 0, 0, 0, 9, 0, 0, 0];
        let mut test_vm = get_test_vm(fill_registers, pc, program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_eq() {
        let fill_registers = Some(vec![(1, 32), (2, 32)]);
        let program = vec![10, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        let fill_registers = Some(vec![(1, 52), (2, 38)]);
        let program = vec![10, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_neq() {
        let fill_registers = Some(vec![(1, 32), (2, 32)]);
        let program = vec![11, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(!test_vm.equal_flag);

        let fill_registers = Some(vec![(1, 52), (2, 38)]);
        let program = vec![11, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_gt() {
        let fill_registers = Some(vec![(1, 8), (2, 5)]);
        let program = vec![12, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        let fill_registers = Some(vec![(1, 5), (2, 8)]);
        let program = vec![12, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_lt() {
        let fill_registers = Some(vec![(1, 8), (2, 5)]);
        let program = vec![13, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(!test_vm.equal_flag);

        let fill_registers = Some(vec![(1, 5), (2, 8)]);
        let program = vec![13, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_gtq() {
        let fill_registers = Some(vec![(1, 77), (2, 64)]);
        let program = vec![14, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        let fill_registers = Some(vec![(1, 21), (2, 21)]);
        let program = vec![14, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        let fill_registers = Some(vec![(1, 8), (2, 64)]);
        let program = vec![14, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_ltq() {
        let fill_registers = Some(vec![(1, 77), (2, 64)]);
        let program = vec![15, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(!test_vm.equal_flag);

        let fill_registers = Some(vec![(1, 21), (2, 21)]);
        let program = vec![15, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        let fill_registers = Some(vec![(1, 8), (2, 64)]);
        let program = vec![15, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        test_vm.run_once();
        assert!(test_vm.equal_flag);
    }

    #[test]
    fn test_opcode_jeq() {
        let pc = Some(4);
        let program = vec![0, 0, 0, 0, 16, 0, 0, 0];
        let mut test_vm = get_test_vm(None, pc, program);
        test_vm.equal_flag = true;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);

        let pc = Some(4);
        let program = vec![0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0];
        let mut test_vm = get_test_vm(None, pc, program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 8);
    }

    #[test]
    fn test_opcode_jneq() {
        let pc = Some(4);
        let program = vec![0, 0, 0, 0, 17, 0, 0, 0];
        let mut test_vm = get_test_vm(None, pc, program);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);

        let pc = Some(4);
        let program = vec![0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0];
        let mut test_vm = get_test_vm(None, pc, program);
        test_vm.equal_flag = true;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 8);
    }
}
