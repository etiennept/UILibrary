use std::error::Error;
use std::fmt::{Debug, Display, Formatter, write};
use std::sync::Mutex;
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};
use wayland_client::globals::BindError;
use wayland_client::protocol::wl_keyboard::WlKeyboard;
use wayland_client::protocol::wl_pointer::WlPointer;
use wayland_client::protocol::wl_registry::WlRegistry;
use wayland_client::protocol::wl_seat;
use wayland_client::protocol::wl_seat::{Capability, WlSeat};
use wayland_client::protocol::wl_touch::WlTouch;
use crate::proxy;
use crate::wayland::keyboard::Keyboard;
use crate::wayland::pointer::Pointer;
use crate::wayland::registry::Registry;
use crate::wayland::seat::SeatError::NotImpl;
use crate::wayland::State;
use crate::wayland::touch::Touch;
use crate::wayland::ProxyWrapper;

proxy!(Seat , WlSeat ) ;


#[derive( Default )]
struct Contain{
    keyboard  : bool,
    pointer : bool,
    touch : bool,
}
pub  struct SeatData{
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

macro_rules! seat_impl {
    ( $name:ident , $ty:ident,  $wl_ty:ty, $data_name:ident  ) => {
        pub fn $name < D : Send +Sync +'static , T : Dispatch< $wl_ty  ,D>+'static   >(&self , qt : &QueueHandle<T> ,  data : D  ) ->  Result<$ ty , SeatError>{
            if self.ptr.data::<SeatData>().unwrap().contain.lock().unwrap().$data_name  {
                Ok( $ty::from_proxy( self.ptr.$name (qt  , data)  )  )
            }else {
                Err(NotImpl)
            }
    }};
}

#[macro_export]
macro_rules! delegate_seat  {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_seat::WlSeat : $crate::wayland::seat::SeatData]=>$crate::wayland::seat::Seat) ;
    };
}
impl Seat {
    pub fn new < T : Dispatch<WlSeat, SeatData> + 'static> (registry : &Registry, qh : &QueueHandle<T > ) -> Result<Seat, BindError> {
        registry.bind( qh,   SeatData::new()    )
    }


    seat_impl!(get_keyboard , Keyboard, WlKeyboard , keyboard     ) ;
    seat_impl!(get_pointer,  Pointer , WlPointer , pointer  ) ;
    seat_impl!(get_touch,  Touch ,WlTouch , touch ) ;
}

impl <T : Dispatch<WlSeat ,SeatData  >>  Dispatch<WlSeat, SeatData , T  > for Seat {
    fn event(state: &mut T , proxy: &WlSeat, event: wl_seat::Event, data: &SeatData , conn: &Connection, qhandle: &QueueHandle<T >) {
         ;
        match event {

            wl_seat::Event::Capabilities { capabilities } => {
                println!("capabilities") ;
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
        self.ptr.release()
    }
}
