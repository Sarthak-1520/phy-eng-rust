use fltk::{app, draw, enums::{Color, Event , Key}, frame::Frame, prelude::*, window::Window};
use std::cell::RefCell;
use std::rc::Rc;

struct Ball {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    radius: f64,
}

impl Ball {
    fn new(x: f64, y: f64, dx: f64, dy: f64, radius: f64) -> Ball {
        Ball {
            x,
            y,
            dx,
            dy,
            radius,
        }
    }

    fn update(&mut self, w: f64, h: f64) {
        self.x += self.dx;
        self.y += self.dy;
        if self.x < 0.0 || self.x > w - self.radius {
            self.dx = -self.dx;
        }
        if self.y < 0.0 || self.y > h - self.radius {
            self.dy = -self.dy;
        }
    }

    fn draw(&self) {
        draw::set_draw_color(Color::Red);
        draw::draw_circle(self.x, self.y, self.radius);
    }
}

fn main() {
    let app = app::App::default();
    let wind = Window::new(100, 100, 400, 300, "FLTK Example");

    let ball = Rc::from(RefCell::from(Ball::new(200.0, 150.0, 0.1, 0.1, 10.0)));
    let wind_rc = Rc::from(RefCell::from(wind));

    // let dx_label = Rc::from(RefCell::from(Frame::new(10, 10, 380, 20, "")));
    // let dy_label = Rc::from(RefCell::from(Frame::new(10, 40, 380, 20, "")));

    {
        let ball = ball.clone();
        // let dx_label = dx_label.clone();
        // let dy_label = dy_label.clone();
        let wind_rc_clone = wind_rc.clone();
        app::add_idle3(move |_| {
            ball.borrow_mut().update(400.0, 300.0);
            wind_rc_clone.borrow_mut().redraw();
            // dx_label.borrow_mut().set_label(&format!("dx: {:.2}", ball.borrow().dx));
            // dy_label.borrow_mut().set_label(&format!("dy: {:.2}", ball.borrow().dy));
        });
    }

    wind_rc.borrow_mut().draw({
        let ball = ball.clone();
        move |_| {
            ball.borrow().draw();
        }
    });

    wind_rc.borrow_mut().handle({
        let ball = ball.clone();
        move |_, ev| match ev {
            Event::KeyDown => {
                let mut ball = ball.borrow_mut();
                match app::event_key() {
                    Key::Up => ball.dy = ball.dy.abs() + 0.1,
                    Key::Down => ball.dy = ball.dy.abs() - 0.1,
                    Key::Left => ball.dx = ball.dx.abs() - 0.1,
                    Key::Right => ball.dx = ball.dx.abs() + 0.1,
                    _ => (),
                }
                true
            }
            _ => false,
        }
    });

    wind_rc.borrow_mut().show();
    app.run().unwrap();
}