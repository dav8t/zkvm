fn main() {
    let mut vm = stackmachine::new();
    let prog = vec![3, 4, 0x40000001, 0x40000000];
    vm.load_program(prog);
    vm.run();
}
