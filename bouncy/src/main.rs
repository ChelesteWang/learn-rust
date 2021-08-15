use std::fmt::{Display, Formatter};

// 垂直
enum VertDir {
    Up,
    Down,
}

// 水平
enum HorizDir {
    Left,
    Right,
}

// 球封装 x y 位置 dir 方向
struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

// 视图容器
struct Frame {
    width: u32,
    height: u32,
}

// 游戏容器
struct Game {
    frame: Frame,
    ball: Ball,
}

// 实现 Game
impl Game {
    // 创建新对象
    fn new() -> Game {
        let frame = Frame {
            width: 60,
            height: 30,
        };
        let ball = Ball {
            x: 2,
            y: 4,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
        };
        Game {frame, ball}
    }
    // 运行
    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

// 实现 Ball
impl Ball {
    // 触底反转
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y == 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y == frame.height - 1 {
            self.vert_dir = VertDir::Up;
        }
    }
    // 移动
    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let top_bottom = |fmt: &mut Formatter| {
            write!(fmt, "+");
            for _ in 0..self.frame.width {
                write!(fmt, "-");
            }
            write!(fmt, "+\n")
        };

        top_bottom(fmt)?;

        for row in 0..self.frame.height {
            write!(fmt, "|");

            for column in 0..self.frame.width {
                let c = if row == self.ball.y && column == self.ball.x {
                    'o'
                } else {
                    ' '
                };
                write!(fmt, "{}", c);
            }

            write!(fmt, "|\n");
        }

        top_bottom(fmt)
    }
}

fn main () {
    // 构造
    let mut game = Game::new();
    let sleep_duration = std::time::Duration::from_millis(33);
    loop {
        println!("{}", game);
        game.step();
        std::thread::sleep(sleep_duration);
    }
}