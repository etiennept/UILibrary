use wayland_client::protocol::wl_pointer::WlPointer;

struct Pointer{
    wl_pointer : WlPointer
}

impl Pointer{
    fn new (){

    }

}

impl Drop for Pointer{
    fn drop(&mut self) {
        self.wl_pointer.release()
    }
}
