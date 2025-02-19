
/// This does nothing if not linux
#[cfg(not(target_os = "linux"))]
macro_rules! to_layershell_message {
    ($item:item) => {
        $item
    };
}

/// Add extra layershell messages
#[cfg(target_os = "linux")]
macro_rules! to_layershell_message {
    ($item:item) => {
        #[to_layer_message]
        $item
    };
}