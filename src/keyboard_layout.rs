pub enum Layout {
    Qwerty_EN_US,
    Dvorak_Programmers,
}

pub const DVORAK_PROGRAMMERS: phf::Map<char, char> = phf_map! {
	'`' => '$',
	'~' => '~',
	'1' => '&',
	'!' => '%',
	'2' => '[',
	'@' => '7',
	'3' => '{',
	'#' => '5',
	'4' => '}',
	'$' => '3',
	'5' => '(',
	'%' => '1',
	'6' => '=',
	'^' => '9',
	'7' => '*',
	'&' => '0',
	'8' => ')',
	'*' => '2',
	'9' => '+',
	'(' => '4',
	'0' => ']',
	')' => '6',
	'-' => '!',
	'_' => '8',
	'=' => '#',
	'+' => '`',
	'q' => ';',
	'w' => ',',
	'e' => '.',
	'r' => 'p',
	't' => 'y',
	'y' => 'f',
	'u' => 'g',
	'i' => 'c',
	'o' => 'r',
	'p' => 'l',
	'[' => '/',
	']' => '@',
	'\\ => '\','
	'a' => 'a',
	's' => 'o',
	'd' => 'e',
	'f' => 'u',
	'g' => 'i',
	'h' => 'd',
	'j' => 'h',
	'k' => 't',
	'l' => 'n',
	';' => 's',
	'\\' => '-',
	'z' => '\'',
	'x' => 'q',
	'c' => 'j',
	'v' => 'k',
	'b' => 'x',
	'n' => 'b',
	'm' => 'm',
	',' => 'w',
	'.' => 'v',
	'/' => 'z',
	'Q' => ':',
	'W' => '<',
	'E' => '>',
	'R' => 'P',
	'T' => 'Y',
	'Y' => 'F',
	'U' => 'G',
	'I' => 'C',
	'O' => 'R',
	'P' => 'L',
	'{' => '?',
	'}' => '^',
	'|' => '|',
	'A' => 'A',
	'S' => 'O',
	'D' => 'E',
	'F' => 'U',
	'G' => 'I',
	'H' => 'D',
	'J' => 'H',
	'K' => 'T',
	'L' => 'N',
	':' => 'S',
	'"' => '_',
	'Z' => '\'',
	'X' => 'Q',
	'C' => 'J',
	'V' => 'K',
	'B' => 'X',
	'N' => 'B',
	'M' => 'M',
	'<' => 'W',
	'>' => 'V',
	'?' => 'Z',
};

impl Layout {
	fn get(&self, ch: char) -> char {
		match self {
			Layout::Qwerty_EN_US => ch,
			Layout::Dvorak_Programmers => DVORAK_PROGRAMMERS.get(&ch).unwrap(),
		}
	}
}