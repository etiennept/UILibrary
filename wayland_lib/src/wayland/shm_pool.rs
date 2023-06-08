use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};
use wayland_client::protocol::wl_buffer::WlBuffer;
use wayland_client::protocol::wl_shm::Format;
use wayland_client::protocol::wl_shm_pool;
use wayland_client::protocol::wl_shm_pool::WlShmPool;
use crate::proxy;
use crate::wayland::buffer::{Buffer, BufferData};
use crate::wayland::ProxyWrapper;
use crate::wayland::shm::Shm;
use crate::wayland::shm_pool::ShmPoulError::FormatNotImpl;
proxy!(ShmPool , WlShmPool);

#[derive(Debug)]
pub enum ShmPoulError {
    FormatNotImpl
}



impl Display for ShmPoulError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f ,"eeeeee")
    }
}

impl Error for ShmPoulError {

}





impl ShmPool{
    pub fn create <T : Dispatch<WlBuffer , BufferData  > + 'static> (&self, offset :i32, width :i32, height :i32, stride :i32, format : Format, qt : &QueueHandle<T >   ) -> Result<Buffer, ShmPoulError> {
        let formats = &self.ptr.data::<ShmPoolData>().unwrap ().format ;
        if formats.get( &format     ).is_some()  {
            Ok(Buffer::from_proxy(self.ptr.create_buffer( offset , width , height ,stride ,  format , qt , BufferData   ) ))
        }  else {
            Err(FormatNotImpl )
        }

    }
    pub fn resize (&self, size :i32){
        self.ptr.resize( size ) ;
    }
}
pub struct ShmPoolData{
    pub(crate) format : HashSet<Format>
}
impl <T : Dispatch<WlShmPool , ShmPoolData> > Dispatch<WlShmPool, ShmPoolData , T  > for  ShmPool {
    fn event(state: &mut T, proxy: &WlShmPool, event: wl_shm_pool::Event, data: &ShmPoolData, conn: &Connection, qhandle: &QueueHandle<T>) {
        todo!()
    }
}

#[macro_export]
macro_rules! delegate_shm_pool  {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_shm_pool::WlShmPool : $crate::wayland::shm_pool::ShmPoolData ]=>$crate::wayland::shm_pool::ShmPool) ;
    };
}

impl Drop for ShmPool{
    fn drop(&mut self) {
        self.ptr.destroy()
    }
}