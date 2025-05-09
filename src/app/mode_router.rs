use crate::app::mode_manifest;

pub fn dispatch(choice: usize) {
    if choice == 0 {
        println!("Bye 👋");
        std::process::exit(0);
    }
    if let Some(mode) = mode_manifest::list().into_iter().find(|m| m.id == choice) {
        (mode.entry)();
    } else {
        println!("無效選項");
    }
}
