
use wayland_client::protocol::wl_keyboard::WlKeyboard;
use wayland_client::{Dispatch, QueueHandle};
use crate::wayland::seat::Seat;

struct Keyboard {
    wl_keyboard :  WlKeyboard
}
impl Keyboard {
    fn new < T  : Dispatch<Keyboard  ,  ()  > >(seat : &Seat  , qt : &QueueHandle<T > ) -> Keyboard {
        Keyboard{
            wl_keyboard:  seat.get_keyboard( qt , () ).unwrap(),
        }
    }
}



impl Drop for Keyboard {
    fn drop(&mut self) {
        self.wl_keyboard.release()
    }
}