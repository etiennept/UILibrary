



use std::borrow::{Borrow, BorrowMut};
use std::cmp::min;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::ops::Deref;

use std::sync::{Arc, Mutex};
use wayland_client::{Connection, delegate_dispatch, Dispatch, Proxy, QueueHandle};
use wayland_client::globals::BindError;

use wayland_client::protocol::wl_registry;
use wayland_client::protocol::wl_registry::WlRegistry;
use wayland_protocols::xdg::shell::client::xdg_toplevel::State;
use crate::proxy;

use crate::wayland::{     ProxyWrapper, RegistryState};


//delegate_dispatch!( RegistryState :[wl ])

proxy!( Registry  ,  WlRegistry ) ;




pub struct RegistryData {
    content: Mutex<HashMap<String , ( u32 , u32  )  >>
}



impl RegistryData {
    pub fn new() -> RegistryData {
        RegistryData{
            content: Mutex::new(HashMap::new()),
        }
    }
    pub fn insert (&self ,  name : u32 , interface : String  , version :u32     ){
        self.content.lock().unwrap().insert( interface  , (name , version) ) ;
    }
     pub fn get (&self, interface  :&str ) -> Option<(u32, u32)> {
         self.content.lock().unwrap().get(interface).map( |it| {
             it.clone()
         })
     }
}



#[macro_export]
macro_rules! delegate_registry   {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_registry::WlRegistry : $crate::wayland::registry::RegistryData ]=>$crate::wayland::registry::Registry) ;
    };
}


impl Registry {
    pub fn new < T : Dispatch<WlRegistry , RegistryData>  + 'static>(connection : &Connection, queue_handle : &QueueHandle<T  >) -> Self {
        let display = connection.display() ;
        Registry::from_proxy( display.get_registry(queue_handle,  RegistryData::new())  )

    }
    pub fn bind< T : ProxyWrapper ,R :Send  +Sync  + 'static,  F  :Dispatch< T::Target, R>  + 'static  > (& self, qh :& QueueHandle<F >, data : R  ) -> Result< T , BindError> {
        let list= self.get_proxy().data::<RegistryData>().unwrap();
         if let Some(global) = list.get(T::Target::interface().name) {
             let (name, version) = global;
             let version = min(T::Target::interface().version, version);
             Ok(T::from_proxy(self.get_proxy().bind::<T::Target, R, F>(name, version, qh, data)))
         } else {
             Err(BindError::NotPresent)
         }
    }
}

impl < T : Dispatch<WlRegistry , RegistryData>   >Dispatch< WlRegistry, RegistryData, T  > for Registry {
    fn event(state: &mut T , proxy: &WlRegistry, event: wl_registry::Event, data: &RegistryData, conn: &Connection, q_handle: &QueueHandle<T >) {
        match event {
            wl_registry::Event::Global { name, interface, version } => {
                data.insert(name , interface ,  version);
            }
            wl_registry::Event::GlobalRemove { name } => {



            }
            _ => {}
        };
    }
}



