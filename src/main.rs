#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(inherent_associated_types)]
#![feature(type_alias_impl_trait)]

pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod types;
pub mod vm;

pub fn main() {
    let mut repl = repl::Repl::new();
    repl.run()
}
