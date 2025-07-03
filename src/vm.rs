use crate::opcodes::Opcode;
use crate::types::BoundedUsize;

const IGNORE_HALTED: bool = false;
const IGNORE_ILLEGAL: bool = false;

#[derive(Debug)]
pub enum Error {
    HaltEncountered,
    IllegalOpcode,
    InstructionNotParsed,
    ReachedEof,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum State {
    #[default]
    Executing,
    Resumed,
    Halted,
    ReachedEof,
    Crashed,
}

#[derive(Debug, Default, Clone)]
pub struct Vm {
    pub registers: [i32; 8],
    pc: usize,
    pub program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
    state: State,
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
        let state = State::default();
        Vm {
            registers,
            pc,
            program,
            remainder,
            equal_flag,
            state,
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

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            let result = self.run_once();
            match result {
                Ok(_unit) => self.state = State::Executing,
                Err(Error::ReachedEof) => self.state = State::ReachedEof,
                Err(Error::HaltEncountered) => self.state = State::Halted,
                Err(Error::IllegalOpcode) => self.state = State::Resumed,
                Err(Error::InstructionNotParsed) => self.state = State::Crashed,
            }
            if self.state == State::Halted || self.state == State::ReachedEof {
                break;
            } else if self.state == State::Crashed {
                return Err(Error::InstructionNotParsed);
            }
        }
        Ok(())
    }

    pub fn run_once(&mut self) -> Result<(), Error> {
        self.state = State::Executing;
        self.execute_instruction()
    }

    fn execute_instruction(&mut self) -> Result<(), Error> {
        if self.pc >= self.program.len() {
            self.state = State::ReachedEof;
            return Err(Error::ReachedEof);
        }
        let decoded = self.decode_opcode();
        #[rustfmt::skip]
        match decoded {
            Opcode::PAD     => self.pad(),
            Opcode::HALT    => return self.halt(),
            Opcode::LOAD    => self.load(),
            Opcode::ADD     => self.add(),
            Opcode::SUB     => self.sub(),
            Opcode::MUL     => self.mul(),
            Opcode::DIV     => self.div(),
            Opcode::JUMP    => self.jump(),
            Opcode::JF      => self.jf(),
            Opcode::JB      => self.jb(),
            Opcode::EQ      => self.eq(),
            Opcode::NE      => self.ne(),
            Opcode::GT      => self.gt(),
            Opcode::LT      => self.lt(),
            Opcode::GTE     => self.gte(),
            Opcode::LTE     => self.lte(),
            Opcode::JE      => self.je(),
            Opcode::JNE     => self.jne(),
            Opcode::INC     => self.inc(),
            Opcode::DEC     => self.dec(),
            Opcode::ILL     => return self.ill(),
        }
        Ok(())
    }

    fn pad(&mut self) {}

    fn halt(&mut self) -> Result<(), Error> {
        self.pc += 3;
        if IGNORE_HALTED {
            return Ok(());
        }
        Err(Error::HaltEncountered)
    }

    fn load(&mut self) {
        let register = self.next_8bits();
        let number = self.next_16bits();
        self.registers[register as usize] = number as i32;
    }

    fn add(&mut self) {
        let (register0, register1) =
            (self.next_register(), self.next_register());
        self.registers[self.next_8bits() as usize] = register0 + register1;
    }

    fn sub(&mut self) {
        let (register0, register1) =
            (self.next_register(), self.next_register());
        self.registers[self.next_8bits() as usize] = register0 - register1;
    }

    fn mul(&mut self) {
        let (register0, register1) =
            (self.next_register(), self.next_register());
        self.registers[self.next_8bits() as usize] = register0 * register1;
    }

    fn div(&mut self) {
        let (register0, register1) =
            (self.next_register(), self.next_register());
        self.registers[self.next_8bits() as usize] = register0 / register1;
        self.remainder = (register0 % register1) as u32;
    }

    fn jump(&mut self) {
        let target = self.next_register();
        self.next_16bits();
        self.pc = target as usize;
    }

    fn jf(&mut self) {
        let value = self.next_register();
        self.next_16bits();
        self.pc += value as usize;
    }

    fn jb(&mut self) {
        let value = self.next_register();
        self.next_16bits();
        self.pc -= value as usize;
    }

    fn eq(&mut self) {
        let (register0, register1) =
            (self.next_register(), self.next_register());
        self.equal_flag = register0 == register1;
        self.next_8bits();
    }

    fn ne(&mut self) {
        let (register0, register1) =
            (self.next_register(), self.next_register());
        self.equal_flag = register0 != register1;
        self.next_8bits();
    }

    fn gt(&mut self) {
        let (register0, register1) =
            (self.next_register(), self.next_register());
        self.equal_flag = register0 > register1;
        self.next_8bits();
    }

    fn lt(&mut self) {
        let (register0, register1) =
            (self.next_register(), self.next_register());
        self.equal_flag = register0 < register1;
        self.next_8bits();
    }

    fn gte(&mut self) {
        let (register0, register1) =
            (self.next_register(), self.next_register());
        self.equal_flag = register0 >= register1;
        self.next_8bits();
    }

    fn lte(&mut self) {
        let (register0, register1) =
            (self.next_register(), self.next_register());
        self.equal_flag = register0 <= register1;
        self.next_8bits();
    }

    fn je(&mut self) {
        let target = self.next_register();
        self.next_16bits();
        if self.equal_flag {
            self.pc = target as usize;
        }
    }

    fn jne(&mut self) {
        let target = self.next_register();
        self.next_16bits();
        if !self.equal_flag {
            self.pc = target as usize;
        }
    }

    fn inc(&mut self) {
        let register = self.next_8bits();
        self.next_16bits();
        self.registers[register as usize] += 1;
    }

    fn dec(&mut self) {
        let register = self.next_8bits();
        self.next_16bits();
        self.registers[register as usize] -= 1;
    }

    fn ill(&mut self) -> Result<(), Error> {
        self.pc += 3;
        if IGNORE_ILLEGAL {
            return Ok(());
        }
        Err(Error::IllegalOpcode)
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
        let result = test_vm.run_once();
        assert!(result.is_err());
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_ill() {
        let program = vec![200, 0, 0, 0];
        let mut test_vm = get_test_vm(None, None, program);
        let result = test_vm.run_once();
        assert!(result.is_err());
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_load() {
        // Remember, this is how we represent `500` using two `u8`s in little
        // endian format: `1, 244`
        let program = vec![2, 0, 1, 244];
        let mut test_vm = get_test_vm(None, None, program);
        let result = test_vm.run_once();
        assert!(result.is_ok());
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let fill_registers = Some(vec![(1, 15), (2, 10)]);
        let program = vec![3, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        let result = test_vm.run_once();
        assert!(result.is_ok());
        assert_eq!(test_vm.registers[0], 25);
    }

    #[test]
    fn test_opcode_sub() {
        let fill_registers = Some(vec![(1, 15), (2, 10)]);
        let program = vec![4, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        let result = test_vm.run_once();
        assert!(result.is_ok());
        assert_eq!(test_vm.registers[0], 5);
    }

    #[test]
    fn test_opcode_mul() {
        let fill_registers = Some(vec![(1, 4), (2, 6)]);
        let program = vec![5, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        let result = test_vm.run_once();
        assert!(result.is_ok());
        assert_eq!(test_vm.registers[0], 24);
    }

    #[test]
    fn test_opcode_div() {
        let fill_registers = Some(vec![(1, 8), (2, 5)]);
        let program = vec![6, 1, 2, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        let result = test_vm.run_once();
        assert!(result.is_ok());
        assert_eq!(test_vm.registers[0], 1);
        assert_eq!(test_vm.remainder, 3)
    }

    #[test]
    fn test_opcode_jump() {
        let fill_registers = Some(vec![(0, 4)]);
        let program = vec![7, 0, 0, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        let result = test_vm.run_once();
        assert!(result.is_ok());
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jf() {
        let fill_registers = Some(vec![(0, 4)]);
        let program = vec![8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut test_vm = get_test_vm(fill_registers, None, program);
        let result = test_vm.run_once();
        assert!(result.is_ok());
        assert_eq!(test_vm.pc, 8);
    }

    #[test]
    fn test_opcode_jb() {
        let fill_registers = Some(vec![(0, 8)]);
        let pc = Some(4);
        let program = vec![0, 0, 0, 0, 9, 0, 0, 0];
        let mut test_vm = get_test_vm(fill_registers, pc, program);
        let result = test_vm.run_once();
        assert!(result.is_ok());
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_eq() {
        let fill_registers0 = Some(vec![(1, 32), (2, 32)]);
        let program0 = vec![10, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        let result0 = test_vm0.run_once();
        assert!(result0.is_ok());
        assert!(test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 52), (2, 38)]);
        let program1 = vec![10, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        let result1 = test_vm1.run_once();
        assert!(result1.is_ok());
        assert!(!test_vm1.equal_flag);
    }

    #[test]
    fn test_opcode_ne() {
        let fill_registers0 = Some(vec![(1, 32), (2, 32)]);
        let program0 = vec![11, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        let result0 = test_vm0.run_once();
        assert!(result0.is_ok());
        assert!(!test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 52), (2, 38)]);
        let program1 = vec![11, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        let result1 = test_vm1.run_once();
        assert!(result1.is_ok());
        assert!(test_vm1.equal_flag);
    }

    #[test]
    fn test_opcode_gt() {
        let fill_registers0 = Some(vec![(1, 8), (2, 5)]);
        let program0 = vec![12, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        let result0 = test_vm0.run_once();
        assert!(result0.is_ok());
        assert!(test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 5), (2, 8)]);
        let program1 = vec![12, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        let result1 = test_vm1.run_once();
        assert!(result1.is_ok());
        assert!(!test_vm1.equal_flag);
    }

    #[test]
    fn test_opcode_lt() {
        let fill_registers0 = Some(vec![(1, 8), (2, 5)]);
        let program0 = vec![13, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        let result0 = test_vm0.run_once();
        assert!(result0.is_ok());
        assert!(!test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 5), (2, 8)]);
        let program1 = vec![13, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        let result1 = test_vm1.run_once();
        assert!(result1.is_ok());
        assert!(test_vm1.equal_flag);
    }

    #[test]
    fn test_opcode_gte() {
        let fill_registers0 = Some(vec![(1, 77), (2, 64)]);
        let program0 = vec![14, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        let result0 = test_vm0.run_once();
        assert!(result0.is_ok());
        assert!(test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 21), (2, 21)]);
        let program1 = vec![14, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        let result1 = test_vm1.run_once();
        assert!(result1.is_ok());
        assert!(test_vm1.equal_flag);

        let fill_registers2 = Some(vec![(1, 8), (2, 64)]);
        let program2 = vec![14, 1, 2, 0];
        let mut test_vm2 = get_test_vm(fill_registers2, None, program2);
        let result2 = test_vm2.run_once();
        assert!(result2.is_ok());
        assert!(!test_vm2.equal_flag);
    }

    #[test]
    fn test_opcode_lte() {
        let fill_registers0 = Some(vec![(1, 77), (2, 64)]);
        let program0 = vec![15, 1, 2, 0];
        let mut test_vm0 = get_test_vm(fill_registers0, None, program0);
        let result0 = test_vm0.run_once();
        assert!(result0.is_ok());
        assert!(!test_vm0.equal_flag);

        let fill_registers1 = Some(vec![(1, 21), (2, 21)]);
        let program1 = vec![15, 1, 2, 0];
        let mut test_vm1 = get_test_vm(fill_registers1, None, program1);
        let result1 = test_vm1.run_once();
        assert!(result1.is_ok());
        assert!(test_vm1.equal_flag);

        let fill_registers2 = Some(vec![(1, 8), (2, 64)]);
        let program2 = vec![15, 1, 2, 0];
        let mut test_vm2 = get_test_vm(fill_registers2, None, program2);
        let result2 = test_vm2.run_once();
        assert!(result2.is_ok());
        assert!(test_vm2.equal_flag);
    }

    #[test]
    fn test_opcode_je() {
        let pc0 = Some(4);
        let program0 = vec![0, 0, 0, 0, 16, 0, 0, 0];
        let mut test_vm0 = get_test_vm(None, pc0, program0);
        test_vm0.equal_flag = true;
        let result0 = test_vm0.run_once();
        assert!(result0.is_ok());
        assert_eq!(test_vm0.pc, 0);

        let pc1 = Some(4);
        let program1 = vec![0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0];
        let mut test_vm1 = get_test_vm(None, pc1, program1);
        let result1 = test_vm1.run_once();
        assert!(result1.is_ok());
        assert_eq!(test_vm1.pc, 8);
    }

    #[test]
    fn test_opcode_jne() {
        let pc0 = Some(4);
        let program0 = vec![0, 0, 0, 0, 17, 0, 0, 0];
        let mut test_vm0 = get_test_vm(None, pc0, program0);
        let result0 = test_vm0.run_once();
        assert!(result0.is_ok());
        assert_eq!(test_vm0.pc, 0);

        let pc1 = Some(4);
        let program1 = vec![0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0];
        let mut test_vm1 = get_test_vm(None, pc1, program1);
        test_vm1.equal_flag = true;
        let result1 = test_vm1.run_once();
        assert!(result1.is_ok());
        assert_eq!(test_vm1.pc, 8);
    }
}
