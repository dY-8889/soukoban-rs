use std::fmt::{Debug, Formatter};

use getch_rs::Key;

use ObjectType::{Box as B, Gool as G, None as N, Player as P, Wall as W};

const STAGE_LIST: [Field; 2] = [
    [
        [W, W, W, W, W, W, W, W, W, W, W, W, W, W, W],
        [W, P, N, N, N, N, N, N, N, N, N, N, N, G, W],
        [W, N, N, N, N, N, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, N, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, W, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, W, B, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, W, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, W, B, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, W, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, W, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, N, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, N, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, N, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, N, N, N, N, N, N, N, N, G, W],
        [W, W, W, W, W, W, W, W, W, W, W, W, W, W, W],
    ],
    [
        [W, W, W, W, W, W, W, W, W, W, W, W, W, W, W],
        [W, P, N, N, N, N, N, N, N, N, N, N, N, G, W],
        [W, N, N, N, N, N, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, W, W, W, W, W, N, N, N, N, W],
        [W, N, N, N, N, W, N, N, N, W, N, N, N, N, W],
        [W, N, N, N, N, W, B, N, N, W, N, N, N, N, W],
        [W, N, N, N, N, W, N, N, N, W, N, N, N, N, W],
        [W, N, N, N, N, W, B, N, N, W, N, N, N, N, W],
        [W, N, N, N, N, W, N, N, N, W, N, N, N, N, W],
        [W, N, N, N, N, W, N, N, N, W, N, N, N, N, W],
        [W, N, N, N, N, N, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, N, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, N, N, N, N, N, N, N, N, N, W],
        [W, N, N, N, N, N, N, N, N, N, N, N, N, G, W],
        [W, W, W, W, W, W, W, W, W, W, W, W, W, W, W],
    ],
];

const PLAYER_INITIAL_POSITION: Position = Position { x: 1, y: 1 };

type Field = [[ObjectType; 15]; 15];

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

struct Game {
    field: Field,
    position: Position,
    b_position: Position,
    move_direction: Direction,
    stage: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum ObjectType {
    Player,
    Box,
    Wall,
    Gool,
    None,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Debug for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y_iter in &self.field {
            for object in y_iter {
                write!(f, " ")?;
                match object {
                    ObjectType::Player => write!(f, "P")?,
                    ObjectType::Box => write!(f, "B")?,
                    ObjectType::Wall => write!(f, "W")?,
                    ObjectType::Gool => write!(f, "G")?,
                    ObjectType::None => write!(f, " ")?,
                }
            }
            writeln!(f)?
        }
        writeln!(f, "{:?}", self.position)?;
        write!(f, "{:?}  ", self.move_direction)
    }
}

impl Position {
    fn left(&mut self) {
        if let Some(num) = self.x.checked_sub(1) {
            self.x = num
        }
    }
    fn right(&mut self) {
        if let Some(num) = self.x.checked_add(1) {
            self.x = num
        }
    }
    fn down(&mut self) {
        if let Some(num) = self.y.checked_add(1) {
            self.y = num
        }
    }
    fn up(&mut self) {
        if let Some(num) = self.y.checked_sub(1) {
            self.y = num
        }
    }
}

impl Game {
    const fn new() -> Game {
        Game {
            field: STAGE_LIST[0],
            position: PLAYER_INITIAL_POSITION,
            b_position: PLAYER_INITIAL_POSITION,
            move_direction: Direction::Right,
            stage: 0,
        }
    }

    fn draw(&self) {
        print!("\x1b[H");

        for y_iter in self.field.iter() {
            for object in y_iter {
                match object {
                    ObjectType::Player => print!("🟩"),
                    ObjectType::Box => print!("⬜️"),
                    ObjectType::Wall => print!("🟫"),
                    ObjectType::Gool => print!("🔶"),
                    ObjectType::None => print!("  "),
                }
            }
            println!()
        }
        println!("{}", self.stage);
    }

    fn move_player(&mut self) {
        self.field[self.b_position.y][self.b_position.x] = ObjectType::None;

        let field = self.field;

        // プレイヤーの移動する位置にBoxがあるなら
        let player_x = self.position.x;
        let player_y = self.position.y;
        if field[player_y][player_x] == ObjectType::Box {
            let Position { x, y } = self.direction();
            // 移動する一つ先に壁or Boxがあるなら
            if field[y][x] == ObjectType::Wall || field[y][x] == ObjectType::Box {
                self.undo_position();
            } else {
                // ないなら
                self.field[y][x] = ObjectType::Box;
            }
        }
        self.field[self.position.y][self.position.x] = ObjectType::Player;
    }
    // プレイヤーの進んだ方向の位置を返す
    const fn direction(&self) -> Position {
        let Position { mut x, mut y } = self.position;
        match self.move_direction {
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
            Direction::Down => y += 1,
            Direction::Up => y -= 1,
        }
        Position { x, y }
    }
    // プレイヤーの位置を前の場所に戻す
    fn undo_position(&mut self) {
        self.position = self.b_position;
    }
    fn left(&mut self) {
        self.position.left();
        self.move_direction = Direction::Left;
    }
    fn right(&mut self) {
        self.position.right();
        self.move_direction = Direction::Right;
    }
    fn down(&mut self) {
        self.position.down();
        self.move_direction = Direction::Down;
    }
    fn up(&mut self) {
        self.position.up();
        self.move_direction = Direction::Up;
    }

    fn next_stage(&mut self) {
        self.stage += 1;
        if STAGE_LIST.len() - 1 < self.stage {
            self.stage = 0;
        }
        self.field = STAGE_LIST[self.stage];
    }
}

fn main() {
    print!("\x1b[2J");
    print!("\x1b[?25l");

    let mut game = Game::new();

    loop {
        game.move_player();
        game.draw();

        game.b_position = game.position;

        match getch_rs::Getch::new().getch() {
            Ok(Key::Char('j')) | Ok(Key::Left) => game.left(),
            Ok(Key::Char('l')) | Ok(Key::Right) => game.right(),
            Ok(Key::Char('i')) | Ok(Key::Up) => game.up(),
            Ok(Key::Char('k')) | Ok(Key::Down) => game.down(),
            Ok(Key::Char('n')) => game.next_stage(),
            Ok(Key::Char('q')) => break,
            _ => (),
        }

        let Position { x, y } = game.position;
        if game.field[y][x] == ObjectType::Wall {
            game.undo_position()
        }
    }

    print!("\x1b[?25h");
}
