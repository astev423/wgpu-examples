fn main() {
    env_logger::init();
    // Press the left/right arrow keys to see prev/next shaders
    /*
    draw_wave()
    draw_water()
    draw_fire()
    draw_magma()
    draw_plasma()
    draw_super_nova()
     */
    graphics::app::run().unwrap();
}
