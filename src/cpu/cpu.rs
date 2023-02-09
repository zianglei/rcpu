use std::{vec::Vec, error::Error, io};
use super::reg::Reg;
use super::mem::Mem;
use super::traits::DebugUI;
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{ 
    backend::{Backend, TermionBackend},
    layout::{Direction, Layout, Constraint, Rect},
    Frame, Terminal
};


type CPUResult = Result<(), Box<dyn Error>>;

#[derive(Debug)]
pub struct RegFile<T> {
    a0: Reg<T>,
}

impl<T> RegFile<T> where T: Copy + Default{
    pub fn new() -> RegFile<T> {
        RegFile {
            a0: Reg::<T>::new(),
        }
    }

    pub fn get(&self, name: &str) -> T {
       self.a0.get() 
    }
}

#[derive(Debug)]
pub struct CPU {
    regs: RegFile<u64>,
    mem: Mem,
}

impl CPU {
    pub fn load_elf(&mut self, bin: &Vec<u8>) -> &Self {
        self.mem.load_array(0, bin);
        self
    } 

    pub fn run(&self) -> &RegFile<u64> {
        &self.regs        
    }

    pub fn new() -> CPU {
        CPU {
            regs: RegFile::new(),
            mem: Mem::new()
        }
    }
}

impl CPU {
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .direction(Direction::Horizontal)
            .split(f.size());
        self.mem.draw(f, chunks[1]); 
    }

    pub fn run_tui(&mut self) -> CPUResult {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| self.draw(f));
        }


        Ok(())
 
    } 
}

