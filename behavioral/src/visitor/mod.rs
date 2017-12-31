pub mod std {
  //! `std` module.
  //!
  //! # Example
  //! 
  //! ```rust
  //! use behavioral::visitor::std::*;
  //! 
  //! let mut document = Document::new(
  //!   vec!(
  //!     Element::title("The Visitor Pattern Example."),
  //!     Element::text("The visitor pattern helps us add extra functionality without changing the classes."),
  //!     Element::hyperlink("Go check it online!", "https://www.google.com/"),
  //!     Element::text("Thanks")
  //!   )
  //! );
  //! 
  //! let mut html_exporter = HtmlExporterVisitor::new();
  //! println!("Export to html:");
  //! document.accept(&mut html_exporter);
  //! println!("{}", html_exporter.get_html()); 
  //! 
  //! let mut plain_text_exporter = PlainTextExporterVisitor::new();
  //! println!("Export to plain text:");
  //! document.accept(&mut plain_text_exporter);
  //! println!("{}", plain_text_exporter.get_text()); 
  //! ```

  use std::rc::Rc;
  use std::cell::RefCell;

  #[derive(Clone, Debug)]
  pub enum Element {
      Title { text: String },
      Text { text: String },
      Hyperlink { text: String, url: String },
  }
  impl Element {
      pub fn accept<T: Visitor + 'static>(&self, visitor: &mut T) {
          (*visitor).visit((*self).clone());
      }
      pub fn title(text: &str) -> Element {
          Element::Title { text: String::from(text) }
      }
      pub fn text(text: &str) -> Element {
          Element::Text { text: String::from(text) }
      }
      pub fn hyperlink(text: &str, url: &str) -> Element {
          Element::Hyperlink {
              text: String::from(text),
              url: String::from(url)
          }
      }
  }

  pub struct Document {
      parts: Rc<RefCell<Vec<Element>>>,
  }
  impl Document {
      pub fn new(parts: Vec<Element>) -> Document {
          Document { parts: Rc::new(RefCell::new(parts)) }
      }
      pub fn accept<T: Visitor + 'static>(&mut self, visitor: &mut T) {
          self.parts.borrow().iter().for_each(move |p| p.accept(visitor));
      }
  }

  pub trait Visitor {
    const END_LINE: &'static str = "\n";
    fn visit(&mut self, element: Element);
  }

  pub struct HtmlExporterVisitor {
    builder: String,
  }
  impl HtmlExporterVisitor {
      pub fn new() -> HtmlExporterVisitor {
          HtmlExporterVisitor { builder: String::new() }
      }
      pub fn get_html(&self) -> String {
          self.builder.clone()
      }
  }
  impl Visitor for HtmlExporterVisitor {
      fn visit(&mut self, element: Element) {
          match element {
              Element::Title { text } => {
                  let line = format!("<h1>{}</h1>{}", text, Self::END_LINE);
                  self.builder.push_str(&line[..]);
              },
              Element::Text { text } =>  {
                  let line = format!("<p>{}</p>{}", text, Self::END_LINE);
                  self.builder.push_str(&line[..]);
              },
              Element::Hyperlink { text, url} =>  {
                  let line = format!("<a herf=\"{}\">{}</a>{}", url, text, Self::END_LINE);
                  self.builder.push_str(&line[..]);
              },           
          }
      }
  }

  pub struct PlainTextExporterVisitor {
      builder: String,
  }
  impl PlainTextExporterVisitor {
      pub fn new() -> PlainTextExporterVisitor {
          PlainTextExporterVisitor { builder: String::new() }
      }
      pub fn get_text(&self) -> String {
          self.builder.clone()
      }
  }
  impl Visitor for PlainTextExporterVisitor {
      fn visit(&mut self, element: Element) {
          match element {
              Element::Title { text } => {
                  let line = format!("{}{}", text, Self::END_LINE);
                  self.builder.push_str(&line[..]);
              },
              Element::Text { text } =>  {
                  let line = format!("{}{}", text, Self::END_LINE);
                  self.builder.push_str(&line[..]);
              },
              Element::Hyperlink { text, url} =>  {
                  let line = format!("{}({}){}", text, url, Self::END_LINE);
                  self.builder.push_str(&line[..]);
              },           
          }
      }
  }
}