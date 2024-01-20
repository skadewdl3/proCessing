use processing::{core::{*, window::createWindow}, event::mouse::{mouseX, mouseY}};

pub extern "C" fn setup () {
    createWindow(800.0, 800.0);
}

pub extern "C" fn draw () {
}

pub extern "C" fn callback () {
    // println!("({:?}, {:?})", mouseX(), mouseY());
}


pub extern "C" fn callback2 () {
    println!("Clicked");
}

fn main () {
    p_init(setup, draw);
    p_on(processing::event::state::PEvent::PMouseMoved, callback);
    p_on(processing::event::state::PEvent::PMousePressed, callback2);
    p_run();
}
