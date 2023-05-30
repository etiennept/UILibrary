mod application;
mod registry;
mod compositor;
mod output;
mod seat;

use std::fs::File;
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
use std::collections::HashMap ;
use std::sync::Arc;
use wayland_backend::client::ObjectData;
use crate::wayland::registry::{GlobalListPtr, Registry, RegistryData, RegistryState};
use crate::wayland::seat::{SeatData, SeatState};

//use crate::wayland::application::Application;

struct State {

}


impl Dispatch<WlRegistry , GlobalListPtr   >  for  State    {
    fn event(state: &mut Self, proxy: &WlRegistry, event: wl_registry::Event, data: &GlobalListPtr, conn: &Connection, qhandle: &QueueHandle<State>) {
        RegistryState::event( state , proxy, event, data, conn, qhandle )
    }
}

impl Dispatch<WlCompositor, ()> for State {
    fn event(state: &mut Self, proxy: &WlCompositor, event: wl_compositor::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        todo!()
    }
}

impl Dispatch<WlSeat, SeatData> for State {
    fn event(state: &mut Self, proxy: &WlSeat, event: wl_seat::Event, data: &SeatData, conn: &Connection, qhandle: &QueueHandle<Self>) {
        SeatState::event(state , proxy , event  , data ,  conn , qhandle )
    }
}


/*
fn draw(tmp: &mut File, (buf_x, buf_y): (u32, u32)) {
    use std::{cmp::min, io::Write};
    /* let mut buf = std::io::BufWriter::new(tmp);
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
    buf.flush().unwrap(); */
}




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
            _ => todo!(),

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
}
struct A {

}


impl Dispatch<WlKeyboard , () > for State {
    fn event(state: &mut Self, proxy: &WlKeyboard, event: wl_keyboard::Event, data: &() , conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event {
            wl_keyboard::Event::Keymap { format, fd, size } => {
                println!("KeyBoard Keymap  size {} " , size)
            }
            wl_keyboard::Event::Enter { serial, surface, keys } => {

            }
            wl_keyboard::Event::Leave { serial, surface } => {
                println!("KeyBoard Leave {}" , serial)
            }
            wl_keyboard::Event::Key { serial, time, key, state } => {
                println!("KeyBoard key { }  time : {} , serial : {}" , key , time , serial ) ;
            }
            wl_keyboard::Event::Modifiers { serial, mods_depressed, mods_latched, mods_locked, group } => {
                println!("Keyboard Modifiers  serial :{} , mod_depressed {} , mods_latched {}, mod_lode {} , group {}" , serial , mods_depressed , mods_latched , mods_latched  , group)
            }
            wl_keyboard::Event::RepeatInfo { rate, delay } => {
                println!("repeatInfo  rate : {} , delay {}" , rate  , delay )
            }
            _ => {

            }
        }
    }
}



impl Dispatch<WlPointer , () > for State {
    fn event(state: &mut Self, proxy: &WlPointer, event: wl_pointer::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event  {
            wl_pointer::Event::Enter { serial, surface, surface_x, surface_y } => {
                println!("Pointer Enter ,serial : {} ,  surface_x : {} , surface_y  : {}  " , serial , surface_x , surface_y  )

            }
            wl_pointer::Event::Leave { serial, surface } => {
                println!("Pointer Leave serial : {}"  , serial)
            }
            wl_pointer::Event::Motion { time, surface_x, surface_y } => {
                println!("Pointer Motion  time {} , surface_x {} surface_y {}" , time , surface_x , surface_x  )
            }
            wl_pointer::Event::Button { serial, time, button, state } => {
                let  state = match state.into_result().unwrap() {
                    ButtonState::Released => {"is_released"}
                    ButtonState::Pressed => {"is_pressed"}
                    _=> { "error "}
                } ;
                println!( "Pointer Button   serial {} , time  {} button {}  state {}  "  , serial , time , button , state   )

            }
            wl_pointer::Event::Axis { time, axis, value } => {}
            wl_pointer::Event::Frame => {
                println!("Pointer  Frame")
            }
            wl_pointer::Event::AxisSource { axis_source } => {}
            wl_pointer::Event::AxisStop { time, axis } => {}
            wl_pointer::Event::AxisDiscrete { axis, discrete } => {}
            wl_pointer::Event::AxisValue120 { axis, value120 } => {}
            _ => {}
        }
    }
}

impl Dispatch<WlTouch, ()> for State {
    fn event(state: &mut Self, proxy: &WlTouch, event: wl_touch::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event {
            wl_touch::Event::Down { serial, time, surface, id, x, y } => {
                //   println!("Touch Down serial : {} , time  {} , id : {} , x :{} , y :{} " ,  serial ,  time , id , x ,y  )
            }
            wl_touch::Event::Up { serial, time, id } => {}
            wl_touch::Event::Motion { time, id, x, y } => {}
            wl_touch::Event::Frame => {}
            wl_touch::Event::Cancel => {}
            wl_touch::Event::Shape { id, major, minor } => {}
            wl_touch::Event::Orientation { id, orientation } => {}
            _ => {}
        }
    }
}



impl Dispatch< WlOutput, () > for State{
    fn event(state: &mut Self, proxy: &WlOutput, event: wl_output::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event {
            wl_output::Event::Geometry { x, y, physical_width, physical_height, subpixel, make, model, transform } => {

                match subpixel.into_result().unwrap() {
                    Subpixel::Unknown => {}
                    Subpixel::None => {}
                    Subpixel::HorizontalRgb => {}
                    Subpixel::HorizontalBgr => {}
                    Subpixel::VerticalRgb => {}
                    Subpixel::VerticalBgr => {}
                    _ => {}
                }
                match  transform.into_result().unwrap()  {
                    Transform::Normal => {}
                    Transform::_90 => {}
                    Transform::_180 => {}
                    Transform::_270 => {}
                    Transform::Flipped => {}
                    Transform::Flipped90 => {}
                    Transform::Flipped180 => {}
                    Transform::Flipped270 => {}
                    _ => {}
                }
                println!("Output Geometry x {}  y {}  physical_width {} physical_height {} , " , x ,y ,physical_width , physical_height )

            }
            wl_output::Event::Mode { flags, width, height, refresh } => {
                println!("Output Mode width :{} ; height {} refresh {}" , width , height , refresh) ;
                let a = flags.into_result().unwrap()  ;

                match flags.into_result().unwrap() {
                    Mode { .. } => {

                    }
                }
            }
            wl_output::Event::Done => { println!(
                "Output Done"
            )}
            wl_output::Event::Scale { factor } => {
                println!( "Output Scale {}" ,   factor)
            }
            wl_output::Event::Name { name } => {println!("Output Name : {}" , name )}
            wl_output::Event::Description { description } => {  println!("Output  Description {}" , description )}
            _ => {}
        }
    }
}


impl Dispatch<XdgWmBase, ()> for State {
    fn event(state: &mut Self, proxy: &XdgWmBase, event: xdg_wm_base::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event {
            xdg_wm_base::Event::Ping { serial } => {
                println!("Xdg_wm_base serial {}" , serial) ;
                proxy.pong(serial);
            }
            _ => { }
        }

    }
}

impl Dispatch<XdgSurface ,()> for State {
    fn event(state: &mut Self, proxy: &XdgSurface, event: xdg_surface::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event  {
            xdg_surface::Event::Configure { serial } => {
                println!("Xdg_surface Configure serial {}" , serial) ;
                proxy.ack_configure(serial);
            }
            _ => {

            }
        } ;
    }
}

impl Dispatch<XdgToplevel, ()> for State {
    fn event(state: &mut Self, proxy: &XdgToplevel, event: xdg_toplevel::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event {
            xdg_toplevel::Event::Configure { width, height, states } => {
                println!("XdgToplevel Configure width {} height {} " , width , height )
            }
            xdg_toplevel::Event::Close => {
                println!("XdgToplevel Destroy") ;
                proxy.destroy() ;
                //state.running  = false
            }
            xdg_toplevel::Event::ConfigureBounds { width, height } => {
                println!("XdgToplevel ConfigureBound  width {}  height {}" , width ,height )
            }
            xdg_toplevel::Event::WmCapabilities { capabilities } => {
                println!("XdgToplevel WmCapabilities")
            }
            _ => {

            }
        }
    }
}

impl Dispatch<WpViewporter , ()> for State {
    fn event(state: &mut Self, proxy: &WpViewporter, event: wp_viewporter::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
    }
}
impl Dispatch<WpViewport, ()> for State {
    fn event(state: &mut Self, proxy: &WpViewport, event: wp_viewport::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {

    }
}

impl Dispatch<WlRegion, ()> for State {
    fn event(state: &mut Self, proxy: &WlRegion, event: wl_region::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {

    }
} */



impl State {
    fn new(& mut  self , q_handle : &QueueHandle<State>) {
/*
        let mut file = tempfile::tempfile().unwrap();
        let shm = self.shm.as_ref().unwrap() ;
        let (w, h) = (300, 300);
        draw(&mut file, (w, h));
        let pool = shm.create_pool(file.as_raw_fd(), (w * h * 4) as i32, q_handle, ());
        let buffer = pool.create_buffer(0, (w) as i32, h as i32, (w as i32) * 4,
                                        wl_shm::Format::Argb8888, &q_handle, ());
        self.ee (  & buffer , q_handle  ) ; */
    }




    fn ee (& self  , buffer : &WlBuffer    , q_handle :  &QueueHandle<State>){
       /* let compositor  =  self.compositor.as_ref().unwrap() ;
        let surface = compositor.create_surface(q_handle , () );
        let xdg_surface = self.xdg_wm_base.as_ref().unwrap().get_xdg_surface(  &surface   ,  q_handle, () ) ;
        surface.id() ;
        let xdg_toplevel = xdg_surface.get_toplevel(q_handle , ()) ;
        xdg_toplevel.set_title("title".to_string()  ) ;
        xdg_toplevel.set_maximized() ;

        /* let a = compositor.create_region(q_handle ,  () ) ;
         a.add(100 , 100 , 100 , 100) ;

         surface.set_input_region(Some( &a )) ; */
        surface.attach(Some(&buffer.clone()), 0, 0);

        surface.commit() ;



        let wl_viewporter = self.wp_viewport.as_ref().unwrap() ;
        wl_viewporter.get_viewport( &surface ,  q_handle , ()  ) ; */

    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::ops::Deref;
    use std::os::linux::raw::stat;
    use std::rc::Rc;
    use wayland_client::Connection;
    use wayland_client::protocol::wl_compositor::WlCompositor;
    use crate::wayland::registry::Registry;
    use crate::wayland::State;


    #[test]
    fn ee( ){

    }


    #[test ]
    fn test_wayland (){
        let connection = Connection::connect_to_env().unwrap() ;
        let mut event_queue = connection.new_event_queue()  ;
        let queue_handle  = event_queue.handle() ;
        let mut state  = State{ };
        let  a = Registry::new(connection  , &queue_handle   ) ;

       // let registry = display.get_registry( queue_handle  , ()  );
        event_queue.roundtrip(&mut state).unwrap();
        let x = a.bind::<WlCompositor  , _ , _ >(  &queue_handle  , ()   ).unwrap();
        //x.create_surface(&queue_handle , () ) ;

        //registries .get("wl_compositor").unwrap().clone()  ;




    }
}