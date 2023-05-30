use std::error::Error;
use std::fmt::{Debug, Display, Formatter, write};
use std::sync::Mutex;
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};
use wayland_client::protocol::wl_keyboard::WlKeyboard;
use wayland_client::protocol::wl_pointer::WlPointer;
use wayland_client::protocol::wl_seat;
use wayland_client::protocol::wl_seat::{Capability, WlSeat};
use wayland_client::protocol::wl_touch::WlTouch;
use crate::wayland::registry::Registry;
use crate::wayland::seat::SeatError::NotImpl;
use crate::wayland::State;

mod keyboard;
mod pointer;



pub(crate) enum SeatState {}
#[derive( Default )]
struct Contain{
    keyboard  : bool,
    pointer : bool,
    touch : bool,
}
pub (crate )  struct SeatData{
    contain  : Mutex<Contain> ,
    name : Mutex<String>
}
impl SeatData {
    fn new () -> SeatData {
        SeatData{
            contain: Mutex::new(Contain{
                keyboard: false,
                pointer: false,
                touch: false,
            }  ),
            name : Mutex::new("".to_string() )
        }
    }
}

#[derive( Debug  )]
pub enum SeatError  {
    NotImpl
}

impl Display for SeatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f , " not implement  " )
    }
}

impl Error for SeatError{

}
struct Seat {
    wl_seat : WlSeat
}

 macro_rules! seat_impl {
    ( $name:ident , $ty:ty, $data_name:ident  ) => {
        pub fn $name < D : Send +Sync +'static , T : Dispatch< $ty  ,D>+'static   >(&self , qt : &QueueHandle<T> ,  data : D  ) ->  Result<$ ty , SeatError>{
            if self.wl_seat.data::<SeatData>().unwrap().contain.lock().unwrap().$data_name  {
                Ok(self.wl_seat.$name (qt  , data)  )
            }else {
                Err(NotImpl)
            }
    }


    };
}

impl Seat {
     fn new < T : Dispatch<WlSeat, SeatData> + 'static> (registry : Registry, qh : &QueueHandle<T > ) -> WlSeat {
        registry.bind( qh,   SeatData::new()    ).unwrap()
    }
    seat_impl!(get_keyboard ,  WlKeyboard , keyboard     ) ;
    seat_impl!(get_pointer, WlPointer , pointer  ) ;
    seat_impl!(get_touch, WlTouch , pointer ) ;
}

impl Dispatch<WlSeat, SeatData , super::State > for SeatState {
    fn event(state: &mut State, proxy: &WlSeat, event: wl_seat::Event, data: &SeatData , conn: &Connection, qhandle: &QueueHandle<super::State>) {
        match event {
            wl_seat::Event::Capabilities { capabilities } => {
                let mut data = data.contain.lock().unwrap();
                let capabilities = capabilities.into_result().unwrap() ;
                data.keyboard = capabilities.contains(  Capability::Keyboard  )  ;
                data.pointer = capabilities.contains( Capability::Pointer ) ;
                data.touch = capabilities.contains(Capability::Touch) ;
            }
            wl_seat::Event::Name { name } => {
                *data.name.lock().unwrap() = name
            }
            _ => {}
        }
    }
}


impl Drop for Seat {
    fn drop(&mut self) {
        self.wl_seat.release()
    }
}