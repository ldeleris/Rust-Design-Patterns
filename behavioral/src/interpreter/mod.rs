pub mod std {
  //! `std` module.
  //!
  //! # Example
  //! 
  //! ```rust
  //! use behavioral::interpreter::std::*;
  //! let expr = "1 2 + 3 * 9 10 + -";
  //! println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));
  //! let expr = "1 2 3 4 5 * * - +";
  //! println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));
  //! let expr = "12 -";
  //! println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));
  //! ```
  
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
  struct Number {
    n: i32,
  }
  impl Number {
    fn new(n:i32) -> Number { Number { n } }
  }
  impl Expression for Number {
    fn interpret(&self) -> i32 {
      self.n
    }
  }

  #[derive(Debug)]
  struct Add {
    right: Box<Expression>,
    left: Box<Expression>,
  }
  impl Add {
    fn new(right: Box<Expression>, left: Box<Expression>) -> Add {
      Add { right, left }
    }
  }
  impl Expression for Add {
    fn interpret(&self) -> i32 {
      self.left.interpret() + self.right.interpret()
    }
  }

  #[derive(Debug)]
  struct Subtract {
    right: Box<Expression>,
    left: Box<Expression>,
  }
  impl Subtract {
    fn new(right: Box<Expression>, left: Box<Expression>) -> Subtract {
      Subtract { right, left }
    }
  }
  impl Expression for Subtract {
    fn interpret(&self) -> i32 {
      self.left.interpret() - self.right.interpret()
    }
  }

  #[derive(Debug)]
  struct Multiply {
    right: Box<Expression>,
    left: Box<Expression>,
  }
  impl Multiply {
    fn new(right: Box<Expression>, left: Box<Expression>) -> Multiply {
      Multiply { right, left }
    }
  }
  impl Expression for Multiply {
    fn interpret(&self) -> i32 {
      self.left.interpret() * self.right.interpret()
    }
  }

  struct Factory;
  impl Factory {
    fn new(operator: &str, right:Option<Box<Expression>>, left: Option<Box<Expression>>) -> Option<Box<Expression>> {
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

pub mod closure {
  //! `closure` module.
  //!
  //! # Example
  //! 
  //! ```rust
  //! use behavioral::interpreter::closure::*;
  //! let expr = "1 2 + 3 * 9 10 + -";
  //! println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));
  //! let expr = "1 2 3 4 5 * * - +";
  //! println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));
  //! let expr = "12 -";
  //! println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));
  //! ```
  
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
  struct Number {
    n: i32,
  }
  impl Number {
    fn new(n:i32) -> Number { Number { n } }
  }
  impl Expression for Number {
    fn interpret(&self) -> i32 {
      self.n
    }
  }

  #[derive(Debug)]
  struct Add {
    right: Box<Expression>,
    left: Box<Expression>,
  }
  impl Add {
    fn new(right: Box<Expression>, left: Box<Expression>) -> Add {
      Add { right, left }
    }
  }
  impl Expression for Add {
    fn interpret(&self) -> i32 {
      self.left.interpret() + self.right.interpret()
    }
  }

  #[derive(Debug)]
  struct Subtract {
    right: Box<Expression>,
    left: Box<Expression>,
  }
  impl Subtract {
    fn new(right: Box<Expression>, left: Box<Expression>) -> Subtract {
      Subtract { right, left }
    }
  }
  impl Expression for Subtract {
    fn interpret(&self) -> i32 {
      self.left.interpret() - self.right.interpret()
    }
  }

  #[derive(Debug)]
  struct Multiply {
    right: Box<Expression>,
    left: Box<Expression>,
  }
  impl Multiply {
    fn new(right: Box<Expression>, left: Box<Expression>) -> Multiply {
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
    pub fn new(operator: &str, mut right: Box<FnMut() -> Option<Box<Expression>>>, mut left: Box<FnMut() -> Option<Box<Expression>>>) -> Option<Box<Expression>> {
      match operator {
        "+" => {
          match (right(), left()) {
            (Some(r), Some(l)) => Some(Box::new(Add::new(r, l))),
            (_, _) => None,
          }     
        },
        "-" => {
          match (right(), left()) {
            (Some(r), Some(l)) => Some(Box::new(Subtract::new(r, l))),
            (_, _) => None,
          }     
        },
        "*" => {
          match (right(), left()) {
            (Some(r), Some(l)) => Some(Box::new(Multiply::new(r, l))),
            (_, _) => None,
          }     
        },
        n => {
                if let Ok(n) = n.parse::<i32>() {
                  Some(Box::new(Number::new(n)))
                } else {
                  None
                }
        },
      }
    }
  }

  use std::rc::Rc;
  use std::cell::RefCell;

  pub struct RPNParser;
  impl RPNParser {
    pub fn parse(expression: &str) -> Option<Box<Expression>> {
      let token: Vec<&str> = expression.split(' ').collect(); 
      let result: Rc<RefCell<VecDeque<Box<Expression>>>> = Rc::new(RefCell::new(VecDeque::new()));
      let out = token.iter()
        .fold(result, |acc, &token | {
          let a1 = Rc::clone(&acc);
          let a2 = Rc::clone(&acc);
          if let Some(item) = Factory::new(token, Box::new(move || (*a1).borrow_mut().pop_front()), Box::new( move || (*a2).borrow_mut().pop_front())) {
            acc.borrow_mut().push_front(item);
          };
          //println!("{:?}", acc);
          //Rc::clone(&acc)
          acc
        });
        let out = out.borrow_mut().pop_front();
        out
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