use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::protocol::wl_pointer;
use wayland_client::protocol::wl_pointer::{Event, WlPointer};
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_protocols::xdg::shell::client::xdg_toplevel::State;
use crate::proxy;
use crate::wayland::seat::{Seat, SeatError};

proxy!(Pointer , WlPointer );

impl Pointer{
    fn set_cursor (&self, serial   :u32, surface: Option<&WlSurface>  , x : i32, y : i32  ){
        self.ptr.set_cursor(serial, surface,x  ,   y  )
    }
}

impl Drop for Pointer{
    fn drop(&mut self) {
        self.ptr.release()
    }
}

pub struct PointerData{
    
}

impl <T : Dispatch<WlPointer, PointerData> > Dispatch<WlPointer, PointerData , T > for Pointer {
    fn event(state: &mut T , proxy: &WlPointer, event: wl_pointer::Event, data: &PointerData, conn: &Connection, qhandle: &QueueHandle<T>) {
        match event  {
            Event::Enter { serial, surface, surface_x, surface_y } => {}
            Event::Leave { serial, surface } => {}
            Event::Motion { time, surface_x, surface_y } => {}
            Event::Button { serial, time, button, state } => {}
            Event::Axis { time, axis, value } => {}
            Event::Frame => {}
            Event::AxisSource { axis_source } => {}
            Event::AxisStop { time, axis } => {}
            Event::AxisDiscrete { axis, discrete } => {}
            Event::AxisValue120 { axis, value120 } => {}
            _ => {}
        }
    }
}
#[macro_export]
macro_rules! delegate_pointer   {
    ( $name:ident   ) => {
        wayland_client::delegate_pointer!( $name : [ wayland_client::protocol::wl_pointer::WlPointer : $crate::wayland::pointer::PointerData ]=>$crate::wayland::pointer::Pointer) ;
    };
}