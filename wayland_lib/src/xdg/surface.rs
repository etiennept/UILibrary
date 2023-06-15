use std::sync::Mutex;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::xdg::shell::client::xdg_surface;
use crate::{handler, proxy};
use wayland_protocols::xdg::shell::client::xdg_surface::{Event, XdgSurface};
use wayland_protocols::xdg::shell::client::xdg_toplevel::{State, XdgToplevel};
use crate::wayland::ProxyWrapper;
use crate::xdg::toplevel::{Toplevel, ToplevelData};
proxy!( Surface , XdgSurface ) ;
pub struct SurfaceData  ;


handler!{
    trait SurfaceHandler{
        fn configure (  serial : u32 , )  ;
    }
}

impl < T : Dispatch<XdgSurface  , SurfaceData  > + SurfaceHandler>  Dispatch<XdgSurface  , SurfaceData   , T > for Surface{
    fn event(state: &mut T, proxy: &XdgSurface, event: xdg_surface::Event, data: &SurfaceData, conn: &Connection, qhandle: &QueueHandle<T>)  {
        match event {
            Event::Configure { serial } => {
                state.configure(serial , conn ,  qhandle)

            }
            _ => {}
        }
    }
}

#[macro_export]
#[ macro_use]
macro_rules! delegate_xdg_surface   {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_protocols::xdg::shell::client::xdg_surface::XdgSurface : $crate::xdg::surface::SurfaceData  ]=>$crate::xdg::surface::Surface) ;
    };
}

impl Surface{
    pub fn get_toplevel <T  :Dispatch<XdgToplevel , ToplevelData > + 'static>(&self , qt : &QueueHandle<T>  ) -> Toplevel {
        Toplevel::from_proxy( self.ptr.get_toplevel( qt  , ToplevelData{ } ) )
    }
    pub  fn get_popup <T  >(&self){


    }
    pub fn ack_configure (&self ,serial : u32){
        self.ptr.ack_configure(serial)
    }
}



impl Drop for Surface{
    fn drop(&mut self) {
        self.ptr.destroy()
    }
}