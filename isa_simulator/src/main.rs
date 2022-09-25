use mylib::simulator;

fn main() {
    let mut reg: [i32; 32] = [0; 32];
    let mut mem: [u8; 1048576] = [0; 1048576];
    let file = String::from("instruction_tests/test_and.bin");
    let len = simulator::read_bytes_to_mem(&file, &mut mem);
    simulator::simulate(&mut reg, &mut mem, &len);
    simulator::print_registers(&reg);
}