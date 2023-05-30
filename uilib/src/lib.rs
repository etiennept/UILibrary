use std::ops::Deref;

mod application;
mod wayland;


struct A {


}

impl Deref for A {
    type Target = ();

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::collections::{BTreeSet, HashSet};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    use wayland_protocols::xdg::shell::client::xdg_toplevel::State;
    use super::*;

    #[test]
    fn it_works() {
        let mut a = Mutex::new(1 ) ;
        *a.lock().unwrap() = 4   ;
        println!("{}" ,  a.lock().unwrap() )



    }
}
