use std::{fmt::Display, io::Write};

use crate::harness::core::error::HarnessError;

pub trait CodegenOutput {
    fn write_line<T>(&mut self, content: T) -> Result<(), HarnessError>
        where T: Display;
    fn indent_up(&mut self);
    fn indent_down(&mut self);
    fn skip_newline(&mut self);
    fn flush(&mut self) -> Result<(), HarnessError>;
}

pub struct WriteCodegenOutput<'a, Output: Write> {
    output: &'a mut Output,
    indent: u64,
    first_line: bool,
    skip_newline: bool
}

impl<'a, Output: Write> WriteCodegenOutput<'a, Output> {
    pub fn new(output: &'a mut Output) -> WriteCodegenOutput<'a, Output> {
        WriteCodegenOutput {
            output,
            indent: 0,
            first_line: true,
            skip_newline: false
        }
    }
}

impl<'a, Output: Write> CodegenOutput for WriteCodegenOutput<'a, Output> {
    fn indent_down(&mut self) {
        self.indent -= 1;
    }

    fn indent_up(&mut self) {
        self.indent += 1;
    }

    fn skip_newline(&mut self) {
        self.skip_newline = true;
    }

    fn write_line<T>(&mut self, content: T) -> Result<(), HarnessError>
            where T: Display {
        if !self.first_line && !self.skip_newline {
            self.output.write_fmt(format_args!("\n")).map_err(| err | HarnessError::new(err.to_string()))?;
        }

        if self.first_line || !self.skip_newline {
            for _ in 0..self.indent {
                self.output.write_fmt(format_args!("  ")).map_err(| err | HarnessError::new(err.to_string()))?;
            }
        }
        self.first_line = false;
        self.skip_newline = false;
        self.output.write_fmt(format_args!("{}", content)).map_err(| err | HarnessError::new(err.to_string()))?;
        Ok(())
    }
    
    fn flush(&mut self) -> Result<(), HarnessError> {
        self.output.flush().map_err(| err | HarnessError::new(err.to_string()))
    }
}