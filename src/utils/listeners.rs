use super::super::{Event, Program};

pub fn check_for_listener(event: Event, program: &Program, param: String) {
    if program.listeners.contains_key(&event) {
        let callbacks = program.listeners.get(&event).unwrap();
        for cb in callbacks {
            cb(program, param.clone())
        }
        std::process::exit(1)
    }
}
