use crate::event::state::{PEvent, PEventCallback, set_event_state, get_event_state};

pub mod window;
pub mod event_loop;

pub extern "C" fn p_init (setup: PEventCallback, draw: PEventCallback) {
    set_event_state! {
        setup = Some(setup);
        draw = Some(draw);
    }
}

pub extern "C" fn p_on (event: PEvent, callback: PEventCallback) {
    set_event_state! {
        events.insert(event, callback);
    }
}

pub extern "C" fn p_run () {
    let setup = get_event_state().setup.expect("No setup function specified. Call the p_init() function to set a setup function.");
    setup();
    pollster::block_on(event_loop::start_event_loop());
    
}