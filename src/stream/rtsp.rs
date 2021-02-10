mod video;

pub fn run() {
    println!("Hello from rtsp!");
    video::pipeline::run();
}
