#![windows_subsystem = "windows"]

use winvd;
use mki::{Keyboard};

fn desktop_next() {

    let desktop_count = winvd::get_desktop_count().unwrap();

    for (index, desktop) in winvd::get_desktops().unwrap().iter().enumerate() {
        if desktop == &winvd::get_current_desktop().unwrap() {
            if desktop_count > index as u32 + 1 {
                winvd::switch_desktop(index as u32 + 1).unwrap();
            }
        }
    }
}

fn desktop_prev() {

    for (index, desktop) in winvd::get_desktops().unwrap().iter().enumerate() {
        if desktop == &winvd::get_current_desktop().unwrap() {

            if index == 0 {
                return;
            }

            winvd::switch_desktop(index as u32 - 1).unwrap();
        }
    }
}

fn main() {

    mki::register_hotkey(&[Keyboard::LeftAlt, Keyboard::D], || {
        desktop_next();
    });

    mki::register_hotkey(&[Keyboard::LeftAlt, Keyboard::A], || {
        desktop_prev();
    });

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1000));
    }
}