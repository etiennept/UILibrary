use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::globals::BindError;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_protocols::xdg::shell::client::xdg_surface::XdgSurface;
use wayland_protocols::xdg::shell::client::xdg_toplevel::State;
use wayland_protocols::xdg::shell::client::xdg_wm_base;
use wayland_protocols::xdg::shell::client::xdg_wm_base::{Event, XdgWmBase};
use crate::{handler, proxy, wayland};
use crate::wayland::ProxyWrapper;
use crate::wayland::registry::Registry;
use crate::xdg::surface::{Surface, SurfaceData};

proxy!(WmBase,XdgWmBase);

handler!{trait  WmBaseHandler {
        fn ping (serial :u32 ,) ;
    }
}
impl<T: Dispatch<XdgWmBase, WmBaseData> + WmBaseHandler> Dispatch<XdgWmBase, WmBaseData, T> for WmBase {
    fn event(state: &mut T, proxy: &XdgWmBase, event: xdg_wm_base::Event, data: &WmBaseData, conn: &Connection, qhandle: &QueueHandle<T>) {
        match event {
            Event::Ping { serial } => {
               state.ping(serial , conn , qhandle)
            }
            _ => {

            }
        }
    }
}

impl WmBase{
    pub fn new< T  : Dispatch<XdgWmBase , WmBaseData>  +'static > (registry : &Registry, qt : &QueueHandle<T>    ) -> Result<WmBase, BindError> {
        registry.bind::<WmBase, WmBaseData , T >( qt ,  WmBaseData   )
    }

    pub fn get_surface< T : Dispatch<XdgSurface ,SurfaceData > + 'static> (&self, surface : & wayland::surface::Surface ,qt : &  QueueHandle<T >  ) -> Surface {
        Surface::from_proxy(self.ptr.get_xdg_surface(surface.get_proxy() , qt  , SurfaceData))
    }
    pub fn create_positioner (&self  ){

    }
    pub fn pong(&self , serial : u32  ){
        self.ptr.pong(serial)
    }
}

#[macro_export]
macro_rules! delegate_xdg_wm_base {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_protocols::xdg::shell::client::xdg_wm_base::XdgWmBase : $crate::xdg::wm_base::WmBaseData  ]=>$crate::xdg::wm_base::WmBase) ;
    };
}

impl Drop for  WmBase{
    fn drop(&mut self) {
         self.ptr.destroy()
    }
}

pub struct WmBaseData ;

