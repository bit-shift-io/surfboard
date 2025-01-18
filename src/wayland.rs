use slint::*;
use super::*;
use raw_window_handle::HasWindowHandle;
use wayland_client::{
    delegate_noop,
    globals::{registry_queue_init, GlobalListContents},
    protocol::{
        wl_buffer::WlBuffer,
        wl_compositor::WlCompositor,
        wl_output::WlOutput,
        wl_registry::{self, WlRegistry},
        wl_shm::{Format, WlShm},
        wl_shm_pool::WlShmPool,
        wl_surface::WlSurface,
    },
    Connection, Dispatch, QueueHandle, Proxy
};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{Layer, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{self, Anchor, KeyboardInteractivity, ZwlrLayerSurfaceV1},
};



/// The gist of it
/// get the connection
/// get the compositor
/// get the layer surface
/// get the layer shell?
/// https://github.com/Smithay/wayland-rs/issues/737
pub fn init(ui: &MainWindow) {
    /*
    let conn = wayland_client::Connection::connect_to_env().unwrap();

    // TODO: need to get access to the globals & event_queue, similar to below
    //let (globals, mut event_queue) = registry_queue_init::<Delegate>(&conn).unwrap();
    //let qh = event_queue.handle();

    // raw window handle for modifying the wayland surface
    match ui                                                                                  
        .window()                                                                             
        .window_handle()                                                                      
        .window_handle()                                                                      
        .unwrap()                                                                             
        .as_raw()                                                                             
    {                                                                                         
        raw_window_handle::RawWindowHandle::Wayland(wayland_window) => { 
            let nn_surface = wayland_window.surface;                         
            let wl_surface_obj_id: wayland_client::backend::ObjectId;                     
            unsafe {                                                                          
                wl_surface_obj_id = wayland_client::backend::ObjectId::from_ptr(              
                    WlSurface::interface(),                                       
                    nn_surface.as_ptr().cast(),                                          
                )                                                                             
                .unwrap();                                                                    
            }
            
            // get surface
            let wl_surface: WlSurface = WlSurface::from_id(&conn, wl_surface_obj_id.clone()).unwrap();

            // assign surface_layer role to wl_surface
            let wlr_layer_shell: ZwlrLayerShellV1 = ZwlrLayerShellV1::from_id(&conn, wl_surface_obj_id).unwrap();

            // TODO:i need to get access to the queue handle via a global?
            // let layer_surface = wlr_layer_shell.get_layer_surface(
            //     &wl_surface, 
            //     None, 
            //     Layer::Overlay, 
            //     "surfboard".to_string(),
            //     &qh, // queue handle
            //     () // user data
            // );
        }                                                                                     
        _ => {}                                                                               
    }
 */

}