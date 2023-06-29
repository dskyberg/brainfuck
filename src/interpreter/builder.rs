//! Builder pattern for the [Interpreter]
use std::io::prelude::*;
use std::io::Write;

use super::{token::Tokens, Interpreter};
use crate::error::{BrainFuckError, Result};

/// This is never actually used, since the CLI in [src/main] sets a default.
const MAX_DATA_PTR: usize = 30_000;

/// Builder pattern for the [Interpreter]
pub struct InterpreterBuilder<'a> {
    _tokens: Option<Tokens>,
    _inbuf: Option<&'a mut dyn Read>,
    _outbuf: Option<&'a mut dyn Write>,
    _max_data: Option<usize>,
}

impl<'a> InterpreterBuilder<'a> {
    pub fn new() -> Self {
        Self {
            _tokens: None,
            _inbuf: None,
            _outbuf: None,
            _max_data: None,
        }
    }
    /// Sets the size of the data byte vector.  Defaults to 30,000
    pub fn max_data(mut self, max_data: usize) -> Self {
        self._max_data = Some(max_data);
        self
    }

    /// The string of tokens to be tokenized.
    /// This method both tokenizes the string and verifies correctness.
    /// The only verification done is to ensure brackets are properly balanced.
    pub fn tokens(mut self, value: &str) -> Result<Self> {
        let tokens = Tokens::try_from(value)?;
        self._tokens = Some(tokens);
        Ok(self)
    }

    /// The [Read] buffer to use for the `,` instruction.
    pub fn inbuf(mut self, inbuf: &'a mut dyn Read) -> Self {
        self._inbuf = Some(inbuf);
        self
    }

    /// The [Write] buffer to use for the `.` command.
    pub fn outbuf(mut self, outbuf: &'a mut dyn Write) -> Self {
        self._outbuf = Some(outbuf);
        self
    }

    /// Build the [Interpreter] with the build args.
    pub fn build(mut self) -> Result<Interpreter<'a>> {
        let tokens = self
            ._tokens
            .take()
            .ok_or(Box::new(BrainFuckError::InterpreterBuildError(
                "No tokens".to_string(),
            )))?;
        tokens.verify_blocks()?;

        let max_data = self._max_data.unwrap_or(MAX_DATA_PTR);
        let data: Vec<u8> = vec![0; max_data];

        let outbuf = self
            ._outbuf
            .take()
            .ok_or(Box::new(BrainFuckError::InterpreterBuildError(
                "No output buffer".to_string(),
            )))?;

        let inbuf = self
            ._inbuf
            .take()
            .ok_or(Box::new(BrainFuckError::InterpreterBuildError(
                "No input buffer".to_string(),
            )))?;

        Ok(Interpreter::new(inbuf, outbuf, tokens, data))
    }
}
