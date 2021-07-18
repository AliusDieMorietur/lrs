mod tokens {
  #[derive(Debug)]
enum TokenType {
  // Single-character tokens.
  LeftParen,
  RightParen,
  KeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,

  // One or two character tokens.
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // Literals.
  Identifier,
  String,
  Number,

  // Keywords.
  And,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,

  Eof,
}

struct Token {
  ttype:   TokenType,
  lexeme:  String,
  literal: String,
  line:    usize,
}

impl Token {
  fn new(
    ttype: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
  ) -> Token {
    Token {
      ttype,
      lexeme,
      literal,
      line,
    }
  }

  fn to_string(self) -> String { format!("{:?} {} {}", self.ttype, self.lexeme, self.literal) }
}
}