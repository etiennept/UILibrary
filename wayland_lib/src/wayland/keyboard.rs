use std::os::fd::AsRawFd;
use std::sync::Mutex;
use wayland_client::protocol::wl_keyboard::{Event, KeymapFormat, KeyState, WlKeyboard};
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::protocol::wl_keyboard;
use xkbcommon::xkb;
use xkbcommon::xkb::{Context, Keymap, State};
use xkbcommon::xkb::ffi::XKB_CONTEXT_NO_FLAGS;

use crate::{handler, proxy};
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


unsafe impl Send for KeyboardData {}
// SAFETY: The state is guarded by a mutex since libxkbcommon has no internal synchronization.
unsafe impl Sync for KeyboardData {}

#[macro_export]
macro_rules! delegate_wl_keyboard {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_keyboard::WlKeyboard : $crate::wayland::keyboard::KeyboardData ]=>$crate::wayland::keyboard::Keyboard) ;
    };
}

enum NoKeyMap {

}

pub struct KeyboardData {
     state:Mutex<Option<State> > ,
}

impl KeyboardData {
    pub(crate) fn new () -> KeyboardData {
        KeyboardData{
            state: Mutex::new(None ),
        }
    }
}
handler!{
    trait KeyboardHandler   {
        fn keymap (    )  ;
        fn enter () ;
        fn leave ( ) ;
        fn key ( ) ;
        fn modifier () ;
        fn repeatInfo () ;

    }

}
impl <T : Dispatch<WlKeyboard ,KeyboardData > > Dispatch<WlKeyboard ,KeyboardData  , T  > for Keyboard  {
    fn event(state: &mut T , proxy: &WlKeyboard, event: wl_keyboard::Event, data: &KeyboardData, conn: &Connection, qhandle: &QueueHandle<T>) {
        match  event {
            Event::Keymap { format, fd, size } => {
                match format.into_result().unwrap()   {
                    KeymapFormat::NoKeymap => {
                        println!("no keymap" )
                    }
                    KeymapFormat::XkbV1 => {
                        println!(" xkv_v1 ") ;
                        let keymap =  unsafe{
                            xkb::Keymap::new_from_fd(&Context::new(XKB_CONTEXT_NO_FLAGS),
                                                                fd.as_raw_fd(),
                                                                size as usize,
                                                                xkb::KEYMAP_FORMAT_TEXT_V1,
                                                                xkb::COMPILE_NO_FLAGS,
                            )
                        }.unwrap() ;
                        if let Some( keymap ) = keymap {
                            *data.state.lock().unwrap() = Some(  State::new( &keymap ) )
                        }

                    }
                    _ => {
                        panic!("e")
                    }
                }
            }
            Event::Enter { serial, surface, keys } => {
                println!("KeyBoard Enter  {:?}" , keys )
            }
            Event::Leave { serial, surface } => {
                println!("KeyBoard Leave ")
            }
            Event::Key { serial, time, key, state } => {
                let xkb_state   = data.state.lock().unwrap();
                let a= match state.into_result().unwrap() {
                    KeyState::Released => {"released"}
                    KeyState::Pressed => {
                        "pressed"
                    }
                    _ => {panic!("eeee")}
                } ;
                println!( "Key state :{} , time : {} , key : {}" , a  , time  , key )



            }
            Event::Modifiers { serial, mods_depressed, mods_latched, mods_locked, group } => {

                println!("Modifier info  mods_depressed : {} , mods_latched : {} , mods_locked : {} group : {}" , mods_depressed , mods_latched  , mods_locked , group)
            }
            Event::RepeatInfo { rate, delay } => {
                println!("RepeatInfo  rate : {} , delay {}" , rate , delay)
            }
            _ => {}
        }
    }
}





impl Drop for Keyboard {
    fn drop(&mut self) {
        self.ptr.release() ;
        //println!("keyboard release")
    }
}