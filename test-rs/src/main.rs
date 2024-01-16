use processing::{core::{*, window::createWindow}, event::mouse::{mouseX, mouseY}};

pub extern "C" fn setup () {
    createWindow(800.0, 800.0);
}

pub extern "C" fn draw () {
}

pub extern "C" fn callback () {
    println!("({:?}, {:?})", mouseX(), mouseY());

}

fn main () {
    p_init(setup, draw);
    p_on(processing::event::state::PEvent::PMouseMoved, callback);
    p_run();
}
