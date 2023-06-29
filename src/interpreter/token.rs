//! The 8 Brainfuck instructions represented as an enum
use crate::error::*;

/// The 8 Brainfuck instructions represented as an enum
#[derive(Clone, Copy, Debug)]
pub enum Token {
    Enc,
    Dec,
    MoveLeft,
    MoveRight,
    Output,
    Input,
    BlockOpen,
    BlockClose,
}

/// Convert a string into a set of tokens
impl TryFrom<char> for Token {
    type Error = BrainFuckError;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '+' => Ok(Token::Enc),
            '-' => Ok(Token::Dec),
            '<' => Ok(Token::MoveLeft),
            '>' => Ok(Token::MoveRight),
            '.' => Ok(Token::Output),
            ',' => Ok(Token::Input),
            '[' => Ok(Token::BlockOpen),
            ']' => Ok(Token::BlockClose),
            _ => Err(BrainFuckError::BadToken(value)),
        }
    }
}

/// Shorthand for a set of tokens.
/// This makes it possible to use try_from.
#[derive(Debug, Clone)]
pub struct Tokens(pub Vec<Token>);

impl Tokens {
    /// Verify that all '[' and ']' tokens are properly matched
    pub fn verify_blocks(&self) -> Result<usize> {
        let mut subs = 0;
        let mut enc = 0;
        for token in &self.0 {
            match token {
                Token::BlockOpen => enc += 1,
                Token::BlockClose => {
                    if enc == 0 {
                        return Err(Box::new(BrainFuckError::FailedToCompile));
                    };
                    enc -= 1;
                    subs += 1;
                }
                _ => {} // Ignore the rest
            };
        }
        match enc {
            0 => Ok(subs),
            _ => Err(Box::new(BrainFuckError::FailedToCompile)),
        }
    }
}

/// Tokenize input.
/// This will ignore anything that is not a valid
/// Brainfuck instruction.
impl TryFrom<&str> for Tokens {
    type Error = Box<BrainFuckError>;
    fn try_from(input: &str) -> std::result::Result<Self, Self::Error> {
        let mut tokens: Vec<Token> = Vec::new();

        for c in input.chars() {
            if let Ok(token) = Token::try_from(c) {
                tokens.push(token);
            }
        }
        Ok(Self(tokens))
    }
}
