extern crate piston_window;
extern crate find_folder;
extern crate rand;

use piston_window::*;
use rand::Rng;

struct Folk {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub active: bool,
    ltr: bool,
    speed: f64,
    blue: [f32; 4],
    color: [f32; 4] // red
}

impl Enemy {
    pub fn new(param_ltr:bool, param_speed:f64) -> Enemy {
        Enemy {
            x: Enemy::decide_x(param_ltr),
            y: 630.0,
            w: 40.0,
            h: 40.0,
            active: true,
            ltr: param_ltr,
            speed: param_speed,
            blue: [0.0, 0.0, 1.0, 1.0],
            color: [1.0, 0.0, 0.0, 1.0]
        }
    }
    fn decide_x(ltr:bool) -> f64 {
        if ltr {
            0.0
        } else {
            500.0
        }
    }
    pub fn update(&mut self) {
        if self.active {
            if self.ltr {
                self.x += 1.0 * self.speed;
                if self.x > 1000.0 {
                    self.active = false;
                }
            } else {
                self.x -= 1.0 * self.speed;
                if self.x < 0.0 - self.w {
                    self.active = false;
                }
            }
        }
    }
    pub fn deactivate(&mut self) {
        self.active = false;
        self.color = self.blue;
    }
}

struct Gun {
    pub active: bool,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64
}

impl GuN {
    pub fn new(param_x: f64, param_y: f64) -> Gun  {
        Gun {
            active: true,
            x: param_x,
            y: param_y,
            w: 40.0,
            h: 40.0
        }
    }
    pub fn update(&mut self) {
        if self.active {
            self.y += 1.0;

            if self.y > 650.0 { // window height_ish
                self.active = false;
            }
        }
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

struct Shots {
    pub total: i32,
    pub gone: i32,
    pub left: i32
}

impl Shots {
    pub fn new() -> Shots {
        Shots {
            total: 12,
            gone: 0,
            left: 12
        }
    }
    pub fn fire(&mut self) {
        if self.left > 0 {
            self.gone += 1;
            self.left -= 1;
        }
    }
}

struct Snyper {
    pub rows: i32,
    pub columns: i32,
    pub x: i32,
    pub y: i32,
    factor: f64,
    pub shots: Shots,
    pub gun: Vec<Gun>
}

impl Snyper {
    pub fn new() -> Snyper {
        Snyper   {
            rows: 4,
            columns: 3,
            x: 0,
            y: 0,
            factor: 110.0,
            shots: Shots::new(),
            gun: vec![]
        }
    }
    fn calc_coord(&mut self, pos: f64) -> f64 {
        self.factor + (pos * self.factor)
    }
    pub fn shoot(&mut self)  {
        let x = self.x as f64;
        let y = self.y as f64;
        let x = self.calc_coord(x);
        let y = self.calc_coord(y);
        println!("{}", self.shots.left);
        if self.shots.left > 0 {
            self.shots.fire(); 
            self.gun.push(Gun::new(x.y)); 
        }
    }
    pub fn update(&mut self) {
        for a in self.gun.iter_mut()  {
            a.update();
        }
    }
    pub fn moving(&mut self, x: i32, y: i32) {
        self.x += x;
        if self.x > self.columns -1 {
            self.x = 0;
        } else if self.x < 0 {
            self.x = self.columns -1;
        }
        self.y += y;
        if self.y > self.rows -1 {
            self.y = 0;
        } else if self.y < 0 {
            self.y = self.rows -1;
        }
    }
}


struct Game {
    pub scene: usize,
    pub Enemys:  Vec<Enemy> ,
    pub Player: Player, 
    last_player:  f64,
    player_interval: f64, 
    global_time: f64,
    pub score: i32
}

impl Game {
    pub fn new(param_scene: usize) -> Game {
        Game {
            scene: param_scene,
            Enemys:  vec![],
            snyper: Snyper::new(), 
            last_folk: 1.0,
            folk_interval: 2.0,
            global_time: 0.0,
            score: 0
        }
    }
    pub fn set_scene(&mut self, scene: usize) {
        self.scene = scene;
    }

    fn spawn_enemy(&mut self)  {
        let mut rng = rand::thread_rng();
        let speed = rng.gen::<f64>() + 1.0;
        let ltr = rng.gen();

        self.enemys.push(Enemy::new(ltr, speed));        
    }

    fn count_up_score(&mut self, points: i32) {
        self.score += points;
    }

    pub fn update(&mut self, dt: f64) {
            if self.snyper.shots.left == 0 {       
            if !self.snyper.guns.get(11).unwrap().active {            
                self.set_scene(3);
            }
        }
         self.snyper.update();        

        self.global_time += dt;
        self.last_enemy += dt; 

       if self.last_enemy > self.enemy_internal {  
         self.last_enemy = 0.0; 

          self.spawn_enemy();  
        }

        
          for f in self.enemys.iter_mut() {
            f.update();
        }
    }

    pub fn check_collision(&mut self) {
        let mut points: i32 = 0;
        for f in self.enemys.iter_mut() { 
            if f.active {
             for a in self.snyper.gun.iter_mut() { 
                    if a.active {

                        if a.x < f.x + f.w &&
                            a.x + a.w > f.x &&
                            a.y < f.y + f.h &&
                            a.y + a.h > f.y {
                                f.deactivate();
                                a.deactivate();
                                points = 20;
                            }

                    }
                }
            }
        }
        self.count_up_score(points);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
         "SnypeYou" , [500, 700])     
        .opengl(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    let assets = find_folder::Search::ParentsThenKids(3,3)
        .for_folder("assets").unwrap();

    let ref font = assets.join("Amatic-Bold.ttf");
    let mut glyphs = Glyphs::new(font, window.factory.clone()).unwrap();

    let house_start = Texture::from_path(
        &mut window.factory,
        assets.join("tokyoG.jpg"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let house = Texture::from_path(
        &mut window.factory,
        assets.join("tokyoG1.jpg"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let gun = Texture::from_path(
        &mut window.factory,
        &assets.join("gun.jpg"), 
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let gun_gone= Texture::from_path( 
        &mut window.factory,
        &(assets.join("gun_gone.jpg")), 
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let black = [0.0, 0.0, 0.0, 1.0];
    let white = [1.0, 1.0, 1.0, 1.0];

    let mut game = Game::new(1);

    while let Some(e) = window.next() {

        match e {
            Input::Release(Button::Keyboard(key)) => {
                match game.scene {
                    1 => {
                        if key == Key::M {
                            game.set_scene(2);
                        }
                    }
                    2 => {
                        match key {
                            Key::W => {
                                game.snyper.moving(0, -1)
                            }
                            Key::S => {
                                game.snyper.moving(0, 1);
                            }
                            Key::A => {
                               game.snyper.moving(-1,0); 
                            }
                            Key::D => {
                                game.snyper.moving(1,0); 
                            }
                            Key::F  => {
                                game.snyper.throw() ;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            Input::Update(args) => {
                if game.scene == 2 {
                    game.update(args.dt);
                    game.check_collision();
                }
            }

            Input::Render(_) => {
                match game.scene {
                    1 => {
                        window.draw_2d(&e, |c, g| {
                            clear(white, g);
                            image(&house_start, c.transform.scale(0.5, 0.5), g);

                            text::Text::new_color(black, 50)
                                .draw(
                                    &"SnypeYou" ,
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(100.0, 100.0),
                                    g);

                            text::Text::new_color(black, 30)
                                .draw(
                                    &"<w><a><s><d> to move",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(120.0, 300.0),
                                    g);
                            text::Text::new_color(black, 30)
                                .draw(
                                    &"<m> to throw",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(120.0, 350.0),
                                    g);

                            text::Text::new_color(black, 30)
                                .draw(
                                    &"Press <m> to start",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(150.0, 500.0),
                                    g);
                        });
                    }
                    2 => {
                        window.draw_2d(&e, |c, g| {
                            clear(white, g);

                            image(&house, c.transform.scale(0.5, 0.5), g);

                            text::Text::new_color(black, 30)
                                .draw(
                                    &format!("Score: {}", game.score),
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(320.0, 40.0),
                                    g);

                                for i in 0..game.snyper.shots.total {                    
                                if i as i32 >= game.snyper.shots.left {
                                    image(&gun_gone , c.transform.scale(0.5, 0.5).trans((50.0 + (i * 50) as f64), 20.0), g);
                                } else {
                                    image(&gun , c.transform.scale(0.5, 0.5).trans((50.0 + (i * 50) as f64), 20.0), g);
                                }
                            }

                            for f in game.enemy.iter() { 
                                if f.active {
                                    let enemy_square  = rectangle::square(0.0, 0.0, f.w);
                                    rectangle(f.color, folk_square, c.transform.trans(
                                        f.x, f.y), g);
                                }
                            }

                            for a in game.snyper.guns.iter() { 
                                if a.active {
                                    image(&gun , c.transform.trans(
                                        a.x, a.y), g);
                                }
                            }

                            for cv in 0..game.snyper.rows  {
                                for rv in 0..game.snyper.columns { 
                                    if rv == game.snyper.x 
                                        && cv == game.snyper.y  {
                                            let square = rectangle::square(0.0, 0.0, 50.0);
                                            rectangle(black, square, c.transform.trans(
                                                game.snyper.calc_coord(rv as f64),
                                                game.snyper.calc_coord(cv as f64)), g);
                                        }
                                }
                            }
                        });
                    }
                    3 => {
                        window.draw_2d(&e, |c, g| {
                            clear(white, g);
                            image(&house_start, c.transform.scale(0.5, 0.5), g);

                            text::Text::new_color(black, 40)
                                .draw(
                                    &"You're out of gun bulletS",

                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(100.0, 100.0),
                                    g);

                            text::Text::new_color(black, 40)
                                .draw(
                                    &format!("Your score is {}", game.score),
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(100.0, 200.0),
                                    g);

                            text::Text::new_color(black, 30)
                                .draw(
                                    &"Press <esc> to close",
                                    &mut glyphs,
                                    &c.draw_state,
                                    c.transform.trans(150.0, 500.0),
                                    g);
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
