use std::error::Error;
use std::fs;
use rcpu::CPU;

type TestResult = Result<(), Box<dyn Error>>;

fn run_isa_elf(isa: &str, inst: &str) -> TestResult {
    let elf_filename = format!("{}-p-{}", isa, inst);
    let elf_bin = fs::read(elf_filename)?;
    let mut cpu = CPU::new();
    let regs = cpu.load_elf(&elf_bin)
        .run();
    assert_eq!(regs.get("a0"), 0);
    Ok(())
}

#[test]
fn rv64ui_lb() -> TestResult {
    run_isa_elf("rv64ui", "lb")
}

#[test]
fn rv64ui_lbu() -> TestResult {
    run_isa_elf("rv64ui", "lbu")
}





