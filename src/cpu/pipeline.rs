use super::{reg::Reg, mem::Mem, traits::Tickable};
use super::CPU;

#[derive(Debug)]
pub struct Fetch {
    pc: Reg<u64>,
    imem: Mem,
    inst: Reg<u32>,
    newpc: Option<u64>,
}

impl Fetch {
    pub fn new(pc: u64) -> Fetch {
        Fetch {
            pc: Reg::<u64>::new(pc - 4),
            imem: Mem::new(),
            inst: Reg::<u32>::default(),
            newpc: None,
        }
    }

    pub fn run(&mut self) {
        self.imem.read(self.pc.get());
        self.inst.set(self.imem.out.get());
        match self.newpc {
            Some(val) => self.pc.set(val),
            None => self.pc.set(self.pc.get() + 4)
        }
        self.newpc = None;
    }

    pub fn set_newpc(&mut self, newpc: u64) {
        self.newpc = Some(newpc);
    }
}

impl Tickable for Fetch {
    fn tick(&mut self) {
        self.pc.tick();
        self.inst.tick();
        self.imem.tick();
    }
}

struct Decode {

}

impl Decode {

}

struct Execute {

}

impl Execute {

}

struct MemAccess {

}

impl MemAccess {

}

struct WriteBack {

}

impl WriteBack {

}
