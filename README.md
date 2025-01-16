# Surf-Board Virtual Keyboard

Based on the example application to implement a custom virtual keyboard in Slint.


## Structure

1. The virtual keyboard itself: This is implemented in `virtual_keyboard.slint` as a re-usable component.
   The application is responsible for placing it in the scene, typically as the last item in the root component.
2. Keyboard visibility: When a `TextInput` element receives the focus, either by the user clicking on it or programmatically
   via a call to `focus()`, it sets the global `TextInputInterface.text-input-focused` property to true. Similarly,
   when the focus is lost, this property is set to false again. Use this property to control visibility of the virtual keyboard.
3. Interaction: When the user clicks on a key in the virtual keyboard, the application needs to simulate a key event as if the user
   pressed the key on a real keyboard. The virtual keyboard invokes `VirtualKeyboardHandler`'s `key_pressed` callback. You need
   to set this callback to dispatch a key event to the `slint::Window`. Slint takes care of routing it to the currently focused
   `TextInput`. In Rust, call `slint::Window::dispatch_event(slint::platform::WindowEvent::KeyPressed{...})` to dispatch
   the event; in C++ call `slint::Window::dispatch_key_press_event(...)`. Subsequently, the you should dispatch a key
   release event using the same family of functions.


## Reference Links

https://www.slintpad.com/

https://docs.slint.dev/latest/docs/slint/reference/overview/

https://docs.slint.dev/latest/docs/slint/reference/elements/path/

https://github.com/slint-ui/slint/discussions/4563

https://github.com/slint-ui/slint/discussions/6910

