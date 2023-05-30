use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::{Dispatch, QueueHandle};

pub struct Compositor{
    wl_compositor : WlCompositor
}
/*impl Compositor{
    fn create_surface<T> (&self , qh : QueueHandle<T  > ) -> WlSurface {
        self.wl_compositor.create_surface(qh  )
    }


} */

pub (crate ) enum  CompositorState{

}






