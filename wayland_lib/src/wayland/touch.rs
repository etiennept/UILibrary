use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::protocol::wl_touch;
use wayland_client::protocol::wl_touch::WlTouch;
use crate::proxy;
use crate::wayland::ProxyWrapper;

proxy!( Touch  , WlTouch  ) ;

pub struct TouchData {}

impl TouchData {
    pub(crate) fn new ( ) -> Self {
        TouchData{ }
    }
}

impl <T : Dispatch<WlTouch ,  TouchData > > Dispatch<WlTouch ,  TouchData  , T  >  for Touch {
    fn event(state: &mut T, proxy: &WlTouch, event: wl_touch::Event, data: &TouchData, conn: &Connection, qhandle: &QueueHandle<T>) {

    }
}

#[macro_export]
macro_rules! delegate_wl_touch   {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_touch::WlTouch : $crate::wayland::touch::TouchData ]=>$crate::wayland::touch::Touch) ;
    };
}

impl Drop for Touch{
    fn drop(&mut self) {
        self.get_proxy().release()
    }
}