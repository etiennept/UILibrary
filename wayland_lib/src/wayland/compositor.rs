use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::{Connection, delegate_dispatch, Dispatch, QueueHandle};
use wayland_client::globals::BindError;
use wayland_client::protocol::wl_compositor;
use wayland_client::protocol::wl_region::WlRegion;
use wayland_client::protocol::wl_registry::WlRegistry;
use crate::{proxy, wayland};
use crate::wayland::ProxyWrapper;
use crate::wayland::region::{Region, RegionData};

use crate::wayland::registry::Registry;
use crate::wayland::surface::{Surface, SurfaceData};



proxy!(Compositor  , WlCompositor  );



impl < T : Dispatch<WlCompositor, CompositorData    >    > Dispatch<WlCompositor, CompositorData, T > for  Compositor {
    fn event(state: &mut T, proxy: &WlCompositor, event: wl_compositor::Event, data: &CompositorData, conn: &Connection, qhandle: &QueueHandle<T >) {

    }
}


pub  struct CompositorData  ;
impl Compositor{
    pub fn new< T : Dispatch< WlCompositor , CompositorData   >  +'static> (registry  : &Registry, qt  : &QueueHandle<T > ) -> Result<Compositor, BindError> {
        registry.bind::<Compositor, CompositorData, T > (qt,CompositorData{})
    }
    pub fn create_surface<  T : Dispatch< WlSurface , SurfaceData  >  +'static> (&self, qh : &QueueHandle<T  >, ) -> Surface {
        //log::

        Surface::from_proxy(self.ptr.create_surface(qh,SurfaceData {}  )  )
    }
    pub fn create_region<  T : Dispatch <WlRegion , RegionData>+'static >(&self, qh : &QueueHandle<T >,  ) -> Region {
       Region::from_proxy(self.ptr.create_region( qh, RegionData {}  ) )
    }

}

#[macro_export]
macro_rules! delegate_wl_compositor {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_compositor::WlCompositor : $crate::wayland::compositor::CompositorData]=>$crate::wayland::compositor::Compositor) ;
    };
}








