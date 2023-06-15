use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::protocol::wl_buffer;
use wayland_client::protocol::wl_buffer::WlBuffer;
use crate::proxy;
proxy!(Buffer , WlBuffer);



#[macro_export]
macro_rules! delegate_wl_buffer {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_buffer::WlBuffer : $crate::wayland::buffer::BufferData]=>$crate::wayland::buffer::Buffer) ;
    };
}
pub struct BufferData ;
impl <T : Dispatch<WlBuffer ,BufferData  >  > Dispatch< WlBuffer ,BufferData , T  > for Buffer{
    fn event(state: &mut T, proxy: &WlBuffer, event: wl_buffer::Event, data: &BufferData, conn: &Connection, qhandle: &QueueHandle<T>) {

    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        self.ptr.destroy()
    }
}