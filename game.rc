extern mod sdl2;
mod engine;

fn main() {
    print("helloWorld");
    
    let mut engine = engine::Engine::new();
    print(fmt!("%d",engine.textureCache.load(~"images/rust-logo.png") as int));

    engine.run();
}
