use wayland_client::protocol::wl_keyboard::{Event, WlKeyboard};
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::protocol::wl_keyboard;
use crate::proxy;
use crate::wayland::compositor::Compositor;
use crate::wayland::seat::{Seat, SeatError};
proxy!(Keyboard  , WlKeyboard );

impl Keyboard {
   /* pub fn new < T  : Dispatch<WlKeyboard  ,  () > + 'static >(seat : &Seat, qt : &QueueHandle<T > ) -> Result<Keyboard, SeatError> {
        seat.get_keyboard( qt , () ).map(|it |{
            Keyboard{
                ptr :  it  ,
            }
        })
    } */
}
/*
impl Dispatch<   > {
    
} */
pub trait KeyboardOuput   {
   // fn  ( st    )


}

#[macro_export]
macro_rules! delegate_keyboard   {
    ( $name:ident   ) => {
        wayland_client::delegate_keyboard!( $name : [ wayland_client::protocol::wl_keyboard::WlKeyboard : $crate::wayland::keyboard::KeyboardData ]=>$crate::wayland::keyboard::Keyboard) ;
    };
}
pub struct KeyboardData ;

impl <T : Dispatch<WlKeyboard ,KeyboardData > > Dispatch<WlKeyboard ,KeyboardData  , T  > for Keyboard  {
    fn event(state: &mut T , proxy: &WlKeyboard, event: wl_keyboard::Event, data: &KeyboardData, conn: &Connection, qhandle: &QueueHandle<T>) {
        match  event {
            Event::Keymap { format, fd, size } => {}
            Event::Enter { serial, surface, keys } => { }
            Event::Leave { serial, surface } => {}
            Event::Key { serial, time, key, state } => {}
            Event::Modifiers { serial, mods_depressed, mods_latched, mods_locked, group } => {}
            Event::RepeatInfo { rate, delay } => {
            }
            _ => {}
        }
    }
}





impl Drop for Keyboard {
    fn drop(&mut self) {
        self.ptr.release() ;
        println!("keyboard release")
    }
}