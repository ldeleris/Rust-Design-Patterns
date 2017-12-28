pub mod std {
  use std;
  use std::collections::VecDeque;

  pub trait Expression 
    where Self: std::fmt::Debug
  {
    fn interpret(&self) -> i32;
    fn print(&self) -> String {
        format!("{:?}", self)
    }
  }


  #[derive(Debug)]
  pub struct Number {
    n: i32,
  }
  impl Number {
    pub fn new(n:i32) -> Number { Number { n } }
  }
  impl Expression for Number {
    fn interpret(&self) -> i32 {
      self.n
    }
  }

  #[derive(Debug)]
  pub struct Add {
    right: Box<Expression>,
    left: Box<Expression>,
  }
  impl Add {
    pub fn new(right: Box<Expression>, left: Box<Expression>) -> Add {
      Add { right, left }
    }
  }
  impl Expression for Add {
    fn interpret(&self) -> i32 {
      self.left.interpret() + self.right.interpret()
    }
  }

  #[derive(Debug)]
  pub struct Subtract {
    right: Box<Expression>,
    left: Box<Expression>,
  }
  impl Subtract {
    pub fn new(right: Box<Expression>, left: Box<Expression>) -> Subtract {
      Subtract { right, left }
    }
  }
  impl Expression for Subtract {
    fn interpret(&self) -> i32 {
      self.left.interpret() - self.right.interpret()
    }
  }

  #[derive(Debug)]
  pub struct Multiply {
    right: Box<Expression>,
    left: Box<Expression>,
  }
  impl Multiply {
    pub fn new(right: Box<Expression>, left: Box<Expression>) -> Multiply {
      Multiply { right, left }
    }
  }
  impl Expression for Multiply {
    fn interpret(&self) -> i32 {
      self.left.interpret() * self.right.interpret()
    }
  }

  pub struct Factory;
  impl Factory {
    pub fn new(operator: &str, right:Option<Box<Expression>>, left: Option<Box<Expression>>) -> Option<Box<Expression>> {
      match (operator, left, right) {
        ("+", Some(l), Some(r)) => Some(Box::new(Add::new(r, l))),
        ("-", Some(l), Some(r)) => Some(Box::new(Subtract::new(r, l))),
        ("*", Some(l), Some(r)) => Some(Box::new(Multiply::new(r, l))),
        (n, _, _) => {
                if let Ok(n) = n.parse::<i32>() {
                  Some(Box::new(Number::new(n)))
                } else {
                  None
                }
        },
      }
    }
  }

  pub struct RPNParser;
  impl RPNParser {
    pub fn parse(expression: &str) -> Option<Box<Expression>> {
      let token: Vec<&str> = expression.split(' ').collect(); 
      let result: VecDeque<Box<Expression>> = VecDeque::new();
      let mut x = token.iter()
        .fold(result, |mut acc, &token | -> VecDeque<Box<Expression>> {
          if let Some(item) = Factory::new(token, RPNParser::test_token(token, &mut acc), RPNParser::test_token(token, &mut acc)) {
            acc.push_front(item);
          };
          //println!("{:?}", acc);
          acc
        }); 
      x.pop_front()
    }
    fn test_token(token: &str, acc: &mut VecDeque<Box<Expression>>) -> Option<Box<Expression>> {
      match token {
        "+" | "-" | "*" => acc.pop_front(),
        _ => None,
      }
    }
  }

  pub struct RPNInterpreter;
  impl RPNInterpreter {
    pub fn interpret(expression: Option<Box<Expression>>) -> Option<i32> {
      match expression {
        Some(e) => Some(e.interpret()),
        None => None,
      }
    }
  }
}
