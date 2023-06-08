use std::cell::RefCell;
use std::sync::Arc;
use wayland_client::Connection;
use wayland_lib::{delegate_compositor, delegate_registry, delegate_seat, delegate_shm, delegate_wm_base};
use wayland_lib::wayland::compositor::Compositor;
use wayland_lib::wayland::output::Output;
use wayland_lib::wayland::registry;
use wayland_lib::wayland::registry::Registry;
use wayland_lib::wayland::seat::Seat;
use wayland_lib::wayland::shm::Shm;
use wayland_lib::xdg::wm_base::WmBase;
use crate::application;

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

        let a

        self.compositor.create_surface( )


    }
}

impl Drop for Application {
    fn drop(&mut self) {

    }
}

delegate_registry!(ApplicationState  ) ;
delegate_shm!(ApplicationState) ;
delegate_seat!(ApplicationState) ;
delegate_compositor!(ApplicationState);
delegate_wm_base!(ApplicationState) ;
