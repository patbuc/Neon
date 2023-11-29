use crate::compiler::Compiler;
use crate::vm::opcodes::OpCode;
use crate::vm::{Block, Result, Value, VirtualMachine};

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: String) -> Result {
        self.reset();

        let mut compiler = Compiler::new();
        let block = compiler.compile(source);
        return if let Some(block) = block {
            self.run(block)
        } else {
            Result::CompileError
        };
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.stack.clear();
    }

    #[inline(always)]
    fn run(&mut self, mut block: Block) -> Result {
        loop {
            // #[cfg(feature = "disassemble")]
            // block.disassemble_instruction(self.ip);
            match OpCode::from_u8(block.read_u8(self.ip)) {
                OpCode::Return => {
                    let value = self.pop();
                    VirtualMachine::print(value);
                    return Result::Ok;
                }
                OpCode::Constant => {
                    let constant_index = block.read_u8(self.ip + 1) as usize;
                    let constant = block.read_constant(constant_index);
                    self.push(constant);
                    self.ip += 1;
                }
                OpCode::Constant2 => {
                    let constant_index = block.read_u16(self.ip + 1) as usize;
                    let constant = block.read_constant(constant_index);
                    self.push(constant);
                    self.ip += 2;
                }
                OpCode::Constant4 => {
                    let constant_index = block.read_u32(self.ip + 1) as usize;
                    let constant = block.read_constant(constant_index);
                    self.push(constant);
                    self.ip += 4;
                }
                OpCode::Negate => {
                    if let Value::Number(..) = self.peek(0) {
                        self.runtime_error("Operand must be a number", block);
                        return Result::RuntimeError;
                    }
                    let value = self.pop();
                    self.push(number!(-as_number!(value)));
                }
                OpCode::Add => self.addition(),
                OpCode::Subtract => self.subtraction(),
                OpCode::Multiply => self.multiplication(),
                OpCode::Divide => self.division(),
                OpCode::Nil => {
                    self.push(nil!());
                }
                OpCode::True => {
                    self.push(boolean!(true));
                }
                OpCode::False => {
                    self.push(boolean!(false));
                }
            }
            self.ip += 1;
        }
    }

    fn addition(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a) + as_number!(b)));
    }

    fn subtraction(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a) - as_number!(b)));
    }

    fn multiplication(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a) * as_number!(b)));
    }

    fn division(&mut self) {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::Number(as_number!(a) / as_number!(b)));
    }

    fn print(value: Value) {
        pub(crate) fn print_nil() {
            print!("nil")
        }
        pub(crate) fn print_string(value: String) {
            print!("{}", value)
        }
        pub(crate) fn print_bool(value: bool) {
            print!("{}", value);
        }
        pub(crate) fn print_number(value: f64) {
            print!("{}", value);
        }

        match value {
            Value::Number(val) => print_number(val),
            Value::Boolean(val) => print_bool(val),
            Value::String(val) => print_string(val),
            Value::Nil => print_nil(),
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn peek(&mut self, distance: usize) -> Value {
        self.stack[self.stack.len() - 1 - distance].clone()
    }
    fn runtime_error(&mut self, error: &str, block: Block) {
        eprint!("{} ", error);
        let line = block.get_line(self.ip).unwrap();
        eprintln!("[line {}] in script", line);
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::Value;

    #[test]
    fn can_create_vm() {
        let vm = super::VirtualMachine::new();
        assert_eq!(0, vm.ip);
        assert_eq!(0, vm.stack.len());
    }

    #[test]
    fn can_execute_simple_arithmetics() {
        let mut block = super::Block::new("ZeBlock");

        block.write_constant(number!(1.0), 0);
        block.write_constant(number!(2.0), 0);
        block.write_op_code(super::OpCode::Add, 0);
        block.write_constant(number!(3.0), 0);
        block.write_op_code(super::OpCode::Multiply, 0);
        block.write_constant(number!(2.0), 0);
        block.write_op_code(super::OpCode::Subtract, 0);
        block.write_constant(number!(2.0), 0);
        block.write_op_code(super::OpCode::Divide, 0);

        // Pushing throw away value to the stack.
        // This is needed because the Return OpCode will pop a value from the stack and print it.
        block.write_constant(number!(0.0), 0);
        block.write_op_code(super::OpCode::Return, 0);

        let mut vm = super::VirtualMachine {
            ip: 0,
            stack: Vec::new(),
        };

        let result = vm.run(block);
        assert_eq!(super::Result::Ok, result);
        assert_eq!(3.5, as_number!(vm.pop()));
    }
}
