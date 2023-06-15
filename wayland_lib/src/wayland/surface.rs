use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use wayland_backend::protocol::AllowNull::No;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::protocol::wl_buffer::WlBuffer;
use wayland_client::protocol::wl_surface;
use wayland_client::protocol::wl_surface::{Event, WlSurface};
use crate::{handler, proxy};
use crate::wayland::buffer::Buffer;
use crate::wayland::ProxyWrapper;

proxy!(Surface, WlSurface);



handler!{
    trait SurfaceHandler{
        fn enter () ;
        fn leave () ;
    }
}


impl Surface{
    pub(crate) fn attach (&self, buffer : Option<&Buffer>, x:i32, y :i32   ) {
        if let Some(buffer ) =  buffer  {
            let a= &buffer.get_proxy().clone()    ;
            self.ptr.attach(   Some(a ) , x, y  )
        }else {
            self.ptr.attach(None , x, y  )
        } ;
        //let a = buffer.map( |buffer |{  })   ;
    }
    fn damage(&self  , x  :i32 , y : i32   ,with : i32 , height :i32  ){
        self.get_proxy().damage(  x ,  y  , with , height )
    }
    fn offset(&self ,  x : i32  , y:i32   ){
        self.ptr.offset(x ,y )
    }
    //fn frame (&self  ){ }
    pub(crate) fn commit (&self ){
        self.ptr.commit()
    }
}
pub struct SurfaceData {


}
impl <T :Dispatch<WlSurface , SurfaceData>  + SurfaceHandler > Dispatch<WlSurface, SurfaceData , T  > for Surface{
    fn event(state: &mut T, proxy: &WlSurface, event: wl_surface::Event, data: &SurfaceData, conn: &Connection, qhandle: &QueueHandle<T>) {
        match event  {
            Event::Enter { output } => {
                state.enter(conn , qhandle)
            }
            Event::Leave { output } => {
                state.leave(conn , qhandle)
            }
            _ => {

            }
        }
    }
}

#[macro_export]
macro_rules! delegate_wl_surface {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_surface::WlSurface : $crate::wayland::surface::SurfaceData ]=>$crate::wayland::surface::Surface) ;
    };
}

impl Drop for Surface{
    fn drop(&mut self) {
         self.ptr.destroy()
    }
}