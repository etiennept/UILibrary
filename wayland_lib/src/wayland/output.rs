use std::io::SeekFrom::Current;
use std::sync::Mutex;
use wayland_backend::protocol::WEnum;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_client::globals::BindError;
use wayland_client::protocol::wl_output;
use wayland_client::protocol::wl_output::{Event, Subpixel, Transform, WlOutput};

use crate::{handler, proxy};


use crate::wayland::registry::Registry;

proxy!(Output,  WlOutput );


#[macro_export]
macro_rules! delegate_wl_output {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_client::protocol::wl_output::WlOutput : $crate::wayland::output::OutputData ]=>$crate::wayland::output::Output) ;
    };
}
#[derive(Default)]
pub struct OutputData {
    geometry: Mutex<Option<Geometry>>,
    mode: Mutex<Option<Mode>>,
    scale: Mutex<Option<i32>>,
    name: Mutex<Option<String>>,
    description: Mutex<Option<String>>,
}


struct Geometry {
    x: i32,
    y: i32,
    physical_width: i32,
    physical_height: i32,
    subpixel: Subpixel,
    make: String,
    model: String,
    transform: Transform,
}

struct Mode {
    is_current: bool,
    is_preferred: bool,
    width: i32,
    height: i32,
    refresh: i32,
}



handler! {
    trait OutputHandler {
        fn geometry ( x: i32, y :i32 ,physical_width :i32 , physical_height :i32 , subpixel :Subpixel, make :String , model :String, transform  :Transform ,) ;
        fn mode (is_current :bool   , is_preferred : bool  , width :i32,height :i32 , refresh  :i32 , ) ;
        fn done ( );
        fn scale (factor :i32 ,) ;
        fn name (name : String ,) ;
        fn description( description :String , ) ;
    }


}


impl Output {
    pub fn new<T: Dispatch<WlOutput, OutputData> + 'static>(registry: &Registry, qh: &QueueHandle<T>) -> Result<Output, BindError> {
        registry.bind::<Output, OutputData, T>(qh, OutputData::default())
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        self.ptr.release()
    }
}

impl<T: Dispatch<WlOutput, OutputData> +OutputHandler > Dispatch<WlOutput, OutputData, T> for Output {
    fn event(state: &mut T, proxy: &WlOutput, event: wl_output::Event, data: &OutputData, conn: &Connection, qhandle: &QueueHandle<T>) {
        match event {
            Event::Geometry { x, y, physical_width, physical_height, subpixel, make, model, transform } => {
                let subpixel = subpixel.into_result().unwrap();
                let transform =transform.into_result().unwrap() ;
                *data.geometry.lock().unwrap() = Some(Geometry {
                    x,
                    y,
                    physical_width,
                    physical_height,
                    subpixel ,
                    make : make.to_string(),
                    model : make.to_string(),
                    transform ,
                });
                state.geometry(x, y, physical_width, physical_height, subpixel,  make.to_string(), model.to_string(), transform, conn, qhandle )
            }
            Event::Mode { flags, width, height, refresh } => {
                let a = flags.into_result().unwrap();
                let is_current = a.contains(wl_output::Mode::Current);
                let is_preferred = a.contains(wl_output::Mode::Preferred);
                *data.mode.lock().unwrap() = Some(Mode {
                    is_current,
                    is_preferred,
                    width,
                    height,
                    refresh,
                });
                state.mode( is_current , is_preferred , width , height , refresh  , conn ,qhandle)
                //println!("Mode : width : {}, height :{} , refresh {} , Mode {} , {}", width , height ,refresh  ,  x ,y    )
            }
            Event::Done => {
                state.done(  conn ,qhandle )
            }
            Event::Scale { factor } => { *data.scale.lock().unwrap() = Some(factor)  ;
                state.scale( factor , conn ,qhandle )
            }
            Event::Name { name } => { *data.name.lock().unwrap() = Some(name.to_string()) ;
                state.name( name.to_string() , conn ,qhandle )
            }
            Event::Description { description } => { *data.description.lock().unwrap() = Some(description.to_string())  ;
                state.description(description.to_string() , conn ,qhandle )
            }
            _ => {}
        }
    }
}
