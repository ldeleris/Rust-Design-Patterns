pub mod std {
  //! `std` module.
  //!
  //! # Example
  //! 
  //! ```rust
  //! use behavioral::memento::std::*;
  //!   
  //! ```

  use std::collections::VecDeque;

  #[derive(Clone, Debug)]
  pub struct Memento<T> {
    state: T,
  }
  impl<T> Memento<T> {
    fn new(state: T) -> Memento<T> { Memento { state } }
    fn get_state(&self) -> &T { &self.state }
  }

/*
  pub trait Caretaker<T> {
    fn states(&self) -> VecDeque<Memento<T>>;
  }
*/
  pub struct Caretaker<T> {
    states: VecDeque<Memento<T>>,
  }
  impl<T> Caretaker<T> {
    fn new() -> Caretaker<T> { Caretaker { states: VecDeque::new() } }
  }

  pub trait Originator<T> {
    fn create_memento(&self) -> Memento<T>;
    fn restore(&mut self, memento: Memento<T>);
  }

  struct TextEditor {
    builder: String,
  }
  impl TextEditor {
    pub fn new() -> TextEditor {
      TextEditor { builder: String::new() }
    }

    pub fn append(&mut self, text: &str) {
      self.builder.push_str(text);
    }
    pub fn delete(&mut self) {
      if !self.builder.is_empty() {
        self.builder.pop();
      }
    }
    pub fn text(&self) -> String { self.builder.clone() } 
  }
  
  impl Originator<String> for TextEditor {
    fn create_memento(&self) -> Memento<String> {
      Memento::<String>::new(self.builder.clone())
    }
    fn restore(&mut self, memento: Memento<String>) {
      self.builder = memento.get_state().clone();
    }
  }

  pub struct TextEditorManipulator {
    states: Caretaker<String>,
    text_editor: TextEditor,
  }
  impl TextEditorManipulator {
    pub fn new() -> TextEditorManipulator {
      TextEditorManipulator { 
        states: Caretaker::new(),
        text_editor: TextEditor::new() }
    }
    pub fn save(&mut self) {
      self.states.states.push_front(self.text_editor.create_memento())
    }
    pub fn undo(&mut self) {
      if let Some(s) = self.states.states.pop_front() {
        self.text_editor.restore(s);
      }
    }
    pub fn append(&mut self, text: &str) {
      self.save();
      self.text_editor.append(text);
    }
    pub fn delete(&mut self) {
      self.save();
      self.text_editor.delete();
    }
    pub fn read_text(&mut self) -> String {
      self.text_editor.text()
    }
  }

}