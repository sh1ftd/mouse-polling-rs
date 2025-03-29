use std::{
    sync::{Arc, Mutex},
    time::Instant,
};
use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

use crate::{app::App, config};

pub fn run_input_thread(app: Arc<Mutex<App>>) {
    println!("Starting Windows mouse polling thread");

    let mut last_pos = (0, 0);

    loop {
        unsafe {
            let mut point = POINT::default();
            if GetCursorPos(&mut point).is_ok() {
                let current_pos = (point.x, point.y);

                if current_pos != last_pos {
                    let now = Instant::now();

                    // Use a short lock to minimize contention
                    {
                        let mut app_guard = app.lock().unwrap();
                        app_guard.add_event(now, Some(current_pos));
                    }

                    last_pos = current_pos;
                }
            }
        }

        std::thread::sleep(config::POLLING_INTERVAL);
    }
}
