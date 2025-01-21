# surfboard - virtual keyboard

A virtual keyboard for wayland and linux.

## installation

### KDE Plasma

Install `surfboard.desktop` to `/usr/share/applications`, with the path corresponding to the binary. Select it in the virtual keyboards menu in system settings, in order for it to see the virtual keyboard wayland APIs - input_method and fake_input

## FAQ

### Run an example other than main.rs

Drop the rs file in the bin folder and run: `cargo run --bin example`