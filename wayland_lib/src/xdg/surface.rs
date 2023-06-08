use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::xdg::shell::client::xdg_surface;
use crate::proxy;
use wayland_protocols::xdg::shell::client::xdg_surface::{Event, XdgSurface};
use wayland_protocols::xdg::shell::client::xdg_toplevel::XdgToplevel;
use crate::wayland::ProxyWrapper;
use crate::xdg::toplevel::{Toplevel, ToplevelData};
proxy!( Surface , XdgSurface ) ;
pub struct SurfaceData  ;


impl < T : Dispatch<XdgSurface  , SurfaceData  >> Dispatch<XdgSurface  , SurfaceData   , T > for Surface{
    fn event(state: &mut T, proxy: &XdgSurface, event: xdg_surface::Event, data: &SurfaceData, conn: &Connection, qhandle: &QueueHandle<T>) {
        match event {
            Event::Configure { serial } => {

            }
            _ => {}
        }
    }
}

/*#[macro_export]
#[ macro_use]
macro_rules! delegate_surface   {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_protocols::xdg::shell::client::xdg_surface::XdgSurface : $crate::xdg::surface::SurfaceData  ]=>$crate::xdg::surface::Surface) ;
    };
} */

impl Surface{
    fn get_toplevel <T  :Dispatch<XdgToplevel , ToplevelData > + 'static>(&self , qt : &QueueHandle<T>  ) -> Toplevel {
        Toplevel::from_proxy( self.ptr.get_toplevel( qt  , ToplevelData ) )
    }
    fn get_popup <T  >(&self){


    }
}



impl Drop for Surface{
    fn drop(&mut self) {
        self.ptr.destroy()
    }
}