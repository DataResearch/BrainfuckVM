use super::token;

pub struct VM<'a> {
    /// Memory for code that will be executed
    code: &'a [token::Token],
    /// Memory for the VM
    memory: [u32; 1024],
    /// Instruction Pointer; keeps track where the next token will be read from for execution
    eip: usize,
    /// Data Pointer; keeps track where the VM will manipulate the memory
    edx: usize,
}

impl<'a> VM<'a> {

    pub fn new(code: &'a [token::Token]) -> VM {
        VM {
            code: code,
            memory: [0; 1024],
            eip: 0,
            edx: 0
        }
    }

    pub fn execute(&mut self) {

        while self.eip != self.code.len() {
            match &self.code[self.eip] {
                token::Token::IncrementCell => self.increment_cell(),
                token::Token::DecrementCell => self.decrement_cell(),
                token::Token::JumpForward(ji) => {
                    if self.memory[self.edx] == 0 {
                        let target = self.find_previous_jump_marker(ji);
                        self.jump(target + 1);
                    }
                },
                token::Token::JumpBackward(ji) => {
                    if self.memory[self.edx] != 0 {
                        let target = self.find_next_jump_marker(ji);
                        self.jump(target + 1);
                    }
                },
                token::Token::MoveNextMemoryCell => self.move_next_memory_cell(),
                token::Token::MovePreviousMemoryCell => self.move_previous_memory_cell(),
                token::Token::ReadToCell => (),
                token::Token::PrintFromCell => self.print_current_memory_cell(),
            };

            self.eip += 1;
        }

    }

    pub fn dump(&self) {
        for x in 0..1024 {
            print!("{} ", self.memory[x as usize]);
        }
    }
}

impl<'a> VM<'a> {

    fn move_next_memory_cell(&mut self) {
        self.edx += 1;
    }

    fn move_previous_memory_cell(&mut self) {
        self.edx -= 1;
    }

    fn jump(&mut self, eip: usize) {
        self.eip = eip;
    }

    fn increment_cell(&mut self) {
        self.memory[self.edx] += 1;
    }

    fn decrement_cell(&mut self) {
        self.memory[self.edx] -= 1;
    }

    fn find_previous_jump_marker(&self, ji: &token::JumpIndex) -> usize {
        let mut current_search_eip = self.eip;
        current_search_eip -= 1;

        while current_search_eip > 0 {

            match &self.code[current_search_eip] {
                token::Token::JumpForward(index) => {
                    if index.read() == ji.read() {
                        return current_search_eip;
                    }
                },
                _ => continue
            }

            current_search_eip -= 1;
        }

        0
    }

    fn find_next_jump_marker(&self, ji: &token::JumpIndex) -> usize {
        let mut current_search_eip = self.eip;
        current_search_eip += 1;

        while current_search_eip < (self.code.len() - 1) {

            match &self.code[current_search_eip] {
                token::Token::JumpForward(index) => {
                    if index.read() == ji.read() {
                        return current_search_eip;
                    }
                },
                _ => continue
            }

            current_search_eip += 1;
        }

        0
    }

    fn print_current_memory_cell(&self) {
        let value = self.memory[self.edx];
        let codepoint = std::char::from_u32(value).unwrap();
        println!("{}", codepoint);
    }
}
