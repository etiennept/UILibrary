use std::borrow::{Borrow, BorrowMut};
use std::cmp::min;
use std::collections::{BTreeSet, HashMap, HashSet};

use std::sync::{Arc, Mutex};
use wayland_client::{Connection, delegate_dispatch, Dispatch, Proxy, QueueHandle};
use wayland_client::globals::BindError;

use wayland_client::protocol::wl_registry;
use wayland_client::protocol::wl_registry::WlRegistry;
use wayland_protocols::xdg::shell::client::xdg_toplevel::State;

pub(crate) enum RegistryState {}
 //delegate_dispatch!( RegistryState :[wl ])

pub(crate) struct RegistryData {
    content: Mutex<HashMap<String , ( u32 , u32  )  >>
}

impl RegistryData {
    pub(crate) fn new  () -> RegistryData {
        RegistryData{
            content: Mutex::new(HashMap::new()),
        }
    }
    pub(crate) fn insert (&self ,  name : u32 , interface : String  , version :u32     ){
        self.content.lock().unwrap().insert( interface  , (name , version) ) ;
    }
     pub(crate)  fn get (&self, interface  :&str ) -> Option<(u32, u32)> {
         self.content.lock().unwrap().get(interface).map( |it| {
             it.clone()
         })
     }
}

pub(crate) type GlobalListPtr  = Mutex<HashMap< String , (u32 , u32  ) >>  ;



pub struct Registry {
    wl_registry : WlRegistry ,
}




impl Registry {
    pub fn new <   T : Dispatch<WlRegistry ,GlobalListPtr >  + 'static>(connection : Connection, queue_handle : &QueueHandle<T  >) -> Self {
        let display = connection.display() ;
        let registry = display.get_registry(queue_handle, Mutex::new(Default::default()) ) ;
        Registry{
            wl_registry: registry,
        }
    }
    pub fn bind<T : Proxy + 'static,   R :Send  +Sync  + 'static,  F  :Dispatch<T, R>  + 'static > (& self, qh :& QueueHandle<F >, data : R  ) -> Result< T , BindError> {
        let list= self.wl_registry.data::<GlobalListPtr>().unwrap();
         if let Some(global, ) =  list.lock().unwrap().get(T::interface().name) {
             let (name, version) = global.clone();
             let version = min(T::interface().version, version);
             Ok(self.wl_registry.bind::<T, R, F>(name, version, qh, data))
         } else {
              Err(BindError::NotPresent)
         }
    }
}

impl Dispatch<WlRegistry,GlobalListPtr, super::State > for RegistryState {
    fn event(state: &mut super::State, proxy: &WlRegistry, event: wl_registry::Event, data: &GlobalListPtr, conn: &Connection, q_handle: &QueueHandle<super::State>) {
        match event {
            wl_registry::Event::Global { name, interface, version } => {
                data.lock().unwrap().insert(interface , (name , version));
            }
            wl_registry::Event::GlobalRemove { name } => {



            }
            _ => {}
        };
    }
}



