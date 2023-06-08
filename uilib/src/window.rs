use std::thread;
use wayland_client::EventQueue;
use wayland_client::protocol::wl_shm::Event;
use wayland_lib::{delegate_surface, wayland};
use wayland_lib::wayland::keyboard::Keyboard;
use wayland_lib::wayland::pointer::Pointer;
use wayland_lib::wayland::touch::Touch;
use wayland_lib::xdg::surface::Surface;
use crate::State;

pub struct WindowState {

}


pub struct Window{
    event_queue : EventQueue<WindowState > ,
    state : WindowState ,
    surface : wayland::Surface,
    keyboard : Option<Keyboard>,
    pointer : Option<Pointer>,
    touch  : Option<Touch>,




}

impl Window {
    pub(crate) fn  new (event_queue :EventQueue<WindowState >,  surface :Surface  , keyboard : Option<Keyboard> , pointer : Option<Pointer> , touch : Option<Touch>         ) -> Window {
        Window{
            event_queue,
            surface,
            keyboard,
            pointer,
            touch,
            state
        }
    }

    fn run (&mut self){

        let a  = thread::spawn( ||{
            loop {
                self.event_queue.blocking_dispatch(&mut WindowState {})
            }
        });

        a.join().unwrap();



    }

}
delegate_surface!(WindowState) ;