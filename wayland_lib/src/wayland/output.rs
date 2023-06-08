use std::io::SeekFrom::Current;
use std::sync::Mutex;
use wayland_backend::protocol::WEnum;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::globals::BindError;
use wayland_client::protocol::wl_output;
use wayland_client::protocol::wl_output::{Event,  Subpixel, Transform, WlOutput};
use crate::proxy;


use crate::wayland::registry::Registry;
use crate::wayland::State;
proxy!(Output,  WlOutput ) ;


#[macro_export]
macro_rules! delegate_output   {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_output::WlOutput : $crate::wayland::output::OutputData ]=>$crate::wayland::output::Output) ;
    };
}
#[derive(Default)]
pub struct OutputData {
    geometry : Mutex<Option<Geometry >> ,
    mode : Mutex<Option<Mode>> ,
    scale : Mutex<Option<i32 >>,
    name : Mutex<Option< String >>,
    description : Mutex<Option<String>>
}

impl OutputData {


}

struct Geometry {
    x : i32,
    y : i32,
    physical_width :i32 ,
    physical_height:i32,
    subpixel : Subpixel,
    make:String ,
    model:String ,
    transform :Transform
}
struct Mode {
    is_current : bool ,
    is_preferred : bool ,
    width : i32,
    height :i32 ,
    refresh : i32 ,
}



impl Output {
    pub fn new< T: Dispatch<WlOutput, OutputData> + 'static>(registry: &Registry, qh: &QueueHandle<T>, ) -> Result<Output, BindError> {
        registry.bind::<Output, OutputData, T>(qh,  OutputData::default() )
    }
}

impl Drop for Output{
    fn drop(&mut self ){
        self.ptr.release()
    }
}

impl <T : Dispatch<WlOutput, OutputData> > Dispatch<WlOutput, OutputData ,T > for Output {
    fn event(state: &mut T , proxy: &WlOutput, event: wl_output::Event, data: &OutputData, conn: &Connection, qhandle: &QueueHandle<T >) {
        match event {
            Event::Geometry { x, y,  physical_width, physical_height, subpixel, make, model, transform } => {
                   *data.geometry.lock().unwrap() = Some(Geometry{
                       x,
                       y,
                       physical_width,
                       physical_height,
                       subpixel: subpixel.into_result().unwrap(),
                       make,
                       model,
                       transform: transform.into_result().unwrap(),
                   } )    ;
            }
            Event::Mode { flags, width, height, refresh } => {
                let a = flags.into_result().unwrap() ;
                let is_current = a.contains( wl_output::Mode::Current );
                let is_preferred = a.contains(wl_output::Mode::Preferred);
                *data.mode.lock().unwrap() = Some(Mode{
                    is_current,
                    is_preferred,
                    width,
                    height,
                    refresh,
                } ) ;
                //println!("Mode : width : {}, height :{} , refresh {} , Mode {} , {}", width , height ,refresh  ,  x ,y    )
            }
            Event::Done => {  }
            Event::Scale { factor } => { *data.scale.lock().unwrap() = Some(factor) }
            Event::Name { name } => { *data.name.lock().unwrap() = Some(name)  }
            Event::Description { description } => { *data.description.lock().unwrap() =Some( description ) }
            _ => {}
        }
    }
}
