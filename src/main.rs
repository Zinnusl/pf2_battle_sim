#![cfg_attr(target_family = "wasm", no_main)]
#![allow(dead_code)]
#[allow(unused_imports)]

use nannou::prelude::*;
use nannou::{
    app::App,
    wgpu::{DeviceDescriptor, Limits},
};
#[cfg(target_family = "wasm")]
use nannou::{
    app::{self},
    wgpu::Backends,
};
use wasm_bindgen_futures::JsFuture;

#[cfg(target_family = "wasm")]
use std::sync::RwLock;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

use std::sync::Arc;
use std::cell::RefCell;
use futures_lite::future::FutureExt;

pub mod task;
pub mod console;

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    thread_local!(static MODEL: RwLock<Option<Model>> = Default::default());
    let model = model();

    MODEL.with(|m| m.write().unwrap().replace(model));

    task::block_on(async { app::Builder::new_async(|app| {
            Box::new(async {
                create_window(app).await;
                MODEL.with(|m| m.write().unwrap().take().unwrap())
            })
        })
        .backends(Backends::PRIMARY | Backends::GL)
        .update(update)
        .run_async()
        .await;
    });

    Ok(())
}

#[cfg(not(target_family = "wasm"))]
pub async fn sleep(ms: u32) {
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub async fn sleep(delay: i32) {
        let mut cb = |resolve: js_sys::Function, reject: js_sys::Function| {
            web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, delay);};

    let p = js_sys::Promise::new(&mut cb);

    wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}

async fn create_window(app: &App) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    app.new_window()             
        .size(1024, 1024)
        .device_descriptor(device_desc)
        .title("Pathfinder 2e Battle Sim")
        .view(view)
        // .mouse_pressed(mouse_pressed)
        // .mouse_released(mouse_released)
        .event(event)
        .build_async()
        .await
        .unwrap();
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::MousePressed(MouseButton::Left) => {
            println!("Mouse pressed");
        }
        WindowEvent::MouseReleased(MouseButton::Left) => {
            println!("Mouse released");
        }
        _ => (),
    }
}

macro_rules! attacks {
    ($A:literal $B:literal $C:literal) => {
        (Attack::One($A), Attack::Two($B), Attack::Three($C))
    };
}

macro_rules! dice {
    ($dice_num:literal d4 $sign:tt $bonus:literal) => {
        Damage {
            0: Die::D4($dice_num),
            1: Bonus(0 $sign $bonus)
        }
    };
    ($dice_num:literal d6 $sign:tt $bonus:literal) => {
        Damage {
            0: Die::D6($dice_num),
            1: Bonus(0 $sign $bonus)
        }
    };
    ($dice_num:literal d8 $sign:tt $bonus:literal) => {
        Damage {
            0: Die::D8($dice_num),
            1: Bonus(0 $sign $bonus)
        }
    };
    ($dice_num:literal d10 $sign:tt $bonus:literal) => {
        Damage {
            0: Die::D10($dice_num),
            1: Bonus(0 $sign $bonus)
        }
    };
    ($dice_num:literal d12 $sign:tt $bonus:literal) => {
        Damage {
            0: Die::D12($dice_num),
            1: Bonus(0 $sign $bonus)
        }
    };
    ($dice_num:literal d20 $sign:tt $bonus:literal) => {
        Damage {
            0: Die::D20($dice_num),
            1: Bonus(0 $sign $bonus)
        }
    };
}

trait Roll {
    fn roll(&self) -> i32;
}

#[derive(Clone, Debug, PartialEq)]
struct Damage(Die, Bonus);

impl Roll for Damage {
    fn roll(&self) -> i32 {
        self.0.roll() + self.1.roll()
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Attack {
    One(i32),
    Two(i32),
    Three(i32),
}

type AttackInRound = i32;

impl Roll for Attack {
    fn roll(&self) -> i32 {
        match self {
            Attack::One(bonus) => dice!(1 d20 + 0).roll() + *bonus,
            Attack::Two(bonus) => dice!(1 d20 + 0).roll() + *bonus,
            Attack::Three(bonus) => dice!(1 d20 + 0).roll() + *bonus,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Stats {
    attacks: (Attack, Attack, Attack),
    damage: Damage,
    ac: i32,
}            

#[derive(Clone, Debug, PartialEq)]
struct Bonus(i32);

impl Roll for Bonus {
    fn roll(&self) -> i32 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Die {
    D4(i32),
    D6(i32),
    D8(i32),
    D10(i32),
    D12(i32),
    D20(i32),
}

impl Roll for Die {
    fn roll(&self) -> i32 {
        match self {
            Die::D4(n) => (0..(*n)).map(|_| rand::random::<u32>() % 4 + 1).sum::<u32>() as i32,
            Die::D6(n) => (0..(*n)).map(|_| rand::random::<u32>() % 6 + 1).sum::<u32>() as i32,
            Die::D8(n) => (0..(*n)).map(|_| rand::random::<u32>() % 8 + 1).sum::<u32>() as i32,
            Die::D10(n) => (0..(*n)).map(|_| rand::random::<u32>() % 10 + 1).sum::<u32>() as i32,
            Die::D12(n) => (0..(*n)).map(|_| rand::random::<u32>() % 12 + 1).sum::<u32>() as i32,
            Die::D20(n) => (0..(*n)).map(|_| rand::random::<u32>() % 20 + 1).sum::<u32>() as i32,
        }
    }
}

#[derive(Clone, Debug)]
struct Pos {
    x: f32,
    y: f32,
}
impl Pos {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
struct Agent {
    name: String,
    pos: Pos,
    stats: Stats,
    attacks_in_round: AttackInRound,
    hp: i32,
}

impl Agent {
    fn new(pos: Pos, stats: Stats) -> Self {
        Self { name: "".to_string(), pos, stats, attacks_in_round: 0, hp: 50 }
    }

    pub fn new_round(&mut self) {
        self.attacks_in_round = 0;
    }

    pub fn in_range(&self, other: &Agent) -> bool {
        let x = self.pos.x - other.pos.x;
        let y = self.pos.y - other.pos.y;
        let dist = (x * x + y * y).sqrt();
        dist < 100.0
    }

    pub fn move_towards(&mut self, other: &Agent) {
        let x = self.pos.x - other.pos.x;
        let y = self.pos.y - other.pos.y;
        let dist = (x * x + y * y).sqrt();
        let x = x / dist;
        let y = y / dist;
        self.pos.x -= x;
        self.pos.y -= y;
    }

    pub fn attack(&mut self, other: &mut Agent) {
        match self.attacks_in_round {
            0 => {
                let roll = self.stats.attacks.0.roll();
                if roll >= other.stats.ac {
                    let damage = self.stats.damage.roll();
                    other.hp -= damage;
                    console::console_log!("{} hit {} on first attack for {} damage", self.name, other.name, damage);
                }
                self.attacks_in_round += 1;
            }
            1 => {
                let roll = self.stats.attacks.1.roll();
                if roll >= other.stats.ac {
                    let damage = self.stats.damage.roll();
                    other.hp -= damage;
                    console::console_log!("{} hit {} on second attack for {} damage", self.name, other.name, damage);
                }
                self.attacks_in_round += 1;
            }
            _ => {
                let roll = self.stats.attacks.2.roll();
                if roll >= other.stats.ac {
                    let damage = self.stats.damage.roll();
                    other.hp -= damage;
                    console::console_log!("{} hit {} on third attack for {} damage", self.name, other.name, damage);
                }
                self.attacks_in_round += 1;
            }
        }
    }
}

struct Model {
    agents: Vec<Arc<RefCell<Agent>>>,
}

fn model() -> Model {
    let rect = Rect::from_w_h(1024.0, 1024.0);
    Model { 
        agents: vec![
            Arc::new(RefCell::new(Agent {
                name: "Bandit".to_string(),
                pos: Pos::new(rect.left() + 100.0, rect.top() - 100.0),
                stats: Stats {
                    attacks: attacks!(0 -5 -10),
                    damage: dice!(1 d6 + 1),
                    ac: 10,
                },
                hp: 50,
                attacks_in_round: 0,
            })),
            Arc::new(RefCell::new(Agent {
                name: "Monk".to_string(),
                pos: Pos::new(rect.right() - 100.0, rect.bottom() + 100.0),
                stats: Stats {
                    attacks: attacks!(0 -4 -8),
                    damage: dice!(1 d4 + 1),
                    ac: 14,
                },
                hp: 30,
                attacks_in_round: 0,
            })),
        ],
    }
}

fn update(_app: &App, m: &mut Model, _update: Update) {

    let a = m.agents[0].clone();
    let b = m.agents[1].clone();

    for _ in 0..=2 {
        if !a.borrow().in_range(&b.borrow()) {
            a.borrow_mut().move_towards(&b.borrow());
        } else {
            a.borrow_mut().attack(&mut b.borrow_mut());
        }

        // sleep for 2 seconds
        // async_std::task::sleep(std::time::Duration::from_secs(2)).await;
    }

    check_dead(&mut m.agents);

    for _ in 0..=2 {
        if !b.borrow().in_range(&a.borrow()) {
            b.borrow_mut().move_towards(&a.borrow());
        } else {
            b.borrow_mut().attack(&mut a.borrow_mut());
        }

        // sleep for 2 seconds
        // async_std::task::sleep(std::time::Duration::from_secs(2)).await;
    }

    check_dead(&mut m.agents);

    a.borrow_mut().new_round();
    b.borrow_mut().new_round();
}

fn check_dead(agents: &mut Vec<Arc<RefCell<Agent>>>) {
    let mut dead = vec![];
    for (i, agent) in agents.iter().enumerate() {
        if agent.borrow().hp <= 0 {
            dead.push(i);
        }
    }
    for i in dead.iter().rev() {
        agents.remove(*i);
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    // Draw agents
    for agent in &m.agents {
        draw.ellipse()
            .x_y(agent.borrow().pos.x, agent.borrow().pos.y)
            .w_h(50.0, 50.0)
            .color(BLACK);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    println!("Must be run as a web app! Use trunk to build.");
    println!("todo: enum_dispatch");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_attack_macro() {
        assert_eq!((Attack::One(7), Attack::Two(3), Attack::Three(-2)), attacks!(7 3 -2));
        assert_ne!((Attack::Three(7), Attack::Two(3), Attack::Three(-2)), attacks!(7 3 -2));
        assert_ne!((Attack::One(8), Attack::Two(3), Attack::Three(-2)), attacks!(7 3 -2));
    }

    #[test]
    fn test_dice_macro() {
        assert_eq!(Damage(Die::D6(1), Bonus(1)), dice!(1 d6 + 1));
        assert_eq!(Damage(Die::D12(5), Bonus(5)), dice!(5 d12 + 5));
        assert_eq!(Damage(Die::D20(66), Bonus(-1)), dice!(66 d20 - 1));
        assert_eq!(Damage(Die::D8(2), Bonus(-5)), dice!(2 d8 - 5));
    }

    #[test]
    fn test_dice() {
        for _ in 0..1000 {
            assert!((2..=7).contains(&dice!(1 d6 + 1).roll()));
        }
        for _ in 0..1000 {
            assert!((9..=53).contains(&dice!(4 d12 + 5).roll()));
        }
        for _ in 0..1000 {
            assert!((-44..=70).contains(&dice!(6 d20 - 50).roll()));
        }
    }
}
