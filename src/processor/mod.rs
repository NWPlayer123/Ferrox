//! This module contains a list of processors, which is defined by the ability to disassemble a given group of
//! bytes, the ability to step through a list of instructions, and any associated rules or quirks specific to
//! that processor.

trait Processor {
    // TODO: does this need to be impl for our use case?
    fn disassembler() -> impl Disassembler;
}

trait Disassembler {
    type Output;

    fn disassemble() -> Self::Output;
}

pub mod ppc750cl;