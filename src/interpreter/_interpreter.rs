use super::{
    builder::InterpreterBuilder,
    token::{Token, Tokens},
};
use crate::error::{BrainFuckError, Result};
use std::io::prelude::*;
use std::io::Write;

pub struct Interpreter<'a> {
    tokens: Tokens,
    data_ptr: usize,
    data: Vec<u8>,
    inbuf: &'a mut dyn Read,
    outbuf: &'a mut dyn Write,
}

impl<'a> Interpreter<'a> {
    pub fn builder() -> InterpreterBuilder<'a> {
        InterpreterBuilder::new()
    }

    pub fn new(
        inbuf: &'a mut dyn Read,
        outbuf: &'a mut dyn Write,
        tokens: Tokens,
        data: Vec<u8>,
    ) -> Self {
        Self {
            tokens,
            data,
            data_ptr: 0,
            inbuf,
            outbuf,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.tokens.verify_blocks()?;
        let mut ins_ptr = 0;

        while ins_ptr < self.tokens.0.len() {
            let token = self.tokens.0[ins_ptr];
            match token {
                Token::Enc => self.enc_data()?,
                Token::Dec => self.dec_data()?,
                Token::MoveLeft => self.shift_left()?,
                Token::MoveRight => self.shift_right()?,
                Token::Output => {
                    let buf = vec![self.data[self.data_ptr]];
                    self.outbuf.write_all(&buf)?;
                }
                Token::Input => {
                    let mut buf: [u8; 1] = [0];
                    self.inbuf.read_exact(&mut buf)?;
                    self.data[self.data_ptr] = buf[0];
                }
                Token::BlockOpen => {
                    if self.data[self.data_ptr] == 0 {
                        let end = self.find_next_matching(ins_ptr + 1)?;
                        ins_ptr = end;
                    }
                }
                Token::BlockClose => {
                    if self.data[self.data_ptr] > 0 {
                        let start = self.find_prev_matching(ins_ptr - 1)?;
                        ins_ptr = start;
                    }
                }
            };

            // Advance the instruction pointer to the next token
            ins_ptr += 1;
        }
        Ok(())
    }

    /// Encrement the data in the current cell.  If the data is already at
    /// max, return an error
    fn enc_data(&mut self) -> Result<()> {
        if self.data[self.data_ptr] == std::u8::MAX {
            return Err(Box::new(BrainFuckError::DataOutOfRange(self.data_ptr)));
        }
        self.data[self.data_ptr] += 1;
        Ok(())
    }

    /// Encrement the data in the current cell.  If the data is already at
    /// max, return an error
    fn dec_data(&mut self) -> Result<()> {
        if self.data[self.data_ptr] == 0 {
            return Err(Box::new(BrainFuckError::DataOutOfRange(self.data_ptr)));
        }
        self.data[self.data_ptr] -= 1;
        Ok(())
    }

    /// Move the data pointer to the previous cell.  If the
    /// data pointer is already at the first cell, return an error.
    fn shift_left(&mut self) -> Result<()> {
        if self.data_ptr == 0 {
            return Err(Box::new(BrainFuckError::DataPointerOutOfRange));
        }
        self.data_ptr -= 1;
        Ok(())
    }

    /// Move the data pointer to the next cell.  If this causes an
    /// overrun, return an error.
    fn shift_right(&mut self) -> Result<()> {
        self.data_ptr += 1;
        if self.data_ptr == self.data.len() {
            return Err(Box::new(BrainFuckError::DataPointerOutOfRange));
        }
        Ok(())
    }
    /// Given a opening brace: '[', find the matching closing brace
    /// from should be advanced before calling find_next_matching.
    /// Example:
    /// for "+[+].", the opening brace is at 1.  So from must be 2.
    /// And the result will be Ok(3)
    fn find_next_matching(&self, from: usize) -> Result<usize> {
        let mut enc = 0;
        let tokens = &self.tokens.0;
        //let mut matching = from + 1;
        for (i, token) in tokens.iter().skip(from).enumerate() {
            match token {
                Token::BlockClose => {
                    if enc == 0 {
                        return Ok(from + i);
                    }
                    enc -= 1;
                }
                Token::BlockOpen => enc += 1,
                _ => {}
            };
        }
        Err(Box::new(BrainFuckError::FailedToCompile))
    }

    /// Given a closing brace: '[', find the matching opening brace
    /// from should be 1 less thant the offset of the closing brace.
    /// Example:
    /// for "+[+].", the closing brace is at 3.  So from must be 2.
    /// And the result will be Ok(1)

    fn find_prev_matching(&self, from: usize) -> Result<usize> {
        let mut enc = 0;
        let tokens = &self.tokens.0;
        // Remember, ranges are inclusive..exclusive.  So +1 to from
        for i in (0..from + 1).rev() {
            match &tokens[i] {
                Token::BlockOpen => {
                    if enc == 0 {
                        return Ok(i);
                    }
                    enc -= 1;
                }
                Token::BlockClose => enc += 1,
                _ => {}
            };
        }
        Err(Box::new(BrainFuckError::FailedToCompile))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::{BufWriter, Cursor};
    #[test]
    fn test_good_verify() {
        let input = "[[]]";
        let tokens = Tokens::try_from(input).expect("failed to tokenize");
        let result = tokens.verify_blocks();
        assert!(result.is_ok());
        assert_eq!(2, result.unwrap());
    }

    #[test]
    fn test_bad_verify_left() {
        let input = "[[]";
        let tokens = Tokens::try_from(input).expect("failed to tokenize");
        let result = tokens.verify_blocks();
        assert!(result.is_err());
    }

    #[test]
    fn test_bad_verify_right() {
        let input = "[[]]]";
        let tokens = Tokens::try_from(input).expect("failed to tokenize");
        let result = tokens.verify_blocks();
        assert!(result.is_err());
    }
    /*
       #[test]
       fn test_find_next_matching() {
           let input = "++[[++.[--]]]++.";
           let tokens = tokenize(input).expect("failed to tokenize");
           let result = Interpreter::find_next_matching(&tokens, 3);
           assert_eq!(result.unwrap(), 12);
           let result = Interpreter::find_next_matching(&tokens, 4);
           assert_eq!(result.unwrap(), 11);
           let result = Interpreter::find_next_matching(&tokens, 8);
           assert_eq!(result.unwrap(), 10);
       }

       #[test]
       fn test_find_prev_matching() {
           // Positions are [2 [3 [7  10] 11] 12]
           let input = "++[[++.[--]]]++.";
           let tokens = tokenize(input).expect("failed to tokenize");
           let result = Interpreter::find_prev_matching(&tokens, 11);
           assert_eq!(result.unwrap(), 2);

           let result = Interpreter::find_prev_matching(&tokens, 10);
           assert_eq!(result.unwrap(), 3);

           let result = Interpreter::find_prev_matching(&tokens, 9);
           assert_eq!(result.unwrap(), 7);
       }
    */
    #[test]
    fn test_it() {
        // The result in Cell 1 is 72.
        let input = "++++++++[>+++++++++<-]>.";

        // Throw away, since there's no input.
        let mut file = Cursor::new(Vec::new());

        let mut out_buf = BufWriter::new(Vec::new());

        let mut interpreter = Interpreter::builder()
            .tokens(input)
            .expect("fail")
            .inbuf(&mut file)
            .outbuf(&mut out_buf)
            .build()
            .expect("failed to build");
        //let mut interpreter = builder.build().expect("failed to build interpreter");
        let result = interpreter.run();
        assert!(result.is_ok());
        assert_eq!(vec![72], out_buf.buffer());
    }
}
