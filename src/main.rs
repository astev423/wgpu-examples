fn main() {
    env_logger::init();
    println!("Press the left/right arrow keys to see other shaders");
    graphics::app::run().unwrap();
}
