
trait GameMode {
    fn new() -> Self;
    fn update(&self,dt:int);
    fn draw(&self);
}

pub struct SpaceMode {
    score:int
}

//impl GameMode  for SpaceMode {
// Either I'm stupid or it's this bug: https://github.com/mozilla/rust/issues/4100
impl SpaceMode {
    pub fn new() -> SpaceMode {
        SpaceMode {score:0}
    }
    pub fn update(&self, dt:int) {
        
    }
    pub fn draw(&self) {
        println("you are in space");
    }
}

