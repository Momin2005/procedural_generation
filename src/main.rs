// This file is generated automatically. Do not edit it directly.
// See the Contributing section in README on how to make changes to it.
use tcod::colors::*;
use tcod::console::*;
use bracket_noise::prelude::*;

// actual size of the window
const SCREEN_WIDTH: i32 = 100;
const SCREEN_HEIGHT: i32 = 100;

// size of the map
const MAP_WIDTH: i32 = 100;
const MAP_HEIGHT: i32 = 100;

const PLAYER: usize = 0;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

const COLOR_DARK_GROUND: Color = Color { r: 255, g: 255, b: 255 };
const COLOR_DARK_WALL: Color = Color { r: 128, g: 128, b: 128 };

struct Tcod {
    root: Root,
    con: Offscreen,
}

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

/// A tile of the map and its properties
#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

/// This is a generic object: the player, a monster, an item, the stairs...
/// It's always represented by a character on screen.
#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    /// move by the given amount, if the destination is not blocked
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut Offscreen) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::Set);
    }
}

fn make_map() -> Map {
    // fill map with "unblocked" tiles
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    let mut noise = FastNoise::seeded(456);
    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(1);
    noise.set_fractal_gain(0.1);
    noise.set_fractal_lacunarity(1.0);
    noise.set_frequency(5.0);

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let point_value = noise.get_noise((x as f32) / 160.0, (y as f32) / 100.0);

            if point_value > 0.0 {
                map[x as usize][y as usize] = Tile::wall();
            }
        }
    }
    map
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    // go through all tiles, and set their background color
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    // draw all objects in the list
    for object in objects {
        object.draw(&mut tcod.con);
    }

    // blit the contents of "con" to the root console
    blit(
        &tcod.con,
        (objects[PLAYER].x - SCREEN_WIDTH/2, objects[PLAYER].y - SCREEN_HEIGHT/2),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
    println!("x: {}, y: {}", objects[PLAYER].x, objects[PLAYER].y)

}

fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true, // exit game

        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),

        _ => {}
    }

    false
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let mut tcod = Tcod { root, con };

    // create object representing the player
    let player = Object::new(MAP_WIDTH/ 2, MAP_HEIGHT / 2, '@', BLACK);

    // the list of objects with those two
    let mut objects = [player];

    let game = Game {
        // generate map (at this point it's not drawn to the screen)
        map: make_map(),
    };

    while !tcod.root.window_closed() {
        // clear the screen of the previous frame
        tcod.con.clear();
        tcod.root.clear();

        // render the screen
        render_all(&mut tcod, &game, &objects);

        tcod.root.flush();

        // handle keys and exit game if needed
        let player = &mut objects[PLAYER];
        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        }
    }
}