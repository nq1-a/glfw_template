use gl;

use glfw::{
    Action,
    Context,
    fail_on_errors,
    Key,
    WindowEvent,
    WindowMode,
};

fn main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Make window
    let (mut window, events) = glfw.create_window(
        600,
        400,
        "hello world",
        WindowMode::Windowed
    ).unwrap();

    window.make_current();
    window.set_key_polling(true);

    // Main loop
    while !window.should_close() {
        window.swap_buffers();

        // Events
        glfw.poll_events();

        for (_, ev) in glfw::flush_messages(&events) {
            match ev {
                WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    window.set_should_close(true)
                },

                _ => {}
            }
        }
    }
}
