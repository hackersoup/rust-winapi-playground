extern crate rand;
use std::{
    thread,
    time,
    ptr,
};
use rand::Rng;

use bindings::windows::win32::{display_devices::POINT, ipc::CreateNamedPipeA, menus_and_resources::{
        GetCursorPos,
        SetCursorPos,
    }};

/// Uses `println!` when in debug mode, otherwise does nothing
macro_rules! println_debug {
    ($($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                println!($($e),+)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($e),+)
            }
        }
    };
}


fn main() -> windows::Result<()> {

    // Fire and forget infinite-running threads
    thread::spawn(shake_mouse);

    // Wait for a newline to continue
    std::io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}

fn named_pipe_server(pipe_name: &str) {
    /*
    TODO:
        - Change arguments to a more idiomatic way than just the raw string.
        - Some way to encrypt strings at compile time.
        - More customization of the pipe settings.
    */

    unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createnamedpipea
        let pipe = CreateNamedPipeA(
            pipe_name.as_ptr() as *const i8,
            3, // PIPE_ACCESS_DUPLEX (couldn't find a definition in the crate)
            0, // Byte-type pipe mode
            255, // PIPE_UNLIMITED_INSTANCES
            4096,
            4069,
            200,
            ptr::null_mut());
    }
}

fn shake_mouse() {
    // Generates the pixel range to shake the mouse with
    let mut rng = rand::thread_rng();

    // Time in milliseconds to sleep before the next "shake" event
    let sleep_time_ms = 20;

    // Max amount of pixels to move the mouse in a direction, per shake event
    let delta = 20;

    // Tracks the current position of the mouse for modification
    let mut current_mouse_pos: POINT = POINT { x: 0, y: 0 };

    loop {
        unsafe { GetCursorPos(&mut current_mouse_pos); }
        let current_x = current_mouse_pos.x;
        let current_y = current_mouse_pos.y;

        let new_x = rng.gen_range((current_x - delta)..(current_x + delta));
        let new_y = rng.gen_range((current_y - delta)..(current_y + delta));

        unsafe { SetCursorPos(new_x, new_y); }


        println_debug!("{:?}", &current_mouse_pos);

        thread::sleep(time::Duration::from_millis(sleep_time_ms));
    }
}