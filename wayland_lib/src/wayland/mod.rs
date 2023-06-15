use wayland_client::Proxy;

pub mod registry;
pub mod compositor;
pub mod output;
pub mod seat;
pub mod shm;
pub mod keyboard;
pub mod pointer;
pub mod surface;
pub mod region;
pub mod buffer;
pub mod shm_pool;
pub mod touch;

pub trait ProxyWrapper {
    type Target : Proxy +'static;
    fn get_proxy( &self) -> &Self::Target ;
    fn from_proxy( value : Self::Target ) ->  Self;
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::ops::Deref;
    use std::os::linux::raw::stat;
    use std::rc::Rc;
    use std::thread;
    use std::thread::Thread;
    use crate::wayland::compositor::{Compositor, CompositorData};
    use crate::wayland::keyboard::{Keyboard, KeyboardData};
    use crate::wayland::output::{Output, OutputData, OutputHandler};
    use crate::wayland::registry::Registry;
    use crate::wayland::seat::Seat;

    use crate::wayland::pointer::PointerData;
    use crate::wayland::shm::Shm;


    use std::cell::RefCell;
    use std::fs::File;
    use std::os::fd::AsRawFd;
    use wayland_client::{Connection, delegate_dispatch, Dispatch, EventQueue, Proxy, QueueHandle, WEnum};
    use wayland_client::protocol::{wl_compositor::*,
                                   wl_display::*,
                                   wl_registry::*,
                                   wl_seat::*,
                                   wl_shm::*  ,
                                   wl_buffer:: * ,
                                   wl_keyboard::* ,
                                   wl_output::* ,
                                   wl_pointer::*,
                                   wl_shm_pool::*,
                                   wl_surface::* ,
                                   wl_touch::*
    };
    use wayland_client::protocol::*;
    use wayland_client::protocol::wl_region::WlRegion;
    use wayland_client::protocol::wl_surface::WlSurface;
    use wayland_client::protocol::wl_touch::WlTouch;
    use wayland_protocols::xdg::shell::client::{xdg_surface::*, xdg_toplevel::*, xdg_wm_base::*};
    use wayland_protocols::xdg::shell::client::*;
    use wayland_protocols::wp::presentation_time::client::* ;
    use wayland_protocols::wp::viewporter::client::* ;
    use wayland_protocols::wp::viewporter::client::wp_viewport::WpViewport;
    use wayland_protocols::wp::viewporter::client::wp_viewporter::WpViewporter;
    use std::sync::{Arc, Mutex};
    use wayland_backend::client::ObjectData;
    use wayland_client::protocol::wl_registry::Request::Bind;
    use crate::{delegate_wl_buffer, delegate_wl_compositor, delegate_wl_keyboard, delegate_wl_output, delegate_wl_pointer, delegate_wl_registry, delegate_wl_seat, delegate_wl_shm, delegate_wl_shm_pool, delegate_wl_surface, delegate_xdg_surface, delegate_xdg_toplevel, delegate_xdg_wm_base, xdg};
    use crate::wayland::ProxyWrapper;
    use crate::wayland::region::RegionData;
    use crate::wayland::shm_pool::ShmPool;
    use crate::wayland::surface::SurfaceHandler;
    use crate::xdg::toplevel::ToplevelData;
    use crate::xdg::wm_base::{WmBase, WmBaseHandler};

//use crate::wayland::application::Application;

    struct State {
        is_closed: bool
    }

    impl WmBaseHandler for State {
        fn ping(&mut self, serial: u32, conn: &Connection, queue_handle: &QueueHandle<Self>)  {

        }
    }

    impl xdg::surface::SurfaceHandler for State{
        fn configure(&mut self, serial: u32, conn: &Connection, queue: &QueueHandle<Self>) where Self: Sized {

        }
    }

    impl xdg::toplevel::ToplevelHandler for State {
        fn configure(&mut self, width: i32, height: i32, state: Vec<u8>, conn: &Connection, queue_handle: &QueueHandle<Self>) where Self: Sized {
            println!("Toplevel Configure  width  : {}  ,height {}  , state {:?}" , width , height , state)
        }

        fn close(&mut self, conn: &Connection, queue_handle: &QueueHandle<Self>) where Self: Sized {
            self.is_closed =true ;
            println!("Toplevel close");
        }

        fn configure_bound(&mut self, width: i32, height: i32, conn: &Connection, queue_handle: &QueueHandle<Self>) where Self: Sized {
            println!("Toplevel Configure bound width : {} , height {}" , width ,height  )
        }

        fn wm_capabilities(&mut self, capabilities: Vec<u8>, conn: &Connection, queue_handle: &QueueHandle<Self>) where Self: Sized {
            println!("Toplevel wm_capabilities  {:?} " ,  capabilities)
        }
    }

    impl SurfaceHandler for State {
        fn enter(& mut self, connection: &Connection, queue_handler: &QueueHandle<Self>) {
            println!("Surface enter")
        }

        fn leave(& mut self, connection: &Connection, queue_handler: &QueueHandle<Self>) {
            println!("Surface leave")
        }
    }

    impl OutputHandler for State {
        fn geometry(&mut self, x: i32, y: i32, physical_width: i32, physical_height: i32, subpixel: Subpixel, make: String, model: String, transform: Transform, conn: &Connection, q_handle: &QueueHandle<Self>) where Self: Sized {
            println!("Output geometry  x : {}  , y : {} physical_width  : {} physical_height: {} , subpixel : {:?} , make : {}  , mode : {}  transform  :{:?}"  , x ,  y , physical_width , physical_height , subpixel   , make , model  ,transform)
        }

        fn mode(&mut self, is_current: bool, is_preferred: bool, width: i32, height: i32, refresh: i32, conn: &Connection, q_handle: &QueueHandle<Self>) where Self: Sized {
            println!( "Output mode is_current : {} , is_preferred : {} , width : {} , height : {} ,refresh : {}"  , is_current , is_preferred , width , height ,refresh  )
        }

        fn done(&mut self, conn: &Connection, q_handle: &QueueHandle<Self>) where Self: Sized {
            println!("Output done ")
        }

        fn scale(&mut self, factor: i32, conn: &Connection, q_handle: &QueueHandle<Self>) where Self: Sized {
            println!( "Output scale  factor : {} "  , factor )
        }

        fn name(&mut self, name: String, conn: &Connection, q_handle: &QueueHandle<Self>) where Self: Sized {
            println!( "Output name  : {} "  , name )
        }

        fn description(&mut self, description: String, conn: &Connection, q_handle: &QueueHandle<Self>) where Self: Sized {
            println!( "Output description , {}" ,  description )
        }
    }
    delegate_wl_registry!(State);
    delegate_wl_compositor!(State);
    delegate_wl_shm!(State);
    delegate_wl_surface!(State) ;
    delegate_wl_seat!(State);
    delegate_wl_output!(State) ;
    delegate_wl_keyboard!(State);
    delegate_wl_pointer!(State);
    delegate_wl_buffer!(State);
    delegate_wl_shm_pool!(State);
    delegate_xdg_wm_base!(State);
    delegate_xdg_surface!(State);
    delegate_xdg_toplevel!(State) ;








    /*




    impl Dispatch<WlSurface, ()> for State {
        fn event(state: &mut Self, proxy: &WlSurface, event: wl_surface::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
            match event {
                wl_surface::Event::Enter { output } => {

                }
                wl_surface::Event::Leave { output } => {
                    println!("Surface Leave")
                }
                _ => {}
            }
        }
    }

    impl Dispatch<WlShm, ()> for State {
        fn event(state: &mut Self, proxy: &WlShm, event: wl_shm::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
            match event {
                wl_shm::Event::Format { format } => {
                    match format  {
                        WEnum::Value(_) => {}
                        WEnum::Unknown(_) => {}
                    }
                }
                _ => {}
            }
        }
    }


    impl Dispatch<WlShmPool, ()> for State {
        fn event(state: &mut Self, proxy: &WlShmPool, event: wl_shm_pool::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
            match event {
                _ => ,

            }
        }
    }

    impl Dispatch<WlBuffer, ()> for State {
        fn event(state: &mut Self, proxy: &WlBuffer, event: wl_buffer::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
            match event {
                wl_buffer::Event::Release => { }
                _ => {}
            }
        }
    } */


    #[test]
    fn ee( ){

    }
    fn draw(tmp: &mut File, (buf_x, buf_y): (u32, u32)) {
        use std::{cmp::min, io::Write};
         let mut buf = std::io::BufWriter::new(tmp);
        for y in 0..buf_y {
            for x in 0..buf_x {
                let a = 0xFF;
                let r = min(((buf_x - x) * 0xFF) / buf_x, ((buf_y - y) * 0xFF) / buf_y);
                let g = min((x * 0xFF) / buf_x, ((buf_y - y) * 0xFF) / buf_y);
                let b = min(((buf_x - x) * 0xFF) / buf_x, (y * 0xFF) / buf_y);

                let color = (a << 24) + (r << 16) + (g << 8) + b;
                buf.write_all(&color.to_ne_bytes()).unwrap();
            }
        }
        buf.flush().unwrap();
    }


    #[test ]
    fn test_wayland (){
        let connection = Connection::connect_to_env().unwrap() ;
        let mut event_queue =  connection.new_event_queue::<State>();
        //let mut event_queue  = connection.new_event_queue::<State>();
        let queue_handle  = event_queue.handle() ;
        //let queue_handle  = event_queue.handle()  ;
        let mut state =  State{ is_closed: false } ;
        let registry = Registry::new(&connection, &event_queue.handle()    ) ;
        //let a  = thread::spawn(move |  |{}) ;
        event_queue.roundtrip( &mut state ).unwrap() ;
        let compositor  = Compositor::new(&registry, &queue_handle,    ).unwrap();
        let seat = Seat::new(&registry, &queue_handle ).unwrap() ;
        let shm =  Shm::new(&registry, &queue_handle ).unwrap();
        let output = Output::new(&registry, &queue_handle ).unwrap() ;
        let wm_base = WmBase::new(&registry  , &queue_handle).unwrap() ;
        event_queue.roundtrip( &mut state ).unwrap() ;

        let surface= compositor.create_surface( &queue_handle , ) ;
        let xdg_surface = wm_base.get_surface(  &surface   ,  &queue_handle, ) ;
        let xdg_toplevel = xdg_surface.get_toplevel(&queue_handle ) ;
        xdg_toplevel.set_title("title".to_string()  ) ;
        //xdg_toplevel.set_maximized() ;
        let mut file = tempfile::tempfile().unwrap();
        let (w, h) = (300, 300);
        draw(&mut file, (w, h));
        let pool = shm.create_pool(file, (w * h * 4) as i32, &queue_handle  );
        let buffer = pool.create(0, (w) as i32, h as i32, (w as i32) * 4,
                                 wl_shm::Format::Argb8888, &queue_handle).unwrap() ;
            // surface.set_input_region(Some( &a )) ;
        surface.attach(Some(&buffer), 0, 0);
        surface.commit() ;



        let mut file = tempfile::tempfile().unwrap();
        let (w, h) = (300, 300);
        draw(&mut file, (w, h));
        let pool = shm.create_pool(file, (w * h * 4) as i32, &queue_handle  );
        let buffer = pool.create(0, (w) as i32, h as i32, (w as i32) * 4,
                                 wl_shm::Format::Argb8888, &queue_handle).unwrap() ;

        surface.commit() ;
        //let keyboard = seat.get_keyboard(&queue_handle ,  KeyboardData::new()  ).unwrap() ;
        //let keyboard2 = seat.get_keyboard(&queue_handle ,  KeyboardData::new()  ).unwrap() ;
       // let pointer =  seat.get_pointer(&queue_handle , PointerData{} ).unwrap();

        thread::spawn(move ||{
            loop {
                if state.is_closed {
                    break
                }
                event_queue.roundtrip (&mut state).unwrap();
            }

        }).join().unwrap()



    }
}
