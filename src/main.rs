use rcpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    if let Err(e) = cpu.run_tui() {
        eprintln!("{}", e);
        std::process::exit(1);
    }    
}
