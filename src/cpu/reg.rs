use super::traits::Tickable;

#[derive(Debug)]
pub struct Reg<T> {
    prev: T,
    next: T,
    enable: bool,
}

impl<T: Default + Copy> Reg<T> {
    pub fn default() -> Reg<T> {
       Reg {
            enable: true,
            prev: T::default(),
            next: T::default(),
        }
    }

    pub fn new(init: T) -> Reg<T> {
        Reg {
            enable: true,
            prev: init,
            next: T::default(),
        }
    }

    pub fn get(&self) -> T {
        return self.prev;
    }

    pub fn set(&mut self, data: T) {
        self.next = data; 
    }

    pub fn enable(&mut self) {
        self.enable = true;
    }

    pub fn disable(&mut self) {
        self.enable = false;
    }
}

impl<T> Tickable for Reg<T> 
where T: Copy {
    fn tick(&mut self) {
        if self.enable {
            self.prev = self.next;
        }
    }
}

#[test]
fn test_reg_default() {
    let reg = Reg::<u64>::new(0);
    assert_eq!(reg.get(), 0 as u64);
    let reg = Reg::<u64>::new(0xffffffffffffffff);
    assert_eq!(reg.get(), 0xffffffffffffffff);
}

#[test]
fn test_reg_tick() {
    let mut reg = Reg::<u64>::default();
    reg.set(32);
    assert_eq!(reg.get(), 0);
    reg.tick();
    assert_eq!(reg.get(), 32); 
}

#[test]
fn test_reg_disable() {
    let mut reg = Reg::<u64>::default();
    reg.set(32);
    reg.disable();
    reg.tick();
    assert_eq!(reg.get(), 0);
}

#[test]
fn test_reg_enable() {
    let mut reg = Reg::<u64>::default();
    reg.set(32);
    reg.enable();
    reg.tick();
    assert_eq!(reg.get(), 32);
}
