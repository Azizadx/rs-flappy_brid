use bracket_lib::prelude::*;
// #[derive(Debug)]
struct State {
} 

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello World!")
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
            .with_title("Flabby Brid")
            .build()?;
    main_loop(context, State{})
}
