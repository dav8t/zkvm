use std::vec::Vec;

#[derive(Debug)]
pub struct n00bvm {
    pc: i32,
    sp: i32,
    memory: Vec<i32>,
    typ: i32,
    dat: i32,
    running: bool,
}

impl n00bvm {
    pub fn new() -> Self {
        StackVM {
            pc: 0,
            sp: 0,
            memory: Vec::with_capacity(1000000),
            typ: 0,
            dat: 0,
            running: true,
        }
    }

    pub fn run(&mut self) {
        self.pc -= 1;
        while self.running {
            self.fetch();
            self.decode();
            self.execute();
        }
    }

    pub fn load_program(&mut self, prog: Vec<i32>) {
        self.memory.extend(prog);
    }

    fn fetch(&mut self) {
        self.typ = self.memory[self.pc];
        self.dat = self.memory[self.pc + 1];
        self.pc += 2;
    }

    fn decode(&mut self) {
        self.typ = (self.typ >> 30) as i32;
        self.dat = self.dat & 0x3fffffff;
    }

    fn execute(&mut self) {
        if self.typ == 0 || self.typ == 2 {
            self.sp += 1;
            self.memory[self.sp] = self.dat;
        } else {
            self.do_primitive();
        }
    }

    fn do_primitive(&mut self) {
        match self.dat {
            0 => {
                println!("halt");
                self.running = false;
            }
            1 => {
                println!("add {} {}", self.memory[self.sp - 1], self.memory[self.sp]);
                self.memory[self.sp - 1] = self.memory[self.sp - 1] + self.memory[self.sp];
            }
            _ => panic!("Unknown primitive instruction: {}", self.dat),
        }
    }
}
