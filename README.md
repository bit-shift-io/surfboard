# surfboard - virtual keyboard

A virtual keyboard for wayland and linux.

## installation

### KDE Plasma

Install `surfboard.desktop` to `/usr/share/applications`, with the path corresponding to the binary. Select it in the virtual keyboards menu in system settings, in order for it to see the virtual keyboard wayland APIs - input_method and fake_input

## FAQ

### Run an example other than main.rs

Drop the rs file in the bin folder and run: `cargo run --bin example`


## TODO
* launch apps + window mode
https://github.com/waycrate/exwlshelleventloop/tree/master/iced_examples

* split keyboard panes 
https://github.com/iced-rs/iced/blob/master/examples/pane_grid/src/main.rs

* keyboard character -> evdev? 
https://discourse.iced.rs/t/how-to-use-keyboard-character/424

* styles, events, canvas
https://github.com/fogarecious/iced_tutorial/blob/main/README.md