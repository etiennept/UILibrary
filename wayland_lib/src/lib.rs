extern crate core;

use std::ops::Deref;


pub mod wayland;
pub mod xdg;


pub(crate) mod wrapper {
    #[macro_export]
    macro_rules! proxy {
        ($wrapper_name:ident , $proxy_name:ident) => {
            pub struct $wrapper_name {
                ptr : $proxy_name
            }
            impl crate::wayland::ProxyWrapper for $wrapper_name   {
                type Target = $proxy_name  ;
                fn get_proxy(&self) -> &Self::Target {
                    &self.ptr
                }
                fn from_proxy(value: Self::Target) -> Self {
                    $wrapper_name{
                    ptr : value
                    }
                }
            }
        };
    }
    #[macro_export]
    macro_rules! handler  {
        (trait $name:ident  {
            $(fn $func_name:ident(  $($arg_name:ident : $arg_type:ty , )* )  ;  )+
        }) => {
            pub trait $name {
                  $( fn $func_name (&mut self ,  $($arg_name : $arg_type ,)*  conn  : &Connection , q_handle : &QueueHandle<Self> ) where Self: Sized  ;     )*

            }


        };
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
        let mut a = Mutex::new(1);
        *a.lock().unwrap() = 4;
        println!("{}", a.lock().unwrap())
    }
}
