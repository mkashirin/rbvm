use program_parsers::{Program, program_parser};

use crate::opcodes::Opcode;

pub mod instruction_parsers;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod program_parsers;
pub mod symbol_parsers;

#[derive(Debug)]
pub enum AssemblerError {
    LabelNotExtracted,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_index: u8 },
    IntOperand { value: i32 },
    LabelDecl { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}
type MaybeToken = Option<Token>;

#[derive(Debug, Default)]
pub struct Assembler {
    phase: AssemblerPhase,
    symbol_table: SymbolTable,
}
impl Assembler {
    pub fn assemble(&mut self, source_code: &str) -> Option<Vec<u8>> {
        match program_parser(source_code) {
            Ok((_, program)) => {
                self.process_first_phase(&program);
                Some(self.process_second_phase(&program))
            }
            Err(err) => {
                println!("There was an error assembling the code: {:?}", err);
                None
            }
        }
    }

    fn process_first_phase(&mut self, program: &Program) {
        self.extract_labels(program);
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, program: &Program) -> Vec<u8> {
        let mut bytecode = vec![];
        for instr in program.instrs.iter() {
            let mut bytes = instr.to_bytes();
            bytecode.append(&mut bytes);
        }
        bytecode
    }

    fn extract_labels(&mut self, program: &Program) {
        let mut offset = 0;
        for instr in program.instrs.iter() {
            if let Ok(name) = instr.try_exrtact_label() {
                let symbol = Symbol::new(name, SymbolType::Label, offset);
                self.symbol_table.push_symbol(symbol);
            }
            offset += 4;
        }
    }
}

#[derive(Debug, Default)]
pub enum AssemblerPhase {
    #[default]
    First,
    Second,
}

// type SymbolTable = HashMap<String, Symbol>;
#[derive(Debug, Default)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}
impl SymbolTable {
    pub fn push_symbol(&mut self, value: Symbol) {
        self.symbols.push(value);
    }

    pub fn symbol_offset(&self, name: &str) -> Option<usize> {
        self.symbols
            .iter()
            .find(|symbol| symbol.name.eq(name))
            .map(|symbol| symbol.offset)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
    offset: usize,
}
impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType, offset: usize) -> Self {
        Self {
            name,
            symbol_type,
            offset,
        }
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
    Directive,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::Vm;

    #[test]
    fn test_symbol_table() {
        let mut table = SymbolTable::default();
        let new_symbol = Symbol::new("test".to_string(), SymbolType::Label, 12);
        table.push_symbol(new_symbol);
        assert_eq!(table.symbols.len(), 1);

        let search0 = table.symbol_offset("test");
        assert!(search0.is_some());
        let value = search0.unwrap();
        assert_eq!(value, 12);

        let search1 = table.symbol_offset("nonexistent");
        assert!(search1.is_none());
    }

    #[test]
    fn test_assemble_program() {
        let mut assembler = Assembler::default();
        let program = r#"load $0 #100
load $1 #1
load $2 #0
test: inc $0
neq $0 $2
jump @test
add $1 $2 $3
halt
        "#;
        let assembled = assembler.assemble(program).unwrap();
        let mut vm = Vm::default();
        assert_eq!(assembled.len(), 32);
        vm.push_bytes(assembled);
        assert_eq!(vm.program.len(), 32);
    }
}
