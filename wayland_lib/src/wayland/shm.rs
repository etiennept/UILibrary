use std::collections::HashSet;
use std::ops::Deref;
use std::os::fd::AsRawFd;
use std::sync::Mutex;
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};
use wayland_client::globals::BindError;
use wayland_client::protocol::wl_shm;
use wayland_client::protocol::wl_shm::{Event, Format, WlShm};
use wayland_client::protocol::wl_shm_pool::WlShmPool;
use crate::proxy;
use crate::wayland::ProxyWrapper;
use crate::wayland::registry::Registry;
use crate::wayland::shm_pool::{ShmPool, ShmPoolData};

proxy!(Shm , WlShm) ;
impl Shm {
    pub fn new  <T  :Dispatch<WlShm , ShmData> +'static >(registry  :&Registry, qh : &QueueHandle<T >  ) -> Result<Shm, BindError> {
         registry.bind(qh, ShmData::default() )
    }
    pub fn create_pool<T : AsRawFd  , F  : Dispatch < WlShmPool , ShmPoolData  ,  F >+ 'static >  (&mut self, fd : T, size : i32, qt : &QueueHandle<F>, ){
        let a=  self.ptr.data::<ShmData>().unwrap().formats.lock().unwrap().clone() ;
        ShmPool::from_proxy(self.ptr.create_pool( fd.as_raw_fd() ,   size  , qt    , ShmPoolData{ format: a  }));
    }
}

#[macro_export]
macro_rules! delegate_shm   {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_shm::WlShm : $crate::wayland::shm::ShmData]=>$crate::wayland::shm::Shm) ;
    };
}

#[derive(Default)]
pub struct ShmData {
    formats : Mutex< HashSet<wl_shm::Format>  >
}
impl <T : Dispatch<WlShm , ShmData >  >  Dispatch<WlShm, ShmData , T >for Shm {
    fn event(state: &mut T , proxy: &WlShm, event: wl_shm::Event, data: &ShmData, conn: &Connection, qhandle: &QueueHandle<T>) {
        match event  {
            Event::Format { format } => {
                println!( "{:?}", format.into_result().unwrap() ) ;
                data.formats.lock().unwrap().insert( format.into_result().unwrap()  ) ;
            }
            _ => {}
        }
    }
}





