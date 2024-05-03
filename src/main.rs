#![allow(warnings, unused)]

fn main() {
    let input = "123 + 456 * 789";

    let mut parser = Parser::new(input);

    let expr = parser.parse(0);
    println!("expr: {:?}", expr);

    // let value = parser.eval();

    // if value.is_some() {
    //     println!("{}", value.unwrap());
    // } else { 
    //     println!("undefined") 
    // };
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
enum Token {
    Num(f64),
    Op(Op),
    Rp,
    Lp,
}

#[derive(Debug)]
struct Lexer {
    input: String,
    pos: usize,
    curr: Option<char>,
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_none() {
            return None;
        }

        self.skip_whitespace();

        let token = match self.curr.unwrap() {
            c if c.is_ascii_digit() || c == '.' => return self.read_number(),
            '+' => Token::Op(Op::Add),
            '-' => Token::Op(Op::Sub),
            '*' => Token::Op(Op::Mul),
            '/' => Token::Op(Op::Div),
            '(' => Token::Lp,
            ')' => Token::Rp,
            c => panic!("illegal token {}", c),
        };

        self.read_char();
        Some(token)
    }
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            pos: 0,
            curr: input.chars().nth(0),
        }
    }

    fn read_number(&mut self) -> Option<Token> {
        let pos = self.pos;

        while let Some(curr) = self.curr {
            if curr.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }

        let s = &self.input[pos..self.pos];
        dbg!(s);
        Some(Token::Num(s.parse().unwrap()))
    }

    fn read_char(&mut self) {
        self.pos += 1;
        self.curr = self.input.chars().nth(self.pos);
    }

    fn skip_whitespace(&mut self) {
        while let Some(curr) = self.curr {
            if curr.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Num(f64),
    Neg(Box<Expr>),
    Bin(Op, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    curr: usize,
    peek: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        let tokens = Lexer::new(input).collect();
        Self {
            tokens,
            curr: 0,
            peek: 1,
        }
    }

    fn log(&self) {
        println!("curr: {:?}, peek: {:?}", self.curr(), self.peek());
    }

    fn curr(&self) -> Option<Token> {
        self.tokens.get(self.curr).cloned()
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.peek).cloned()
    }

    fn next_token(&mut self) {
        self.curr += 1;
        self.peek += 1;
    }

    fn eval(&mut self) -> Option<f64> {
        let expr = self.parse(0)?;
        self.eval_expr(expr)
    }

    fn eval_expr(&self, expr: Expr) -> Option<f64> {
        let value = match expr {
            Expr::Num(n) => n,
            Expr::Neg(expr) => -self.eval_expr(*expr)?,
            Expr::Bin(op, left, right) => {
                match op {
                    Op::Add => self.eval_expr(*left)? + self.eval_expr(*right)?,
                    Op::Sub => self.eval_expr(*left)? - self.eval_expr(*right)?,
                    Op::Mul => self.eval_expr(*left)? * self.eval_expr(*right)?,
                    Op::Div => {
                        let right = self.eval_expr(*right)?;

                        if right == 0.0 {
                            return None;
                        }

                        self.eval_expr(*left)? / right
                    }
                }
            }
        };

        Some(value)
    }

    fn parse(&mut self, prec: usize) -> Option<Expr> {
        if let Some(mut left) = self.parse_prefix() {
        //     println!("left: {:?}", left);

            // println!("self.curr: {:?}", self.curr());
            // println!("self.curr_prec: {:?}", self.curr_prec());

            while self.curr < self.tokens.len() - 1 && prec < self.peek_prec() {
                self.next_token();

                if let Some(right) = self.parse_infix(left.clone()) {
                    left = right;
                } else {
                    return Some(left);
                }
            }

            Some(left)
        } else {
            return None;
        }
    }

    fn parse_prefix(&mut self) -> Option<Expr> {
        if let Some(token) = self.curr() {
            match token {
                Token::Num(n) => Some(Expr::Num(n)),
                Token::Lp => self.parse_grouped_expr(),
                Token::Op(Op::Sub) => {
                    self.next_token();

                    let value = self.parse(1)?;

                    Some(Expr::Neg(Box::new(value)))
                }
                _ => None,
            }
        } else {
            None
        }
    }

    fn parse_infix(&mut self, left: Expr) -> Option<Expr> {
        match self.curr() {
            Some(token) => match token {
                Token::Op(op) => {
                    self.next_token();

                    let right = self.parse(self.peek_prec())?;

                    println!("left: {:?}, right: {:?}", left, right);

                    Some(Expr::Bin(op, Box::new(left), Box::new(right)))
                }
                _ => None,
            },
            None => None,
        }
    }

    fn parse_grouped_expr(&mut self) -> Option<Expr> {
        todo!()
    }

    fn curr_prec(&self) -> usize {
        match self.curr() {
            Some(token) => Self::check_prec(&token),
            None => 0,
        }
    }

    fn peek_prec(&self) -> usize {
        println!("peek: {:?}", self.peek());

        match self.peek() {
            Some(token) => Self::check_prec(&token),
            None => 0,
        }
    }

    fn check_prec(token: &Token) -> usize {
        match token {
            Token::Op(op) => match op {
                Op::Add | Op::Sub => 1,
                Op::Mul | Op::Div => 2,
            },
            _ => 0,
        }
    }

    fn prefix_error(&self) {
        panic!("unable to parse prefix {:?}", self.curr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num() {
        // assert!(Parser::new("123").eval().unwrap() == 123.0);
        // assert!(Parser::new("123.456").eval().unwrap() == 123.456);
        // assert!(Parser::new("123 + 456").eval().unwrap() == 579.0);
        // assert!(Parser::new("123 - 456").eval().unwrap() == -333.0);
        // assert!(Parser::new("123 * 456").eval().unwrap() == 56088.0);
        // assert!(Parser::new("123 / 456").eval().unwrap() == 0.26973684210526316);
        assert_eq!(Parser::new("2 + 3 * 4").eval().unwrap(), 14.0);
        assert_eq!(Parser::new("2 * 4 + 3").eval().unwrap(), 11.0);
        // assert!(Parser::new("123 + 456 / 789").eval().unwrap() == 123.5764331210191);
        // assert!(Parser::new("123 / 456 + 789").eval().unwrap() == 789.2697368421053);
        // assert!(Parser::new("123 * 456 / 789").eval().unwrap() == 70.52631578947368);
    }
}