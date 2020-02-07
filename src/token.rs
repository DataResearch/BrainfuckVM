#[derive(Debug)]
pub struct JumpIndex {
    jump_stack_index: usize,
}

impl JumpIndex {
    fn new(index: usize) -> JumpIndex {
        JumpIndex {
            jump_stack_index: index
        }
    }
}

impl JumpIndex {
    pub fn read(&self) -> usize {
        self.jump_stack_index
    }
}

#[derive(Debug)]
pub enum Token {
    IncrementCell,
    DecrementCell,
    /// Jumps to the corresponding ] element, if the current cell value is '0'
    JumpForward(JumpIndex),
    /// Jumps back after the corresponding [ element, if the current cell is non '0'
    JumpBackward(JumpIndex),
    MoveNextMemoryCell,
    MovePreviousMemoryCell,
    ReadToCell,
    PrintFromCell
}

pub fn tokenize(code: &str) -> Vec<Token> {

    let mut tokens = Vec::<Token>::new();
    let mut jump_stack_counter = 0usize;

    for x in code.chars() {
        let token = match x {
            '+' => Token::IncrementCell,
            '-' => Token::DecrementCell,
            '>' => Token::MoveNextMemoryCell, 
            '<' => Token::MovePreviousMemoryCell,
            '[' =>  {
                jump_stack_counter += 1;
                Token::JumpForward(JumpIndex::new(jump_stack_counter))
            },
            ']' => {
                let value = jump_stack_counter;
                jump_stack_counter -= 1;
                Token::JumpBackward(JumpIndex::new(value))
            },
            '.' => Token::PrintFromCell,
            ',' => Token::ReadToCell,
            _ => continue,
        };

        tokens.push(token);
    }

    if jump_stack_counter != 0 {
        panic!("ERROR: jumps doesn't match - missmatch of open and closing jumps instructions");
    }

    tokens
}
