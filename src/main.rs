use bracket_lib::prelude::*;
// #[derive(Debug)]
// constant
const SCREEN_WIDTH:i32 = 80;
const SCREEN_HEIGHT:i32 = 50;
const FRAME_DURATION:f32 = 75.0;
#[derive(Debug)]
enum GameMode {
    Menu,
    Playing,
    End
}
struct State {
    player:Player,
    frame_time: f32,
    obstacle:Obstacle,
    mode:GameMode,
} 
struct Player {
    x:i32, // game level = world space position
    y:i32, // player position = screen space
    velocity: f32
}
struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}
impl State {
    fn new()-> Self {
        State {
            player: Player::new(5,25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu 
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) { 
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon"); 
        ctx.print_centered(8, "(P) Play Game"); 
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart() ,
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {

        //set the background color
        ctx.cls_bg(NAVYBLUE);
        //set the time frame 
        self.frame_time+=ctx.frame_time_ms;
        //reset the time frame to zero
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        //flap the brid 
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        self.obstacle.render(ctx, self.player.x);
       if self.player.x  > self.obstacle.x {
            self.obstacle = Obstacle::new(self.player.x+SCREEN_WIDTH, 0);
        }
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;    
        }
    }
    fn restart(&mut self){
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
    }
    fn dead(&mut self, ctx:&mut BTerm){
       ctx.cls();
       ctx.print_centered(5,"You Dead!");
       ctx.print_centered(8, "(P) Play Again");
       ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key{
            match key {
               VirtualKeyCode::P => self.play(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }    
        }

    }

    
}
impl Player {
    fn new(x:i32, y:i32) -> Self {
       Player { x: x ,y: y, velocity: 0.0 } 
    }
    fn render(&mut self, ctx:&mut BTerm){
       ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@')); 
    }
    fn gravity_and_move(&mut self){
        if self.velocity < 2.0 {
            self.velocity +=0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }
    fn flap(&mut self){
        self.velocity = -2.0;
    }
}
impl Obstacle{
    fn new(x: i32, score: i32)-> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
           x,
           gap_y: random.range(10,40),
           size: i32::max(2, 20-score)
        }
           
    }
    fn render(&mut self, ctx:&mut BTerm, x_axis: i32){
        let screen_x = self.x - x_axis;
        let half_size = self.size / 2;

        //Draw the top half onstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'),);

        }

        //Draw the bottom half
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

    }
    
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx)
        } 
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
            .with_title("Flabby Brid")
            .build()?;
    main_loop(context, State::new())
}
