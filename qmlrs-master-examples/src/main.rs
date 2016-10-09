#[macro_use]
extern crate qmlrs;
mod math;

fn main() {
    let mut engine = qmlrs::Engine::new();

    engine.set_property("factorial123", math::Factorial);
    engine.load_local_file("src/main_ui.qml");

    engine.exec();
}
