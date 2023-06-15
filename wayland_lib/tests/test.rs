use std::fs::File;

use std::os::unix::io::AsRawFd;
use std::process::Output;
use std::{env, thread};
use std::ffi::c_void;
use std::ptr::{null, null_mut};

use wayland_client::{Connection, Dispatch, EventQueue, Proxy, QueueHandle, WEnum};
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


struct State {
    running : bool,
    surface : Option<WlSurface>  ,
    compositor: Option<WlCompositor>,
    shm: Option<WlShm>,
    seat: Option<WlSeat>,
    output : Option<WlOutput>,
    xdg_wm_base: Option<XdgWmBase>,
    wp_viewport : Option<WpViewporter> ,
}


impl Dispatch<WlRegistry, ()> for State {
    fn event(state: &mut Self, proxy: &WlRegistry, event: wl_registry::Event, data: &(), conn: &Connection, q_handle: &QueueHandle<Self>) {
        match event {
            wl_registry::Event::Global { name, interface, version } => {
                match interface.as_str() {
                    "wl_compositor" => {
                        let compositor = proxy.bind::<WlCompositor, _, _>(name, version, q_handle, ());
                        state.compositor = Some(compositor);
                    }
                    "wl_shm" => {
                        let shm = proxy.bind::<WlShm, _, _>(name, version, q_handle, ());
                        state.shm = Some(shm);
                    }
                    "wl_output"=>{
                        let output  = proxy.bind::<WlOutput , _ , _>( name   , version ,q_handle , () );
                        state.output = Some(output)
                    }
                    "xdg_wm_base" => {
                        let xdg_wm_base = proxy.bind::<XdgWmBase, _, _>(name, version, q_handle, ());
                        state.xdg_wm_base = Some(xdg_wm_base);
                    }
                    "wp_viewporter" =>{
                        let viewporter = proxy.bind::<WpViewporter, _ ,_  >( name  , version , q_handle  , () ) ;
                        state.wp_viewport  = Some(viewporter) ;
                    }
                    "wl_seat" => {
                        let seat = proxy.bind::<WlSeat, _, _>(name, version, q_handle, ());
                        state.seat = Some(seat) ;
                    }
                    _ => {}
                }

            }
            wl_registry::Event::GlobalRemove { name } => {

            }
            _ => {}
        };
    }
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


impl Dispatch<WlCompositor, ()> for State {
    fn event(state: &mut Self, proxy: &WlCompositor, event: wl_compositor::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {}
}

impl Dispatch<WlSurface, ()> for State {
    fn event(state: &mut Self, proxy: &WlSurface, event: wl_surface::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event {
            wl_surface::Event::Enter { output } => {
                println!("Surface Enter")
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

impl Dispatch<WlSeat, ()> for State {
    fn event(state: &mut Self, proxy: &WlSeat, event: wl_seat::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event {
            wl_seat::Event::Capabilities { capabilities } => {
                println!("Seat Capabilities" ) ;
                let capabilities = capabilities.into_result().unwrap() ;
                if capabilities.contains( Capability::Pointer )  {
                    proxy.get_pointer( qhandle , ()) ;
                    println!("pointer init") ;
                }
                if  capabilities.contains(  Capability::Keyboard  ) {
                    proxy.get_keyboard( qhandle, () ) ;
                    println!("keyboard init") ;

                }


                /*if a.contains(Capability::Touch )   {
                     proxy.get_touch(qhandle, () )
                    println!( "touch init")
                } */


                /*
                match capabilities.into_result().unwrap() {
                    Capability::Keyboard   =>{



                    }
                    Capability::Touch  =>{

                        proxy.get_touch(qhandle , ()  );
                    }
                    Capability::Pointer =>{
                        println!("pointer init") ;
                        proxy.get_pointer(qhandle , () ) ;
                    }
                    _=>{
                        println!("error")
                    }
                } */


            }
            wl_seat::Event::Name { name } => {
                println!("Seat Name {}" , name )
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



impl Dispatch<WlKeyboard , () > for State {
    fn event(state: &mut Self, proxy: &WlKeyboard, event: wl_keyboard::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event {
            wl_keyboard::Event::Keymap { format, fd, size } => {
                println!("KeyBoard Keymap  size {} " , size)
            }
            wl_keyboard::Event::Enter { serial, surface, keys } => {
                println!("KeyBoard Enter serial : {}" , serial)
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
                state.running  = false
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
}



impl State {
    fn new(& mut  self , q_handle : &QueueHandle<State>) {

        let mut file = tempfile::tempfile().unwrap();
        let shm = self.shm.as_ref().unwrap() ;
        let (w, h) = (300, 300);
        draw(&mut file, (w, h));
        let pool = shm.create_pool(file.as_raw_fd(), (w * h * 4) as i32, q_handle, ());
        let buffer = pool.create_buffer(0, (w) as i32, h as i32, (w as i32) * 4,
                                          wl_shm::Format::Argb8888, &q_handle, ());
        self.ee (  & buffer , q_handle  ) ;
    }




    fn ee (& self  , buffer : &WlBuffer    , q_handle :  &QueueHandle<State>){
         let compositor  =  self.compositor.as_ref().unwrap() ;
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
        wl_viewporter.get_viewport( &surface ,  q_handle , ()  ) ;

    }
}








#[test]
fn test() {

     /*   let conn = Connection::connect_to_env().unwrap();
        let x = conn.display();

        let a  = unsafe { GetDisplay(x.id().as_ptr() as *const c_void ) };
        let con = vec! [
            EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
            EGL_RED_SIZE, 8,
            EGL_GREEN_SIZE, 8,
            EGL_BLUE_SIZE, 8,
            EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
            EGL_NONE
        ] ;
        unsafe { Initialize(a ,  null_mut() , null_mut()); }
        //unsafe {CreateContext( a ,   con as * const c_void  )  }


        let mut event_queue = conn.new_event_queue();
        let r = &event_queue.handle();
        x.get_registry(r, ());
        let mut state =   State { running : true  , surface: None, compositor: None, shm: None, seat: None, output: None, xdg_wm_base: None, wp_viewport: None };
        event_queue.roundtrip(&mut state).unwrap();
        state.new(&r  );
        while state.running{
            event_queue.roundtrip( &mut state).unwrap();
        } */


}
#[test]
fn ee(){



}