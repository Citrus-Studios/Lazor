use std::{fmt::Display, str::FromStr};

use crossterm::event::KeyCode;
use phf::phf_map;

#[derive(Debug)]
pub enum Layout {
    QwertyENUS,
    DvorakProgrammers,
}

impl FromStr for Layout {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "QwertyENUS" => Ok(Layout::QwertyENUS),
            "DvorakProgrammers" => Ok(Layout::DvorakProgrammers),
            _ => Err("Unknown Keyboard Layout".to_string()),
        }
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layout::QwertyENUS => f.write_str("QwertyENUS"),
            Layout::DvorakProgrammers => f.write_str("DvorakProgrammers"),
        }
    }
}

pub const DVORAK_PROGRAMMERS: phf::Map<char, char> = phf_map! {
    '$' => '`',
    '~' => '~',
    '&' => '1',
    '%' => '!',
    '[' => '2',
    '7' => '@',
    '{' => '3',
    '5' => '#',
    '}' => '4',
    '3' => '$',
    '(' => '5',
    '1' => '%',
    '=' => '6',
    '9' => '^',
    '*' => '7',
    '0' => '&',
    ')' => '8',
    '2' => '*',
    '+' => '9',
    '4' => '(',
    ']' => '0',
    '6' => ')',
    '!' => '-',
    '8' => '_',
    '#' => '=',
    '`' => '+',
    ';' => 'q',
    ',' => 'w',
    '.' => 'e',
    'p' => 'r',
    'y' => 't',
    'f' => 'y',
    'g' => 'u',
    'c' => 'i',
    'r' => 'o',
    'l' => 'p',
    '/' => '[',
    '@' => ']',
    '\\' => '\\',
    'a' => 'a',
    'o' => 's',
    'e' => 'd',
    'u' => 'f',
    'i' => 'g',
    'd' => 'h',
    'h' => 'j',
    't' => 'k',
    'n' => 'l',
    's' => ';',
    '-' => '\'',
    '\''=> 'z',
    'q' => 'x',
    'j' => 'c',
    'k' => 'v',
    'x' => 'b',
    'b' => 'n',
    'm' => 'm',
    'w' => ',',
    'v' => '.',
    'z' => '/',
    ':' => 'Q',
    '<' => 'W',
    '>' => 'E',
    'P' => 'R',
    'Y' => 'T',
    'F' => 'Y',
    'G' => 'U',
    'C' => 'I',
    'R' => 'O',
    'L' => 'P',
    '?' => '{',
    '^' => '}',
    '|' => '|',
    'A' => 'A',
    'O' => 'S',
    'E' => 'D',
    'U' => 'F',
    'I' => 'G',
    'D' => 'H',
    'H' => 'J',
    'T' => 'K',
    'N' => 'L',
    'S' => ':',
    '_' => '"',
    '"' => 'Z',
    'Q' => 'X',
    'J' => 'C',
    'K' => 'V',
    'X' => 'B',
    'B' => 'N',
    'M' => 'M',
    'W' => '<',
    'V' => '>',
    'Z' => '?',
};

impl Layout {
    pub fn get(&self, ch: char) -> KeyCode {
        KeyCode::Char(match self {
            Layout::QwertyENUS => ch,
            Layout::DvorakProgrammers => *DVORAK_PROGRAMMERS.get(&ch).unwrap(),
        })
    }
}
