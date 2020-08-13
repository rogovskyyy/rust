mod memory;
fn main()  {
    let mut memory = memory::Memory::new();
    let a = memory.read(10);
    println!("{}", a);
    memory.write(10, 0xb);
    let b = memory.read(10);
    println!("{}", b);
    
}