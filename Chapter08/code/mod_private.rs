mod library {
  pub struct Interface {
    m_impl: Impl   // private field
  }
  
  impl Interface {
    pub fn new() -> Interface {
      Interface{ m_impl: Impl }
    }
    
    pub fn f(&self){
      self.m_impl.f();
    }
  }
  
  struct Impl;
  
  impl Impl {
    fn f(&self){ // private method
      println!("f");
    }
  }
}

fn main() {
  let o = library::Interface::new();
  o.f();
}
// f