#![allow(unused_labels)]
#![feature(let_chains)]

use windows::{Platform, Window};
use common::geo::{Vector2, Rect2};
use graphics::{Bitmap, Pixel};

const WORLD_DIMENSIONS: Rect2 = Rect2::new(20, 20);
const TILE_COUNT: u32 = WORLD_DIMENSIONS.area();

type Frame = u128;

fn main() {
    let platform = Platform::init();
    let mut ui = UI::new(platform.create_window());

    let mut rect = Rectangle::new(
        Vector2::new(0,0),
        Rect2::new(300,100),
    );

    loop {
        let ui = &mut ui;
        use std::time::Instant;
        let time_start = Instant::now();

        let mut elements: Vec<&mut dyn Element> = vec![
            &mut rect
        ];

        ui.update(elements.as_mut_slice());

        'frame_timing: {
            use std::time::Duration;
            const FPS_TARGET: u32 = 60;
            const FPS_TARGET_MS: u32 = 1000 / FPS_TARGET;
            const FPS_DURATION: Duration = Duration::from_millis(FPS_TARGET_MS as u64);
            if time_start.elapsed().as_millis() > (FPS_TARGET_MS - 3) as u128 {
                while time_start.elapsed().as_millis() <= FPS_TARGET_MS as u128 {}
            } else {
                std::thread::sleep(FPS_DURATION - Duration::from_millis(3));
                while time_start.elapsed().as_millis() <= FPS_TARGET_MS as u128 {}
            }
        }

        ui.render(elements.as_mut_slice());

    }
}

struct Rectangle {
    pos: Vector2,
    size: Rect2,
    color: RGB,
    moving: MovingState,
}

impl Rectangle {
    const DEFAULT_COLOR: RGB = RGB::new(0,0,200);

    pub fn new(pos: Vector2, size: Rect2) -> Self {
        Self {
            pos,
            size,
            color: Self::DEFAULT_COLOR,
            moving: MovingState::Not
        }
    }

    pub fn point_is_within(&self, point: Vector2) -> bool {
        if point.x < self.pos.x || point.y < self.pos.y { return false; }
        point.x - self.pos.x < self.size.width && point.y - self.pos.y < self.size.height
    }
}

enum MovingState {
    Not,
    InProgress {
        start_offset: Vector2
    }
}

impl Element for Rectangle {
    fn update(&mut self, ui: &UI) {
        self.color = Self::DEFAULT_COLOR;

        let UI { mouse, .. } = ui;
        let window = &ui.window;
        'dragging: {
            // if the mouse is dragging...
            if let MouseEvent::Dragging = mouse
            // and the element is not already being moved...
            && let MovingState::Not = self.moving 
            // and the mouse is 'hovering over' the element...
            && self.point_is_within(window.mouse.pos) {
                // allow moving to start
                self.moving = MovingState::InProgress { start_offset: window.mouse.pos - self.pos };
                break 'dragging;
            }

            if let MovingState::InProgress { start_offset: offset } = self.moving {
                match mouse {
                    MouseEvent::Dragging => self.pos = window.mouse.pos - offset,
                    _ => self.moving = MovingState::Not,
                };
            }
        }
    }

    fn render(&self, ui: &UI, bitmap: &mut Bitmap) {
        bitmap.draw_rect(self.pos, self.size, self.color);

        if let MovingState::InProgress{..} = self.moving {
            const BORDER_THICKNESS: u32 = 3;
            const BORDER_COLOR: RGB = RGB::new(255,0,0);

            for mut x in 0..=BORDER_THICKNESS {
                x += self.pos.x;
                for mut y in 0..=self.size.height {
                    y += self.pos.y;
                    bitmap.draw_point(
                        (x, y),
                        BORDER_COLOR
                    )
                }
            }

            for mut x in (self.size.width - BORDER_THICKNESS)..=self.size.width {
                x += self.pos.x;
                for mut y in 0..=self.size.height {
                    y += self.pos.y;
                    bitmap.draw_point(
                        (x, y),
                        BORDER_COLOR
                    )
                }
            }

            for mut y in 0..=BORDER_THICKNESS {
                y += self.pos.y;
                for mut x in 0..=self.size.width {
                    x += self.pos.x;
                    bitmap.draw_point(
                        (x, y),
                        BORDER_COLOR
                    )
                }
            }

            for mut y in (self.size.height - BORDER_THICKNESS)..=self.size.height {
                y += self.pos.y;
                for mut x in 0..=self.size.width {
                    x += self.pos.x;
                    bitmap.draw_point(
                        (x, y),
                        BORDER_COLOR
                    )
                }
            }
        }
    }
}

struct UI<'p> {
    pub window: Window<'p>,
    pub frame_counter: Frame,
    pub mouse: MouseEvent,
}

impl<'p> UI<'p> {
    pub fn new(window: Window<'p>) -> Self {
        Self {
            window,
            frame_counter: 0,
            mouse: MouseEvent::Not
        }
    }
    
    pub fn update(&mut self, elements: &mut [&mut dyn Element]) {
        self.frame_counter += 1;

        self.window.process_messages();

        {
            use MouseEvent::*;
            match self.mouse {
                Not => {
                    if self.window.mouse.left || self.window.mouse.right {
                        self.mouse = Holding(self.frame_counter);
                        println!("-> Holding")
                    }
                },
                Click { .. } => {
                    self.mouse = Not;
                    println!("-> Not")
                }
                Holding(frame) => {
                    if self.window.mouse.left || self.window.mouse.right {
                        if frame + 5 <= self.frame_counter {
                            self.mouse = DragStarted;
                            println!("-> DragStarted")
                        }
                    } else {
                        self.mouse = Click {
                            left:  self.window.mouse.left,
                            right: self.window.mouse.right
                        };
                        println!("-> Click")
                    }
                },
                DragStarted => {
                    self.mouse = Dragging;
                    println!("-> Dragging")
                }
                Dragging => {
                    if !(self.window.mouse.left || self.window.mouse.right) {
                        self.mouse = DragEnded;
                        println!("-> DragEnded")
                    }
                    
                }
                DragEnded => {
                    self.mouse = Not;
                    println!("-> Not")
                },
            }
        }

        for element in elements {
            element.update(self);
        }
        
    }
    fn render(&mut self, elements: &mut [&mut dyn Element]) {
        let window = &self.window;
        let mut bitmap = Bitmap::new(window.size);
        for element in elements {
            element.render(self, &mut bitmap);
        }
        window.swap_buffers(&bitmap);
    }
}


trait Render {
    fn render(&self, bitmap: &mut Bitmap);
}

#[derive(Debug, Clone, Copy)]
struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<RGB> for Pixel {
    fn from(value: RGB) -> Self {
        Pixel::new(value.r, value.g, value.b)
    }
}

trait Element {
    fn update(&mut self, ui: &UI);
    fn render(&self, ui: &UI, bitmap: &mut Bitmap);
}

#[derive(Clone, Copy, Debug)]
enum MouseEvent {
    Not,
    Click {
        left: bool,
        right: bool
    },
    /// transitions to either DragStarted or Click
    Holding(Frame),
    /// single frame transition state for events
    DragStarted,
    Dragging,
    /// single frame transition state for events
    DragEnded
}