#[feature(managed_boxes)];
#[desc = "Test game I write to learn rust"];
#[license = "GPLv2"];

extern mod nphysics;
extern mod rsfml;
mod engine;

fn main() {
    print("helloWorld");
    
    let mut engine = engine::Engine::new();
    print(format!("{:?}\n",engine.textureCache.load(~"images/rust-logo.png")));

    engine.run();
}
