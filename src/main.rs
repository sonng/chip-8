mod cpu;

fn main() {
    let mut c = cpu::CPU::new();

    let mem: [u8; 3176] = [0; 3176];

    c.run();

    println!("Hello, world!");
}
