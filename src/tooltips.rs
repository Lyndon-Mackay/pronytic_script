use std::{collections::HashMap, fmt};

use lalrpop_util::lalrpop_mod;
use logos::Logos;

use crate::{LexicalError, common::DataParser};

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[\s\t\f]+", error = LexicalError)]
#[logos(skip r"//[^\n\r]*")]
pub enum ToolTipsToken {
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[token(":")]
    Colon,
}

impl fmt::Display for ToolTipsToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lalrpop_mod!(pub tooltips);

#[derive(Clone, Debug)]
pub struct ToolTipsData {
    pub map: HashMap<String, Vec<ToolTipsContent>>,
}

#[derive(Clone, Debug)]
pub enum ToolTipsContent {
    String(String),
    Term(String),
    Highlight(String),
}

/// Creates tooltip content from a string that interprets terms for definitions
/// and highlighting
// If this gets much more complicated will write a seperate parser
pub fn create_tooltip_content(value: String) -> Vec<ToolTipsContent> {
    let mut result = Vec::new();
    let mut current_string = String::new();
    let mut opened_backtick = false;
    let mut opened_tilde = false;
    for char in value.chars() {
        match char {
            '`' => {
                if !opened_backtick {
                    result.push(ToolTipsContent::String(current_string.clone()));
                    current_string.clear();
                } else {
                    result.push(ToolTipsContent::Term(current_string.clone()));
                    current_string.clear();
                }
                opened_backtick = !opened_backtick;
            }
            '~' => {
                if !opened_tilde {
                    result.push(ToolTipsContent::String(current_string.clone()));
                    current_string.clear();
                } else {
                    result.push(ToolTipsContent::Highlight(current_string.clone()));
                    current_string.clear();
                }
                opened_tilde = !opened_tilde;
            }
            c => {
                current_string.push(c);
            }
        }
    }
    if opened_backtick {
        panic!("Trailing backtick");
    }
    if current_string.chars().count() > 0 {
        result.push(ToolTipsContent::String(current_string));
    }
    result
}

impl<'s> DataParser<'s> for ToolTipsData {
    type Token = ToolTipsToken;

    fn parse_tokens(
        tokens: Vec<(usize, Self::Token, usize)>,
    ) -> Result<Vec<Self>, lalrpop_util::ParseError<usize, Self::Token, String>> {
        tooltips::ToolTipsDataParser::new().parse(tokens)
    }
}
