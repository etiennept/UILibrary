use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::protocol::wl_region;
use wayland_client::protocol::wl_region::WlRegion;
use crate::proxy;
use crate::wayland::region;
proxy!(Region , WlRegion) ;

pub struct RegionData ;

impl <T: Dispatch<WlRegion , RegionData >> Dispatch<WlRegion, RegionData , T > for Region {
    fn event(state: &mut T , proxy: &WlRegion, event: wl_region::Event, data: &RegionData, conn: &Connection, qhandle: &QueueHandle<T >) {
        todo!()
    }
}
#[macro_export]
macro_rules! delegate_region {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_region::WlRegion : $crate::wayland::region::RegionData ]=>$crate::wayland::region::Region) ;
    };
}