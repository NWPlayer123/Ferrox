use crate::format::Segment;

//use super::{Disassembler, Processor};
use ppc750cl::Ins;

struct PPC750CL;

impl PPC750CL {
    fn init(segments: &[Segment<u32>], entrypoint: u32) -> Self {
        Self {}
    }

    fn step(&mut self) {

    }
}