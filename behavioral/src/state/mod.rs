pub mod std {
  //! `std` module.
  //!
  //! # Example
  //! 
  //! ```rust
  //! use behavioral::state::std::*;
  //! 
  //! let mut player = MediaPlayer::new();
  //! 
  //! player.press_play_or_pause_button();
  //! player.press_play_or_pause_button();
  //! player.press_play_or_pause_button();
  //! player.press_play_or_pause_button();
  //! ```

  pub trait State {
    fn press(&self) -> Box<State>;
    fn eject(&self) -> Box<State>;
  }

  #[derive(Debug)]
  pub struct Playing;
  impl State for Playing {
    fn press(&self) -> Box<State> {
      println!("{:?} -> paused.", self);
      Box::new(Paused)
    }
    fn eject(&self) -> Box<State> {
      println!("{:?} -> No action", self);
      Box::new(Playing)
    }
  }

  #[derive(Debug)]
  pub struct Paused;
  impl State for Paused {
    fn press(&self)  -> Box<State> {
      println!("{:?} -> playing.", self);
      Box::new(Playing)
    }
    fn eject(&self) -> Box<State> {
      println!("{:?} -> Ejected cd.", self);
      Box::new(Ejected)
    }
  }

  #[derive(Debug)]
  pub struct Ejected;
  impl State for Ejected {
    fn press(&self)  -> Box<State> {
      println!("{:?} -> No action", self);
      Box::new(Ejected)
    }
    fn eject(&self) -> Box<State> {
      println!("{:?} -> Paused", self);
      Box::new(Paused)
    }
  }

  pub struct MediaPlayer
  {
    state: Box<State>,
  }
  impl MediaPlayer {
    pub fn new() -> MediaPlayer {
      MediaPlayer { state: Box::new(Paused) }
    }
    pub fn press_play_or_pause_button(&mut self) {
      print!("Press play or pause button...");
      self.state = self.state.press();
    }
    pub fn press_eject_insert_button(&mut self) {
      print!("Press eject or insert button...");
      self.state = self.state.eject();
    }
  }

}
