// Single token 
pub struct Token {
    tok: String,
}

impl Token {
    pub fn new(tok: &str) -> Self {
        Token { tok.to_string() }
    }
}

pub struct Tokenizer {
    token_size: u8,
}

// Token factory
impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer { token_size: 4 }
    }

    pub fn set_size(mut self, size: u8) -> Self {
        self.token_size = size;
        self
    }

    // Takes in a string slice and first splits it by whitespace, then iterates over each split by
    // token_size and generates tokens accordingly, adding them to a Vec<Token> before returning. 
    pub fn split(item: &str, token_vec: Vec<Token>) -> Vec<Token> {
        let mut whitespace = item.split_ascii_whitespace();

        for substr in whitespace {
            let mut i = 0;
            while i < self.token_size {
                let chunk = &substr[i..=i + self.token_size - 1];
                token_vec.push(Token::new(chunk));
                i += token_size;
            }
        }

        token_vec
    }
}

#[test]
fn tokenizer_test() {
    println!("TOKENIZER_TEST_UNIMPL");
    assert_eq!(1, 0);
}
