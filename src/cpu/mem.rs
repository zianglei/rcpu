use std::vec::Vec;
use tui::{
    backend::Backend,
    widgets::{Widget, Block, Borders, BorderType, Paragraph},
    style::{Style, Color, Modifier},
    text::{Spans, Span},
    layout::Rect,
    Frame
};
use super::traits::{Tickable, DebugUI};
use super::reg::Reg;

const MEMSIZE: usize = 0x4_0000;
const KERNBASE: usize = 0x2_0000;
const INSTBASE: u64 = 0x8000_0000;

#[derive(Debug)]
pub struct Mem {
    en: bool,
    wen: bool,
    raddr: u64,
    waddr: u64,
    wdata: u8,
    pub out: Reg<u32>,
    mem: [u8; MEMSIZE],
    curpc: u64,
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            en: true,
            raddr: 0,
            waddr: 0,
            wen: false,
            wdata: 0,
            out: Reg::<u32>::default(),
            mem: [0; MEMSIZE],
            curpc: INSTBASE,
        }
    }

    pub fn load_array(&mut self, addr: u64, bin: &[u8]) -> &Self {
        if addr >= 0x8000_0000 {
            let index = KERNBASE + (addr - 0x8000_0000) as usize;
            self.mem[index..(index as usize+bin.len())].copy_from_slice(bin);
        }
        self
    }
    
    pub fn read(&mut self, addr: u64) {
        if addr >= 0x8000_0000 {
            self.raddr = 0x2_0000 + (addr - 0x8000_0000);
        } else {
            return;
        }
    }

    pub fn write(&mut self, addr: u64, data: u8) {
        if addr >= 0x8000_0000 {
            self.waddr = 0x2_0000 + (addr - 0x8000_0000);
        } else {
            return;
        }
        self.wdata = data;
        self.wen = true;
    }

    fn read_u32(&mut self, addr: u64) -> u32 {
        let mut raddr: usize = addr as usize;
        if addr >= 0x8000_0000 {
           raddr = KERNBASE + (addr - 0x8000_0000) as usize;
        }
        ((self.mem[raddr] as u32)) +
        ((self.mem[raddr+1] as u32) << 8) +
        ((self.mem[raddr+2] as u32) << 16) +
        ((self.mem[raddr+3] as u32) << 24) 
    }
}

impl Tickable for Mem {
    fn tick(&mut self) {
        if self.en {
            if self.wen {
                self.mem[self.waddr as usize] = self.wdata;
            }
            self.wen = false;
            self.out.tick();
            let rdata = self.read_u32(self.raddr);
            self.out.set(rdata);
        } 
    }
}

impl DebugUI for Mem {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let mut text = Vec::<Spans>::new();
        for i in 0..area.height {
            let data = match self.curpc + (i * 4) as u64 - INSTBASE {
                addr if addr > 0 && addr < MEMSIZE as u64 => Some(self.read_u32(addr)),
                _ => None 
            };
            if let Some(inst) = data {
                text.push(Spans::from(format!("{:x}", inst)));
            } 
        }
        let block = Block::default().borders(Borders::ALL).title(Span::styled(
            "Mem",
             Style::default()
                 .fg(Color::Magenta)
                 .add_modifier(Modifier::BOLD),
             ));
        let paragraph = Paragraph::new(text).block(block);
        f.render_widget(paragraph, area);
    } 
}

#[test]
fn test_mem_init() {
    let test = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut mem = Mem::new();
    mem.load_array(0x8000_0000, test);
    for i in 0..8 {
        assert_eq!(mem.mem[(i as u32 + 0x2_0000 as u32) as usize], i);
    }
}

 #[test]
fn test_mem_write() {
    let mut mem = Mem::new();
    mem.write(0x8000_0000, 1);
    mem.tick();
    assert_eq!(mem.mem[0x2_0000], 1);
}

#[test]
fn test_mem_read() {
    let mut mem = Mem::new();
    mem.mem[0x2_0003] = 3;
    mem.read(0x8000_0003);
    assert_eq!(mem.out.get(), 0);
    mem.tick();
    assert_eq!(mem.out.get(), 0);
    mem.tick();
    assert_eq!(mem.out.get(), 3);
}
