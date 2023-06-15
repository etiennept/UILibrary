use std::cell::RefCell;
use std::sync::Arc;
use wayland_client::Connection;
use wayland_lib::{delegate_wl_compositor, delegate_wl_registry, delegate_wl_seat, delegate_wl_shm, delegate_xdg_wm_base};
use wayland_lib::wayland::compositor::Compositor;
use wayland_lib::wayland::output::Output;
use wayland_lib::wayland::registry;
use wayland_lib::wayland::registry::Registry;
use wayland_lib::wayland::seat::Seat;
use wayland_lib::wayland::shm::Shm;
use wayland_lib::xdg::wm_base::WmBase;
use crate::application;
use crate::window::WindowState;

struct ApplicationState{

}



pub struct Application {
    connection : Connection,
    compositor : Compositor,
    seat : Seat ,
    output : Output,
    shm : Shm ,
    shell : WmBase ,

}

impl Application {
    fn new () -> Application {
        let connection =  Connection::connect_to_env().unwrap()   ;
        let mut event_queue =  connection. new_event_queue::<ApplicationState>(  );
        let queue_hanble = &event_queue.handle() ;
        let registry = Registry::new(&connection, queue_hanble) ;
        let mut state=   &mut ApplicationState {} ;
        event_queue.roundtrip(state ).unwrap();
        let application = Application {
            connection,
            compositor: Compositor::new (&registry   , &queue_hanble).unwrap()   ,
            seat: Seat::new( &registry  ,&queue_hanble  ).unwrap() ,
            output:Output::new(&registry ,&queue_hanble).unwrap() ,
            shm: Shm::new(&Registry ,&queue_hanble  ).unwrap()  ,
            shell: WmBase::new( &Registry ,&queue_hanble ).unwrap(),
        } ;
        event_queue.roundtrip( state ).unwrap() ;
        return application
    }
    fn create_window(&self ){
        let a = self.connection.new_event_queue::<WindowState>() ;



    }
}

impl Drop for Application {
    fn drop(&mut self) {

    }
}

delegate_wl_registry!(ApplicationState  ) ;
delegate_wl_shm!(ApplicationState) ;
delegate_wl_seat!(ApplicationState) ;
delegate_wl_compositor!(ApplicationState);
delegate_xdg_wm_base!(ApplicationState) ;
