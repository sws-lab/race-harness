use crate::harness::core::error::HarnessError;

#[derive(Debug)]
pub enum DSLFragment {
    Verbatim(String),
    Interpreted(String)
}

struct DSLScanner<'a, Input: Iterator<Item = Result<char, HarnessError>>> {
    input: &'a mut Input,
    location: (u64, u64),
    lookahead: [Option<char>; 2]
}

pub struct DSLParser<'a, Input: Iterator<Item = Result<char, HarnessError>>> {
    scanner: DSLScanner<'a, Input>
}

enum ControlFlowBlock {
    Function,
    Do,
    If,
    While(bool),
    For(bool),
    Repeat,
    Parenthese,
    Brace,
    Bracket
}

impl<'a, Input: Iterator<Item = Result<char, HarnessError>>> DSLScanner<'a, Input> {
    fn new(input: &'a mut Input) -> Result<DSLScanner<'a, Input>, HarnessError> {
        let mut scanner = DSLScanner {
            input,
            location: (1, 1),
            lookahead: [None, None]
        };
        scanner.next()?;
        scanner.next()?;
        Ok(scanner)
    }

    fn lookahead(&self, index: usize) -> Option<char> {
        *self.lookahead.get(index).expect("Invalid lookahead index")
    }

    fn next(&mut self) -> Result<Option<char>, HarnessError> {
        if let Some('\n') = self.lookahead[0] {
            self.location.0 += 1;
            self.location.1 = 1;
        } else {
            self.location.1 += 1;
        }
        self.lookahead[0] = self.lookahead[1];
        self.lookahead[1] = match self.input.next() {
            Some(Ok(chr)) => Some(chr),
            Some(Err(e)) => return Err(e),
            None => None
        };
        Ok(self.lookahead(0))
    }

    fn next_unit(&mut self) -> Result<Option<String>, HarnessError> {
        let mut content = String::new();
        loop {
            match self.lookahead(0) {
                Some(chr) if chr.is_alphanumeric() => {
                    content.push(chr);
                    self.next()?;
                }
                
                Some(chr) if content.is_empty() => {
                    content.push(chr);
                    self.next()?;
                    break;
                }

                Some(_) => break,
                None => break
            }
        }

        if content.is_empty() {
            Ok(None)
        } else {
            Ok(Some(content))
        }
    }
}

impl<'a, Input: Iterator<Item = Result<char, HarnessError>>> DSLParser<'a, Input> {
    fn new(input: &'a mut Input) -> Result<DSLParser<'a, Input>, HarnessError> {
        let scanner = DSLScanner::new(input)?;
        Ok(DSLParser {
            scanner
        })
    }

    pub fn parse(input: &'a mut Input) -> Result<Vec<DSLFragment>, HarnessError> {
        let mut parser = DSLParser::new(input)?;
        let mut fragments = Vec::new();
        while let Some(fragment) = parser.next_fragment()? {
            fragments.push(fragment);
        }
        Ok(fragments)
    }

    fn next_fragment(&mut self) -> Result<Option<DSLFragment>, HarnessError> {
        match self.scanner.lookahead(0) {
            Some('@') => {
                self.scanner.next()?;
                self.next_interpreted_fragment().map(Some)
            },
            Some(_) => self.next_verbatim_fragment().map(Some),
            None => Ok(None)
        }
    }

    fn next_verbatim_fragment(&mut self) -> Result<DSLFragment, HarnessError> {
        let mut content = String::new();
        loop {
            match self.scanner.lookahead(0) {
                Some('@') => break,
                
                Some(chr) => {
                    content.push(chr);
                    self.scanner.next()?;
                }

                None => break
            }
        }

        Ok(DSLFragment::Verbatim(content))
    }

    fn next_interpreted_fragment(&mut self) -> Result<DSLFragment, HarnessError> {
        let mut content = String::new();
        let mut raw_string_mode = None;
        let mut control_flow = Vec::new();

        loop {
            let (line, column) = self.scanner.location;
            if let Some(depth) = raw_string_mode {
                match (self.scanner.lookahead(0), self.scanner.lookahead(1)) {
                    (Some(']'), Some(']')) | (Some(']'), Some('=')) => {
                        let mut unit = String::new();
                        unit.push(self.scanner.lookahead(0).unwrap());
                        self.scanner.next()?;
                        let mut detected_depth = 0 as u64;
                        while let Some('=') = self.scanner.lookahead(0) {
                            detected_depth += 1;
                            unit.push(self.scanner.lookahead(0).unwrap());
                            self.scanner.next()?;
                        }

                        if detected_depth  == depth {
                            if let Some(']') = self.scanner.lookahead(0) {
                                raw_string_mode = None;
                                unit.push(self.scanner.lookahead(0).unwrap());
                                self.scanner.next()?;
                            }
                        }

                        content.push_str(&unit);
                        continue;
                    }

                    (Some(chr), _) => {
                        content.push(chr);
                        self.scanner.next()?;
                        continue;
                    }

                    (None, _) => break
                }
            }

            match (self.scanner.lookahead(0), self.scanner.lookahead(1)) {
                (Some('['), Some('[')) | (Some('['), Some('=')) => {
                    let mut unit = String::new();
                    unit.push(self.scanner.lookahead(0).unwrap());
                    self.scanner.next()?;
                    let mut depth = 0 as u64;
                    while let Some('=') = self.scanner.lookahead(0) {
                        depth += 1;
                        unit.push(self.scanner.lookahead(0).unwrap());
                        self.scanner.next()?;
                    }
                    if let Some('[') = self.scanner.lookahead(0) {
                        raw_string_mode = Some(depth);
                        unit.push(self.scanner.lookahead(0).unwrap());
                        self.scanner.next()?;
                    }

                    content.push_str(&unit);
                    continue;
                }

                _ => ()
            }

            let unit = self.scanner.next_unit()?;
            match unit.as_deref() {
                Some("\n") if control_flow.is_empty() => break,

                Some("function") =>
                    control_flow.push(ControlFlowBlock::Function),

                Some("do") => {
                    match control_flow.last() {
                        Some(ControlFlowBlock::For(false)) => {
                            control_flow.pop();
                            control_flow.push(ControlFlowBlock::For(true));
                        }
                        
                        Some(ControlFlowBlock::While(false)) => {
                            control_flow.pop();
                            control_flow.push(ControlFlowBlock::While(true));
                        }

                        _ => {
                            control_flow.push(ControlFlowBlock::Do);
                        }
                    }
                }

                Some("while") =>
                    control_flow.push(ControlFlowBlock::While(false)),

                Some("for") =>
                    control_flow.push(ControlFlowBlock::For(false)),

                Some("repeat") =>
                    control_flow.push(ControlFlowBlock::Repeat),

                Some("if") =>
                    control_flow.push(ControlFlowBlock::If),

                Some("(") =>
                    control_flow.push(ControlFlowBlock::Parenthese),

                Some("{") =>
                    control_flow.push(ControlFlowBlock::Brace),

                Some("[") =>
                    control_flow.push(ControlFlowBlock::Bracket),

                Some(")") => if let Some(ControlFlowBlock::Parenthese) = control_flow.last() {
                    control_flow.pop();
                } else {
                    return Err(HarnessError::new(format!("Failed to parse interpreted code in the harness template at {}:{}", line, column)));
                }

                Some("]") => if let Some(ControlFlowBlock::Bracket) = control_flow.last() {
                    control_flow.pop();
                } else {
                    return Err(HarnessError::new(format!("Failed to parse interpreted code in the harness template at {}:{}", line, column)));
                }

                Some("}") => if let Some(ControlFlowBlock::Brace) = control_flow.last() {
                    control_flow.pop();
                } else {
                    return Err(HarnessError::new(format!("Failed to parse interpreted code in the harness template at {}:{}", line, column)));
                }

                Some("end") => match control_flow.last() {
                    Some(ControlFlowBlock::Function) | Some(ControlFlowBlock::Do) | Some(ControlFlowBlock::For(_)) |
                    Some(ControlFlowBlock::While(_)) | Some(ControlFlowBlock::If) | Some(ControlFlowBlock::Repeat) => {
                        control_flow.pop();
                    }

                    _ => return Err(HarnessError::new(format!("Failed to parse interpreted code in the harness template at {}:{}", line, column)))
                }

                Some(_) => (),

                None => break
            }

            if let Some(unit) = unit {
                content.push_str(&unit);
            }
        }

        Ok(DSLFragment::Interpreted(content))
    }
}
