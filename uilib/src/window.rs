use std::process::Output;
use std::thread;
use wayland_client::EventQueue;
use wayland_client::protocol::wl_shm::Event;
use wayland_lib::{delegate_wl_surface, wayland, xdg};
use wayland_lib::wayland::compositor::Compositor;

use wayland_lib::wayland::keyboard::Keyboard;
use wayland_lib::wayland::pointer::Pointer;
use wayland_lib::wayland::ProxyWrapper;
use wayland_lib::wayland::seat::Seat;
use wayland_lib::wayland::touch::Touch;
use wayland_lib::xdg::surface::Surface;
use wayland_lib::xdg::toplevel::Toplevel;
use crate::State;

pub struct WindowState {
    surface : wayland::surface::Surface,
    keyboard : Option<Keyboard>,
    pointer : Option<Pointer>,
    touch  : Option<Touch>,
    shell_surface : xdg::Surface ,
    toplevel : Toplevel ,
}

pub struct Window{
    event_queue : EventQueue<WindowState > ,
    state : WindowState ,
}

impl Window {
    pub(crate) fn  new (event_queue :EventQueue<WindowState >,
                        compositor  : Compositor,
                        seat  : Seat  ,
                        output : Output ,
                        shell : xdg::wm_base::WmBase  ,
                         ) -> Window {
        let a =  &event_queue.handle() ;
        let surface  =  compositor.create_surface(a) ;
        let shell_surface = shell.get_surface( &surface ,a  ) ;
        /*let keyboard  = seat.get_keyboard( &a     ).ok();
        let pointer = seat.get_pointer(&a ).ok() ;
        let touch = seat.get_touch(&a  ).ok() ; */
        let toplevel = shell_surface.get_toplevel(  a );
        Window{
            event_queue,
            state: WindowState {
                surface,
                keyboard: None,
                pointer: None,
                touch: None,
                shell_surface,
                toplevel,
            },
        }
    }

    fn run (&mut self){
        let a  = thread::spawn( ||{
            loop {
                self.event_queue.blocking_dispatch( & mut self.state )
            }
        });
        a.join().unwrap();



    }

}
delegate_wl_surface!(WindowState) ;