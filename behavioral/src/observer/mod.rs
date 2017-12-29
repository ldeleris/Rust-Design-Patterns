pub mod std {
  //! `std` module.
  //!
  //! # Example
  //! 
  //! ```rust
  //! use behavioral::observer::std::*;
  //! 
  //! let user_yvan = User::new("Ivan");
  //! let user_maria = User::new("Maria");
  //! let user_john = User::new("John");
  //! 
  //! println!("Create a post...");
  //! let mut post = Post::new(user_yvan.clone(), "This is a post about the observer design pattern.");
  //! 
  //! println!("Add a comment...");
  //! post.add_comment(Comment::new(user_yvan.clone(), "I hope you like the post!"));
  //!     
  //! println!("Maria and John subscribe to the comments");
  //! post.add_observer(Box::new(user_john));
  //! post.add_observer(Box::new(user_maria.clone()));
  //! 
  //! println!("Add a comment...");
  //! post.add_comment(Comment::new(user_yvan.clone(), "Why are you so quiet? Do you like it?"));
  //! 
  //! println!("Add a comment...");
  //! post.add_comment(Comment::new(user_maria.clone(), "It is amazing! Thanks!"));
  //! ```
  
  use std;

  pub trait Observer<T> 
    where Self: std::fmt::Debug
  {
    fn handle_update(&self, subject: &T);
    fn print(&self) -> String {
        format!("{:?}", self)
    }
  }

  pub trait Observable {
    type Item;
    fn add_observer(&mut self, observer: Box<Observer<Self::Item>>);
    fn notify_observers(&mut self);
  }

  #[derive(Debug, Clone)]
  pub struct User {
    name: String,
  }
  impl User {
    pub fn new(name: &str) -> User {
      User { name: String::from(name) }
    }
  }
  impl Observer<Post> for User 
    where Post: std::fmt::Debug  {
    fn handle_update(&self, subject: &Post) {
      println!("Hey, I'm {}. The post got some new comments: {:#?}", self.name, subject.comments);
    }
  }

  #[derive(Debug, Clone)]
  pub struct Comment {
    user: User,
    text: String,
  }
  impl Comment {
    pub fn new(user: User, text: &str) -> Comment {
      Comment {
        user,
        text: String::from(text),
      }
    }
  }

  

  #[derive(Debug)]
  pub struct Post 
  {
    user: User,
    text: String,
    comments: Vec<Comment>,
    observers: Vec<Box<Observer<Post>>>,
  }
  impl Post 
    where Self: std::fmt::Debug
  {
    pub fn new(user: User, text: &str) -> Post {
      Post {
        user,
        text: String::from(text),
        comments: Vec::new(),
        observers: Vec::new(),
      }
    }
    pub fn add_comment(&mut self, comment: Comment) {
      self.comments.push(comment);
      self.notify_observers();
    }
    fn print(&self) -> String {
        format!("{:?}", self)
    }
    pub fn comments(&self) -> Vec<Comment> { self.comments.clone() }
  }
  impl Observable for Post {
    type Item = Post;
    fn add_observer(&mut self, observer: Box<Observer<Self::Item>>) {
      self.observers.push(observer);
    }
    fn notify_observers(&mut self) {
      self.observers.iter().for_each(|o| o.handle_update(&self));
    } 
  }  
}