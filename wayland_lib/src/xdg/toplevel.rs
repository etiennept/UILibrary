use std::sync::Mutex;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::xdg::shell::client::xdg_toplevel;
use wayland_protocols::xdg::shell::client::xdg_toplevel::{Event, XdgToplevel};
use crate::{handler, proxy};
proxy!( Toplevel , XdgToplevel ) ;

pub struct ToplevelData {



}
handler!{
    trait ToplevelHandler {
        fn  configure (width : i32  , height :i32 ,   state  : Vec<u8> , ) ;
        fn close () ;
        fn configure_bound (width : i32  , height :i32 ,) ;
        fn wm_capabilities (capabilities : Vec<u8>  , );
    }
}


impl<T> Dispatch< XdgToplevel , ToplevelData  , T  > for Toplevel
    where  T :Dispatch< XdgToplevel , ToplevelData > +ToplevelHandler {
    fn event(state: &mut T, proxy: &XdgToplevel, event: xdg_toplevel::Event, data: &ToplevelData, conn: &Connection, qhandle: &QueueHandle<T>) {
        match event {
            Event::Configure { width, height, states } => {
                state.configure(width , height , states ,  conn , qhandle  )
            }
            Event::Close => {  state.close( conn , qhandle ) }
            Event::ConfigureBounds { width, height } => {
                state.configure_bound(width  , height ,conn ,qhandle)
            }
            Event::WmCapabilities { capabilities } => {
                state.wm_capabilities( capabilities , conn ,qhandle)
            }
            _ => {}
        }
    }
}

#[macro_export]
macro_rules! delegate_xdg_toplevel   {
    ( $name:ident   ) => {
        wayland_client::delegate_dispatch!( $name : [ wayland_protocols::xdg::shell::client::xdg_toplevel::XdgToplevel : $crate::xdg::toplevel::ToplevelData  ]=>$crate::xdg::toplevel::Toplevel) ;
    };
}

impl Toplevel {
    fn set_parent (&self,  toplevel  : Option<Toplevel> ){
        //self.ptr.set_parent(   )
    }
    pub(crate) fn set_title (&self, title : String ){
        self.ptr.set_title( title)
    }
    fn set_app_id( &self , id : String ){
        self.ptr.set_app_id(id )
    }
    pub(crate) fn set_maximized(&self ){
        self.ptr.set_maximized()
    }
    fn unset_maximized(&self ){
        self.ptr.unset_maximized()
    }
    fn set_minimized (&self ){
        self.ptr.set_minimized( )
    }
    fn set_max_size(&self , width  :i32 , height : i32 ) {
        self.ptr.set_max_size(width , height )
    }
    fn set_min_size(&self , width :i32 , height :i32 ){
        self.ptr.set_min_size(width , height )
    }

    //fn set_geometry(&self , id :   )

}

impl Drop for Toplevel {
    fn drop(&mut self) {
        self.ptr.destroy()
    }
}