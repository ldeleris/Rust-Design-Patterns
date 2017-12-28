
pub mod std {
  //! `std` module.
  //!
  //! # Example
  //! 
  //! ```rust
  //! use behavioral::command::std::*;
  //! use std::rc::Rc;
  //! 
  //! let robot = Robot;
  //! let mut robot_controller = RobotController::new();
  //! println!("{}", robot_controller.issue_command(Rc::new(MakeSandwichCommand::new(robot.clone()))));
  //! println!("{}", robot_controller.issue_command(Rc::new(PourJuiceCommand::new(robot.clone()))));
  //! println!("I'm eating and having some juice.");
  //! println!("{}", robot_controller.issue_command(Rc::new(CleanupCommand::new(robot.clone()))));
  //! println!("Here is what I asked my robot to do:");
  //! robot_controller.show_history();
  //! ```
  
  use std::rc::Rc;
  use std;

  #[derive(Debug, PartialEq, Hash, Clone)]
  pub struct Robot;
  impl Robot {
    pub fn cleanup(&self) -> String {
      String::from("Cleaning up.")
    }
    pub fn pour_juice(&self) -> String {
      String::from("Pouring juice.")
    }
    pub fn make_sandwich(&self) -> String {
      String::from("Making a sandwich.")
    }
  }

  pub trait RobotCommand 
    where Self: std::fmt::Debug
  {
    fn execute(&self) -> String;
    fn print(&self) -> String {
        format!("{:?}", self)
    }
  }
    

  #[derive(Debug, PartialEq, Hash, Clone)]
  pub struct MakeSandwichCommand {
    robot: Robot,
  }
  impl MakeSandwichCommand {
    pub fn new(robot: Robot) -> Box<RobotCommand> {
      Box::new(MakeSandwichCommand { robot })
    }
  }
  impl RobotCommand for MakeSandwichCommand {
    fn execute(&self) -> String {
      self.robot.make_sandwich()
    }
  }

  #[derive(Debug, PartialEq, Hash, Clone)]
  pub struct PourJuiceCommand {
    robot: Robot,
  }
  impl PourJuiceCommand {
    pub fn new(robot: Robot) -> Box<RobotCommand + 'static> {
      Box::new(PourJuiceCommand { robot })
    }
  }
  impl RobotCommand for PourJuiceCommand {
    fn execute(&self) -> String {
      self.robot.pour_juice()
    }
  }

  #[derive(Debug, PartialEq, Hash, Clone)]
  pub struct CleanupCommand {
    robot: Robot,
  }
  impl CleanupCommand {
    pub fn new(robot: Robot) -> Box<RobotCommand> {
      Box::new(CleanupCommand { robot })
    }
  }
  impl RobotCommand for CleanupCommand {
    fn execute(&self) -> String {
      self.robot.cleanup()
    }
  }

  pub struct RobotController {
    history: Vec<Rc<Box<RobotCommand>>>,
  }
  impl RobotController {
    pub fn new() -> RobotController {
      RobotController { history: Vec::new() }
    }
    pub fn issue_command(&mut self, command: Rc<Box<RobotCommand>>) -> String {
      self.history.push(Rc::clone(&command));
      command.execute()
    }
    pub fn show_history(&self) {
      for h in &self.history {
        println!("{:?}", h);
      }
    }
  }
}
