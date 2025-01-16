slint::include_modules!();



pub fn main() {
    let ui = MainWindow::new().unwrap();

    init_wayland(&ui);       

    virtual_keyboard::init(&ui);

    ui.run().unwrap();
}


mod virtual_keyboard {
    use super::*;
    use slint::*;

    pub fn init(app: &MainWindow) {
        let weak = app.as_weak();
        app.global::<VirtualKeyboardHandler>().on_key_pressed({
            move |key| {
                weak.unwrap()
                    .window()
                    .dispatch_event(slint::platform::WindowEvent::KeyPressed { text: key.clone() });
                weak.unwrap()
                    .window()
                    .dispatch_event(slint::platform::WindowEvent::KeyReleased { text: key });
            }
        });
    }
}


pub fn init_wayland(ui: &MainWindow) {
    /*
    use raw_window_handle::HasWindowHandle;
    use wayland_client::protocol::wl_surface;
    use wayland_client::Proxy;

    // TODO: make this work for all wayland compositors
    let conn = wayland_client::Connection::connect_to_env().unwrap();
    let display = conn.display();
    
    
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
            let mut wl_surface_obj_id: wayland_client::backend::ObjectId;                     
            unsafe {                                                                          
                wl_surface_obj_id = wayland_client::backend::ObjectId::from_ptr(              
                    wl_surface::WlSurface::interface(),                                       
                    nn_surface.as_ptr().cast(),                                          
                )                                                                             
                .unwrap();                                                                    
            }                                                                                 
            let wl_surface: wl_surface::WlSurface = 
                                    wl_surface::WlSurface::from_id(&conn, wl_surface_obj_id).unwrap()                                                                
                                                                                            
            //appdata.clone().lock().unwrap().surface = Some(wl_surface);                       
        }                                                                                     
        _ => {}                                                                               
    }
     */

}