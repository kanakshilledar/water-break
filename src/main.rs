slint::include_modules!();
use notify_rust::Notification;
use std::{time::{Duration, Instant}, thread};

fn reminder(mut interval: u64) {
    interval = interval * 60;
    let mut last_run = Instant::now();
    loop {
        if last_run.elapsed() >= Duration::from_secs(interval) {
            Notification::new()
                .summary("Drink Water")
                .body("Water has zero calories, making it best for your diet")
                .icon("/home/klug/Documents/projects/water-break/assets/bottles.svg")
                .show().unwrap();
            last_run = Instant::now();
        }

        thread::sleep(Duration::from_secs(interval));
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_start_reminding({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let intval: i32 = ui.get_interval();
            Notification::new()
                .summary("Water Break Configurator")
                .body(&format!("Remind every {} minutes", intval))
                .icon("/home/klug/Documents/projects/water-break/assets/bottles.svg")
                .show().unwrap();

            reminder(intval as u64);
        }
    });

    ui.run()
}
