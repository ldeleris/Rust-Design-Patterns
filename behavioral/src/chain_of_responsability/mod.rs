
#[derive(Debug, PartialEq, Hash, Clone)]
pub struct Money {
  amount: i32,
}
impl Money {
  pub fn new(amount: i32) -> Money {
    Money { amount }
  }
}

pub mod std {
  use std::rc::Rc;
  use super::Money;

  pub trait Dispenser {
    fn amount(&self) -> i32;
    fn next(&self) -> Option<Rc<Box<Dispenser>>>;
    fn dispense(&self, money: Money) {
      if money.amount >= self.amount() {
        let notes = money.amount / self.amount();
        let left = money.amount % self.amount();
        println!("Dispensing {} note/s of {}.", notes, self.amount());
        if left > 0 {
          if let Some(d) = self.next() {
            d.dispense(Money::new(left));
          };
        }
      } else {
        if let Some(d) = self.next() {
          d.dispense(money);
        }
      }
    }
  }

  pub struct Dispenser50 {
    next: Option<Rc<Box<Dispenser>>>,
  }
  impl Dispenser50 {
    pub fn new(next: Option<Rc<Box<Dispenser>>>) -> Dispenser50 {
      Dispenser50 { next }
    }
  }
  impl Dispenser for Dispenser50 {
    fn amount(&self) -> i32 { 50 }
    fn next(&self) -> Option<Rc<Box<Dispenser>>> { 
      if let Some(ref d) = self.next {
        Some(Rc::clone(&d))
      } else {
        None
      }
    }
  }

  pub struct Dispenser20 {
    next: Option<Rc<Box<Dispenser>>>,
  }
  impl Dispenser20 {
    pub fn new(next: Option<Rc<Box<Dispenser>>>) -> Dispenser20 {
      Dispenser20 { next }
    }
  }
  impl Dispenser for Dispenser20 {
    fn amount(&self) -> i32 { 20 }
    fn next(&self) -> Option<Rc<Box<Dispenser>>> { 
      if let Some(ref d) = self.next {
        Some(Rc::clone(&d))
      } else {
        None
      }
    }
  }

  pub struct Dispenser10 {
    next: Option<Rc<Box<Dispenser>>>,
  }
  impl Dispenser10 {
    pub fn new(next: Option<Rc<Box<Dispenser>>>) -> Dispenser10 {
      Dispenser10 { next }
    }
  }
  impl Dispenser for Dispenser10 {
    fn amount(&self) -> i32 { 10 }
    fn next(&self) -> Option<Rc<Box<Dispenser>>> { 
      if let Some(ref d) = self.next {
        Some(Rc::clone(&d))
      } else {
        None
      }
    }
  }

  pub struct Dispenser5 {
    next: Option<Rc<Box<Dispenser>>>,
  }

  impl Dispenser5 {
    pub fn new(next: Option<Rc<Box<Dispenser>>>) -> Dispenser5 {
      Dispenser5 { next }
    }
  }
  impl Dispenser for Dispenser5 {
    fn amount(&self) -> i32 { 5 }
    fn next(&self) -> Option<Rc<Box<Dispenser>>> { 
      if let Some(ref d) = self.next {
        Some(Rc::clone(&d))
      } else {
        None
      }
    }
  }

  pub struct ATM {
    dispenser: Box<Dispenser>,
  }
  impl ATM {
    pub fn new() -> ATM {
      ATM {
        dispenser: {
          let d1 = Dispenser5::new(None);
          let d2 = Dispenser10::new(Some(Rc::new(Box::new(d1))));
          let d3 = Dispenser20::new(Some(Rc::new(Box::new(d2))));
          Box::new(Dispenser50::new(Some(Rc::new(Box::new(d3)))))
        }       
      }
    }
    pub fn request_money(&self, money: Money) {
      if money.amount % 5 != 0 {
        println!("The smallest nominal is 5 and we can not satisfy your request.");
      } else {
        self.dispenser.dispense(money);
      }
    }
  }
}

pub mod closure {
  use std::rc::Rc;
  use super::Money;

  pub struct Dispenser {
    amount: i32,
  }
  impl Dispenser {
    pub fn new(amount: i32) -> Dispenser { Dispenser { amount } }
    pub fn dispense(&self, money: Money) -> Result<Money, ()> {
      if money.amount >= self.amount {
        let notes = money.amount / self.amount;
        let left = money.amount % self.amount;
        println!("Dispensing {} note/s of {}.", notes, self.amount);
        if left > 0 {
          Ok(Money::new(left))
        }
        else {
          Err(())
        }
      } else {
        Ok(money)
      }
    }
  }  

  pub struct ATM {}
  impl ATM {
    pub fn request_money(money: Money) {
      if money.amount % 5 != 0 {
        println!("The smallest nominal is 5 and we can not satisfy your request.");
      } else {
        let _ = Dispenser::new(50).dispense(money)
          .and_then(|m| Dispenser::new(20).dispense(m))
          .and_then(|m| Dispenser::new(10).dispense(m))
          .and_then(|m| Dispenser::new(5).dispense(m));
      }
    }
  }
}
