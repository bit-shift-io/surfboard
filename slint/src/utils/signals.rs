use tokio::sync::{
    mpsc,
    Mutex
};
use once_cell::sync::Lazy;
use std::sync::Arc;


pub static MOUSE_POSITION_CHANNEL: Lazy<(Arc<Mutex<mpsc::Sender<[usize; 2]>>>, Arc<Mutex<mpsc::Receiver<[usize; 2]>>>)> = Lazy::new(|| {
    let (tx, rx) = mpsc::channel::<[usize; 2]>(32);
    (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
});


pub async fn get_mouse_position_tx() -> mpsc::Sender<[usize; 2]> {
    let tx = MOUSE_POSITION_CHANNEL.0.lock().await;
    tx.clone()
}

pub async fn get_mouse_position_rx() -> tokio::sync::MutexGuard<'static, mpsc::Receiver<[usize; 2]>> {
    MOUSE_POSITION_CHANNEL.1.lock().await
}
