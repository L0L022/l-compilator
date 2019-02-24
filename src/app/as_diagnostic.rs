use crate::lexer::LexicalError;
use crate::parser::ParseError;
use codespan::{ByteIndex, FileMap, Span};
use codespan_reporting::{Diagnostic, Label, Severity};
use failure::Fail;
use std::fmt::Write;

pub trait AsDiagnostic {
    fn as_diagnostic(&self, file_map: &FileMap) -> Option<Diagnostic>;
}

impl AsDiagnostic for failure::Error {
    fn as_diagnostic(&self, file_map: &FileMap) -> Option<Diagnostic> {
        self.as_fail().as_diagnostic(file_map)
    }
}

impl AsDiagnostic for &dyn Fail {
    fn as_diagnostic(&self, file_map: &FileMap) -> Option<Diagnostic> {
        if let Some(error) = self.downcast_ref::<LexicalError>() {
            return error.as_diagnostic(file_map);
        }

        if let Some(error) = self.downcast_ref::<ParseError>() {
            return error.as_diagnostic(file_map);
        }

        None
    }
}

impl LexicalError {
    fn as_diagnostic(&self, _file_map: &FileMap) -> Option<Diagnostic> {
        let span = Span::new(
            ByteIndex(self.range.start as u32 + 1),
            ByteIndex(self.range.end as u32 + 1),
        );
        let diag = Diagnostic::new(Severity::Error, "Lexical error occured")
            .with_label(Label::new_primary(span).with_message(self.error.to_string()));
        Some(diag)
    }
}

impl ParseError {
    fn as_diagnostic(&self, file_map: &FileMap) -> Option<Diagnostic> {
        use lalrpop_util::ParseError::*;

        match &self.error {
            UnrecognizedToken { token, expected } => {
                let error = Diagnostic::new(Severity::Error, "An unexpected token was observed");

                let (error, span) = if let Some(token) = token {
                    let (start, token, end) = token;
                    let span = Span::new(ByteIndex(*start as u32 + 1), ByteIndex(*end as u32 + 1));

                    (
                        error.with_label(
                            Label::new_primary(span)
                                .with_message(format!("unexpected token: {}", token)),
                        ),
                        span,
                    )
                } else {
                    let end = file_map.span().end().to_usize() as u32;
                    let span = Span::new(ByteIndex(end - 1), ByteIndex(end));

                    (
                        error.with_label(Label::new_primary(span).with_message("unrecognized EOF")),
                        span,
                    )
                };

                let error = if !expected.is_empty() {
                    let mut message = String::new();
                    for (i, e) in expected.iter().enumerate() {
                        let sep = match i {
                            0 => "expected one of",
                            _ if i < expected.len() - 1 => ",",
                            // Last expected message to be written
                            _ => " or",
                        };
                        write!(&mut message, "{} {}", sep, e).unwrap();
                    }
                    error.with_label(Label::new_secondary(span).with_message(message))
                } else {
                    error
                };

                Some(error)
            }
            ExtraToken { token } => {
                let (start, token, end) = token;
                let span = Span::new(ByteIndex(*start as u32 + 1), ByteIndex(*end as u32 + 1));

                let error = Diagnostic::new(Severity::Error, "An unexpected token was observed")
                    .with_label(
                        Label::new_primary(span).with_message(format!("extra token: {}", token)),
                    );

                Some(error)
            }
            User { error } => error.as_diagnostic(file_map),
            _ => None,
        }
    }
}
