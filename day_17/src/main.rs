use std::fs;

#[derive(Debug)]
struct Computer {
    // registers
    a: i32,
    b: i32,
    c: i32,
    ip: usize,
    output: Vec<i32>,
    program: Vec<i32>,
}

impl Computer {
    fn new(a: i32, b: i32, c: i32, program: Vec<i32>) -> Computer {
        Computer {
            a,
            b,
            c,
            program,
            ip: 0,
            output: Vec::new(),
        }
    }

    fn combo(&self, operand: i32) -> i32 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand"),
        }
    }

    fn out(&mut self, value: i32) {
        self.output.push(value);
    }

    fn div(&self, a: i32, b: i32) -> i32 {
        let mut b = b;
        let mut result = a;
        while b > 0 && result > 0 {
            result /=2;
            b -= 1;
        }
        result
    }
    

    fn step(&mut self) -> bool {
        if self.ip >= self.program.len() {
            return false;
        }
        let opcode = self.program[self.ip as usize];
        let operand = self.program[self.ip as usize + 1];
        // println!("State before: {:?}", self);
        match opcode {
            0 => {
                let result = self.div(self.a, self.combo(operand));
                self.a = result;
                self.ip += 2;
            }
            1 => {
                let result = self.b ^ operand;
                self.b = result;
                self.ip += 2;
            }
            2 => {
                let result = self.combo(operand) % 8;
                self.b = result;
                self.ip += 2;
            }
            3 => {
                if self.a == 0 {
                    self.ip += 2;
                } else {
                    self.ip = operand as usize;
                }
            }
            4 => {
                let result = self.b ^ self.c;
                self.b = result;
                self.ip += 2;
            }
            5 => {
                let result = self.combo(operand) % 8;
                self.out(result);
                self.ip += 2;
            }
            6 => {
                let result = self.div(self.a, self.combo(operand));
                self.b = result;
                self.ip += 2;
            }
            7 => {
                let result = self.div(self.a, self.combo(operand));
                self.c = result;
                self.ip += 2;
            }
            _ => panic!("Invalid opcode"),
        }
        // println!("State after: {:?}\n", self);
        true
    }

    fn run(&mut self) {
        let mut c = 0;
        while self.step() {
            c += 1;
            print!(".");
            if c % 80 == 0 {
                println!();
            }
        }
        println!();
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<_> = contents.lines().collect();
    assert!(lines.len() == 5);
    let a = &lines[0][11..].trim().parse::<i32>().unwrap();
    let b = &lines[1][11..].trim().parse::<i32>().unwrap();
    let c = &lines[2][11..].trim().parse::<i32>().unwrap();
    let program = lines[4][9..]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut computer = Computer::new(*a, *b, *c, program);
    computer.run();
    println!("Output:\n{}", computer.output.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
}
