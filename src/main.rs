#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(inherent_associated_types)]

pub mod instruction;
pub mod repl;
pub mod types;
pub mod vm;

pub fn main() {
    let mut repl = repl::Repl::new();
    repl.run()
}
