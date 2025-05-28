use std::fmt::Display;

use crate::harness::core::error::HarnessError;

#[derive(Debug)]
pub enum TemplateFragment {
    Literal(String),
    InterpretedSequence(Vec<TemplateFragment>),
    VerbatimSequence(Vec<TemplateFragment>)
}

struct TemplateScanner<'a, Input: Iterator<Item = Result<char, HarnessError>>> {
    input: &'a mut Input,
    location: (u64, u64),
    lookahead: [Option<char>; 2]
}

pub struct TemplateParser<'a, Input: Iterator<Item = Result<char, HarnessError>>> {
    scanner: TemplateScanner<'a, Input>
}

impl TemplateFragment {
    pub fn canonicalize(self) -> TemplateFragment {
        match self {
            Self::Literal(content) => Self::Literal(content),

            Self::InterpretedSequence(fragments) => {
                let mut canonicalized_sequence = Vec::new();
                for fragment in fragments {
                    let fragment = fragment.canonicalize();
                    if let Self::InterpretedSequence(subfragments) = fragment {
                        canonicalized_sequence.extend(subfragments);
                    } else {
                        canonicalized_sequence.push(fragment);
                    }
                }
                Self::InterpretedSequence(canonicalized_sequence)
            }

            Self::VerbatimSequence(fragments) => {
                let mut canonicalized_sequence = Vec::new();
                for fragment in fragments {
                    let fragment = fragment.canonicalize();
                    if let Self::VerbatimSequence(subfragments) = fragment {
                        canonicalized_sequence.extend(subfragments);
                    } else {
                        canonicalized_sequence.push(fragment);
                    }
                }
                Self::VerbatimSequence(canonicalized_sequence)
            }
        }
    }
}

impl Display for TemplateFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Literal(content) => write!(f, "{}", content)?,

            Self::VerbatimSequence(elts) => for elt in elts {
                if let Self::InterpretedSequence(_) = elt {
                    write!(f, "@{{")?;
                    elt.fmt(f)?;
                    write!(f, "}}@")?;
                } else {
                    elt.fmt(f)?;
                }
            },

            Self::InterpretedSequence(elts) => for elt in elts {
                if let Self::VerbatimSequence(_) = elt {
                    write!(f, "@\"")?;
                    elt.fmt(f)?;
                    write!(f, "\"@")?;
                } else {
                    elt.fmt(f)?;
                }
            },
        }
        Ok(())
    }
}

impl<'a, Input: Iterator<Item = Result<char, HarnessError>>> TemplateScanner<'a, Input> {
    fn new(input: &'a mut Input) -> Result<TemplateScanner<'a, Input>, HarnessError> {
        let mut scanner = TemplateScanner {
            input,
            location: (1, 1),
            lookahead: [None, None]
        };
        scanner.next()?;
        scanner.next()?;
        Ok(scanner)
    }

    fn line_number(&self) -> u64 {
        self.location.0
    }

    fn column_number(&self) -> u64 {
        self.location.1
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

    fn expect(&mut self, expected: &str) -> Result<(), HarnessError> {
        for chr in expected.chars() {
            match self.lookahead(0) {
                Some(c) if c == chr => {
                    self.next()?;
                }

                _ => return Err(HarnessError::new(format!("Expected '{}' at {}:{}", expected, self.line_number(), self.column_number())))
            }
        }
        Ok(())
    }
}

impl<'a, Input: Iterator<Item = Result<char, HarnessError>>> TemplateParser<'a, Input> {
    fn new(input: &'a mut Input) -> Result<TemplateParser<'a, Input>, HarnessError> {
        let scanner = TemplateScanner::new(input)?;
        Ok(TemplateParser {
            scanner
        })
    }

    pub fn parse(input: &'a mut Input) -> Result<TemplateFragment, HarnessError> {
        TemplateParser::new(input)?.next_fragment_sequence(true).map(TemplateFragment::canonicalize)
    }

    fn next_fragment_sequence(&mut self, verbatim_mode: bool) -> Result<TemplateFragment, HarnessError> {
        let mut fragments = Vec::new();

        loop {
            match self.next_fragment(verbatim_mode)? {
                Some(fragment) => fragments.push(fragment),
                None => break
            }
        }

        if verbatim_mode {
            Ok(TemplateFragment::VerbatimSequence(fragments))
        } else {
            Ok(TemplateFragment::InterpretedSequence(fragments))
        }
    }

    fn next_fragment(&mut self, verbatim_mode: bool) -> Result<Option<TemplateFragment>, HarnessError> {
        let mut content = String::new();

        loop {
            match (self.scanner.lookahead(0), self.scanner.lookahead(1)) {
                (Some('@'), Some('{')) => {
                    if !verbatim_mode {
                        return Err(HarnessError::new(format!("Unexpected '@{{' in interpreted mode at {}:{}", self.scanner.line_number(), self.scanner.column_number())));
                    }
                    if !content.is_empty() {
                        return Ok(Some(TemplateFragment::Literal(content)));
                    }
                    self.scanner.next()?;
                    self.scanner.next()?;

                    let subfragment = self.next_fragment_sequence(false)?;
                    self.scanner.expect("}@")?;
                    return Ok(Some(subfragment));
                }
                (Some('@'), Some('"')) => {
                    if verbatim_mode {
                        return Err(HarnessError::new(format!("Unexpected '@\"' in verbatim mode at {}:{}", self.scanner.line_number(), self.scanner.column_number())));
                    }
                    if !content.is_empty() {
                        return Ok(Some(TemplateFragment::Literal(content)))
                    }
                    self.scanner.next()?;
                    self.scanner.next()?;

                    let subfragment = self.next_fragment_sequence(true)?;
                    self.scanner.expect("\"@")?;
                    return Ok(Some(subfragment));
                },
                (Some('@'), _) => {
                    if !verbatim_mode {
                        return Err(HarnessError::new(format!("Unexpected '@' in interpreted mode at {}:{}", self.scanner.line_number(), self.scanner.column_number())));
                    }
                    if !content.is_empty() {
                        return Ok(Some(TemplateFragment::Literal(content)));
                    }

                    self.scanner.next()?;
                    let mut subfragments = Vec::new();
                    let mut subcontent  = String::new();

                    loop {
                        match self.scanner.lookahead(0) {
                            Some('\n') => {
                                self.scanner.next()?;
                                break;
                            }

                            Some('@') => {
                                if !subcontent.is_empty() {
                                    subfragments.push(TemplateFragment::Literal(subcontent));
                                    subcontent = String::new();
                                }
                                let subfragment = match self.scanner.lookahead(1) {
                                    Some('"') => self.next_fragment(!verbatim_mode)?,
                                    _ => return Err(HarnessError::new(format!("Expected '@\"' at {}:{}", self.scanner.line_number(), self.scanner.column_number())))
                                };

                                if let Some(subfragment) = subfragment {
                                    subfragments.push(subfragment);
                                } else {
                                    break;
                                }
                            }

                            Some(chr) => {
                                subcontent.push(chr);
                                self.scanner.next()?;
                            }

                            None => break
                        }
                    }

                    subcontent.push('\n');
                    subfragments.push(TemplateFragment::Literal(subcontent));

                    return Ok(Some(TemplateFragment::InterpretedSequence(subfragments)));
                },
                (Some('"'), Some('@')) if verbatim_mode => break,
                (Some('}'), Some('@')) if !verbatim_mode => break,
                (Some(chr), _) => {
                    content.push(chr);
                    self.scanner.next()?;
                }
                (None, _) => break
            }
        }

        if !content.is_empty() {
            Ok(Some(TemplateFragment::Literal(content)))
        } else {
            Ok(None)
        }
    }
}
