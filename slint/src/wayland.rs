use slint::*;
use super::*;
use raw_window_handle::HasWindowHandle;
use wayland_client::{
    backend::Backend, 
    delegate_noop, 
    globals::{
        registry_queue_init, 
        GlobalListContents
    }, 
    protocol::{
        wl_buffer::WlBuffer,
        wl_compositor::WlCompositor,
        wl_output::WlOutput,
        wl_registry::{self, WlRegistry},
        wl_shm::{Format, WlShm},
        wl_shm_pool::WlShmPool,
        wl_surface::WlSurface,
    }, 
    Connection, 
    Dispatch, 
    Proxy, 
    QueueHandle
};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{
        self,
        Layer, 
        ZwlrLayerShellV1
    },
    zwlr_layer_surface_v1::{
        self, 
        Anchor, 
        KeyboardInteractivity, 
        ZwlrLayerSurfaceV1
    },
};




/// The gist of it
/// get the connection
/// get the compositor
/// get the layer surface
/// get the layer shell?
/// https://github.com/Smithay/wayland-rs/issues/737
/// 
/// //https://github.com/obhq/obliteration/blob/main/gui/src/ui/backend/wayland.rs
/// https://github.com/obhq/obliteration/blob/main/gui/src/ui/backend/window.rs
/// 
/// https://github.com/obhq/obliteration/blob/main/gui/src/ui/backend/wayland.rs#L29
pub fn init(ui: &MainWindow) {
    //let display = ui.window().window_handle().

    let conn = wayland_client::Connection::connect_to_env().unwrap();
    info!("wayland connection: {:#?}", conn);
    //let display = conn.display();

    // TODO: need to get access to the globals & event_queue, similar to below
    //let (globals, mut event_queue) = registry_queue_init::<Delegate>(&conn).unwrap();
    //let qh = event_queue.handle();

    // this requires var to be set to skia, the qt backend does not work
    let handle_binding = ui.window().window_handle();                               
    let handle = handle_binding.window_handle();
    if handle.is_err() {
        log::warn!("Failed to get Slint window handle! {:?}", handle.unwrap_err());
        return;
    };

    let raw_window_handle = handle.unwrap().as_raw();

    // raw window handle for modifying the wayland surface
    match raw_window_handle {                                                                                         
        raw_window_handle::RawWindowHandle::Wayland(wayland_window) => {
            info!("window handle: {:#?}", wayland_window);

            let nn_surface = wayland_window.surface;                         
            let wl_surface_obj_id: wayland_client::backend::ObjectId;                     
            unsafe {                                                                          
                wl_surface_obj_id = wayland_client::backend::ObjectId::from_ptr(              
                    WlSurface::interface(),                                       
                    nn_surface.as_ptr().cast(),                                          
                )                                                                             
                .unwrap();                                                                    
            }
            
            // // get surface
            let wl_surface: WlSurface = WlSurface::from_id(&conn, wl_surface_obj_id.clone()).unwrap();
            info!("wl_surface: {:#?}", wl_surface);

            let display_ptr = wl_surface.backend().upgrade().unwrap().display_ptr();
            let display_id = wl_surface.backend().upgrade().unwrap().display_id();

            // unsafe {
            //     // Get wayland connection.
            //     let backend = Backend::from_foreign_display(display_ptr.cast());
            //     let conn = Connection::from_backend(backend);
            //     info!("wayland connection: {:#?}", conn);
            // }

            // Get global objects
            // need ti implement own state?
            let (globals, mut queue) = registry_queue_init::<Delegate>(&conn).unwrap();
            let qh = queue.handle();

            let layer_shell: ZwlrLayerShellV1 = globals.bind(&qh, 1..=1, ()).unwrap();

            let layer_surface = layer_shell.get_layer_surface(
                &wl_surface, 
                None, 
                Layer::Overlay, 
                "surfboard".to_string(),
                &qh, // queue handle
                () // user data
            );

            layer_surface.set_anchor(Anchor::Bottom);
            layer_surface.set_size(800, 300);

            //layer_shell.write_request(conn, req)
            //queue.roundtrip(data)



        }                                                                                     
        _ => {}                                                                               
    }

}

/// wayland state
struct Delegate;
impl Dispatch<WlRegistry, GlobalListContents> for Delegate {
    fn event(
        _: &mut Self,
        _: &WlRegistry,
        _event: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, ()> for Delegate {
    fn event(
        _: &mut Self,
        layer_surface: &ZwlrLayerSurfaceV1,
        event: <ZwlrLayerSurfaceV1 as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        info!("test layer surface");
        if let zwlr_layer_surface_v1::Event::Configure { serial, .. } = event {
            layer_surface.ack_configure(serial);
        }
    }
}

impl Dispatch<ZwlrLayerShellV1, ()> for Delegate {
    fn event(
        state: &mut Self,
        proxy: &ZwlrLayerShellV1,
        event: <ZwlrLayerShellV1 as Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        info!("test layer shell");
    }
}

delegate_noop!(Delegate: ignore WlOutput);
delegate_noop!(Delegate: ignore WlShm);
delegate_noop!(Delegate: ignore WlShmPool);
delegate_noop!(Delegate: ignore WlBuffer);
delegate_noop!(Delegate: ignore WlCompositor);
delegate_noop!(Delegate: ignore WlSurface);
//delegate_noop!(Delegate: ignore ZwlrLayerShellV1);