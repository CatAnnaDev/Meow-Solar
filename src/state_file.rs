use std::f32::consts::PI;
use std::time::Instant;
use iced::{Color, mouse, Point, Rectangle, Renderer, Size, Theme, Vector, window};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas;
use iced::widget::canvas::{gradient, Path, Stroke, stroke, Text};
use iced::widget::text::LineHeight;
use rand::Rng;

#[derive(Debug)]
pub struct Planet{
   pub period: f32,
   pub orbit: f32,
   pub radius: f32,
   pub name: String,
   pub lune: bool,
   pub moon_speed: f32,
   pub color: Color,
   pub line_color : Color
}

#[derive(Debug)]
pub struct State {
    pub space_cache: canvas::Cache,
    pub system_cache: canvas::Cache,
    pub start: Instant,
    pub now: Instant,
    pub stars: Vec<(Point, f32)>,
    pub speed_multi: f32,
    pub mercury: Planet,
    pub venus: Planet,
    pub earth: Planet,
    pub mars: Planet,
    pub jupiter: Planet,
    pub saturn: Planet,
    pub uranus: Planet,
    pub neptune: Planet,
}

impl State {
    const SUN_RADIUS: f32 = 60.0;

    pub fn new() -> State {
        let mut rng = rand::thread_rng();
        let now = Instant::now();
        let (width, height) = window::Settings::default().size;

        State {
            space_cache: Default::default(),
            system_cache: Default::default(),
            start: now,
            now,
            stars: Self::generate_stars(width *2, height *2),
            speed_multi: 5.0,
            mercury: Planet {
                period: 87.97,
                orbit: 100.0,
                radius: 9.0,
                name: "mercury".to_string(),
                lune: false,
                moon_speed: rng.gen_range(1.0..=15.0 ),
                color: Color::from_rgb(0.66, 0.66, 0.66),
                line_color: Color::from_rgba(0.66, 0.66, 0.66, 0.1)
            },
            venus: Planet {
                period: 224.0,
                orbit: 140.0,
                radius: 12.0,
                name: "venus".to_string(),
                lune: false,
                moon_speed: rng.gen_range(1.0..=15.0 ),
                color: Color::from_rgb(1.0, 1.0, 0.0),
                line_color: Color::from_rgba(1.0, 1.0, 0.0, 0.1)
            },
            earth: Planet {
                period: 365.0,
                orbit: 180.0,
                radius: 12.0,
                name: "earth".to_string(),
                lune: true,
                moon_speed: rng.gen_range(1.0..=15.0 ),
                color: Color::from_rgb(0.0, 0.0, 1.0),
                line_color: Color::from_rgba(0.0, 0.0, 1.0, 0.1)
            },
            mars: Planet {
                period: 1.88 * 365.26,
                orbit: 220.0,
                radius: 11.0,
                name: "mars".to_string(),
                lune: true,
                moon_speed: rng.gen_range(1.0..=15.0 ),
                color: Color::from_rgb(0.91, 0.55, 0.48),
                line_color: Color::from_rgba(0.91, 0.55, 0.48, 0.1)
            },
            jupiter: Planet {
                period: 11.86 * 365.26,
                orbit: 270.0,
                radius: 18.0,
                name: "jupiter".to_string(),
                lune: true,
                moon_speed: rng.gen_range(1.0..=15.0 ),
                color: Color::from_rgb(0.55, 0.27, 0.07),
                line_color: Color::from_rgba(0.55, 0.27, 0.07, 0.1)
            },
            saturn: Planet {
                period: 29.46 * 365.26,
                orbit: 3300.0,
                radius: 18.0,
                name: "saturn".to_string(),
                lune: true,
                moon_speed: rng.gen_range(1.0..=15.0 ),
                color: Color::from_rgb(1.0, 0.84, 0.0),
                line_color: Color::from_rgba(1.0, 0.84, 0.0, 0.1)
            },
            uranus: Planet {
                period: 84.01 * 365.26,
                orbit: 390.0,
                radius: 16.0,
                name: "uranus".to_string(),
                lune: true,
                moon_speed: rng.gen_range(1.0..=15.0 ),
                color: Color::from_rgb(0.47, 0.87, 0.76),
                line_color: Color::from_rgba(0.47, 0.87, 0.76, 0.1)
            },
            neptune: Planet {
                period: 164.79 * 365.26,
                orbit: 450.0,
                radius: 16.0,
                name: "neptune".to_string(),
                lune: true,
                moon_speed: rng.gen_range(1.0..=15.0 ),
                color: Color::from_rgb(0.0, 0.41, 0.78),
                line_color: Color::from_rgba(0.0, 0.41, 0.78, 0.1)
            },
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.now = now;
        self.system_cache.clear();
    }

    fn generate_stars(width: u32, height: u32) -> Vec<(Point, f32)> {
        let mut rng = rand::thread_rng();
        let divide = 2.0f32;
        (0..5000)
            .map(|_| {
                (
                    Point::new(
                        rng.gen_range((-(width as f32) / divide)..(width as f32 / divide), ),
                        rng.gen_range((-(height as f32) / divide)..(height as f32 / divide), ),
                    ),
                    rng.gen_range(0.5..2.0),
                )
            })
            .collect()
    }
}

impl<Message> canvas::Program<Message> for State {
    type State = ();
    fn draw(&self, _state: &Self::State, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor, ) -> Vec<canvas::Geometry> {
        let background =
            self.space_cache.draw(renderer, bounds.size(), |frame| {
                let stars = Path::new(|path| {
                    for (p, size) in &self.stars {
                        path.rectangle(*p, Size::new(*size, *size));
                    }
                });
                frame.translate(frame.center() - Point::ORIGIN);
                frame.fill(&stars, Color::WHITE);
            });

        let system = self.system_cache.draw(renderer, bounds.size(), |frame| {
            let center = frame.center();
            let sun = Path::circle(center, Self::SUN_RADIUS);
            frame.fill(&sun, Color::from_rgb8(0xF9, 0xD7, 0x1C));

            let elapsed = self.now - self.start;

            let rotation_mercury = (2.0 * PI / self.mercury.period) * elapsed.as_secs_f32() * self.speed_multi;
            draw_planet(frame, center, rotation_mercury, &self.mercury, rotation_mercury); // Mercury
            draw_planet_line(frame, &Path::circle(center, self.mercury.orbit), self.mercury.line_color); // Mercury

            let rotation_venus = (2.0 * PI / self.venus.period) * elapsed.as_secs_f32() * self.speed_multi;
            draw_planet(frame, center, rotation_venus, &self.venus, rotation_venus); // Venus
            draw_planet_line(frame, &Path::circle(center, self.venus.orbit), self.venus.line_color); // Venus

            let rotation_earth = (2.0 * PI / self.earth.period) * elapsed.as_secs_f32() * self.speed_multi;
            draw_planet(frame, center, rotation_earth, &self.earth, rotation_earth); // Terre
            draw_planet_line(frame, &Path::circle(center, self.earth.orbit), self.earth.line_color); // Terre

            let rotation_mars = (2.0 * PI / self.mars.period) * elapsed.as_secs_f32() * self.speed_multi;
            draw_planet(frame, center, rotation_mars, &self.mars, rotation_earth); // Mars
            draw_planet_line(frame, &Path::circle(center, self.mars.orbit), self.mars.line_color); // Mars

            let rotation_jupiter = (2.0 * PI / self.jupiter.period) * elapsed.as_secs_f32() * self.speed_multi;
            draw_planet(frame, center, rotation_jupiter, &self.jupiter, rotation_earth); // Jupiter
            draw_planet_line(frame, &Path::circle(center, self.jupiter.orbit), self.jupiter.line_color); // Jupiter

            let rotation_saturn = (2.0 * PI / self.saturn.period) * elapsed.as_secs_f32() * self.speed_multi;
            draw_planet(frame, center, rotation_saturn, &self.saturn, rotation_earth); // Saturn
            draw_planet_line(frame, &Path::circle(center, self.saturn.orbit), self.saturn.line_color); // Saturn

            let rotation_uranus = (2.0 * PI / self.uranus.period) * elapsed.as_secs_f32() * self.speed_multi;
            draw_planet(frame, center, rotation_uranus, &self.uranus, rotation_earth); // Uranus
            draw_planet_line(frame, &Path::circle(center, self.uranus.orbit), self.uranus.line_color); // Uranus

            let rotation_neptune = (2.0 * PI / self.neptune.period) * elapsed.as_secs_f32() * self.speed_multi;
            draw_planet(frame, center, rotation_neptune, &self.neptune, rotation_earth); // Neptune
            draw_planet_line(frame, &Path::circle(center, self.neptune.orbit), self.neptune.line_color); // Neptune

            let mut text = Text::from(format!("Time is {} days / sec\n{}", self.speed_multi, time_converter(elapsed.as_secs_f32() * self.speed_multi, ) ), );
            text.size = 30.0;
            text.horizontal_alignment = Horizontal::Center;
            text.position = Point::new(150f32, 10f32);
            text.color = Color::from_rgba(1.0, 0.0, 1.0, 0.5);
            frame.fill_text(text);
        });

        vec![background, system]
    }
}

fn rotation_calculator(period: f32, elapsed: f32, speed: f32) -> f32{
    (2.0 * PI / period) * elapsed * speed
}

fn time_converter(seconds: f32) -> String {
    let jours = seconds.floor() as u32;
    let months = (jours as f32 / 30.417).floor() as u32;
    let years = (months as f32 / 12.0).floor() as u32;
    format!("{} jours\n{} mois\n{} années", jours, months, years)
}

fn draw_planet_line(frame: &mut canvas::Frame, planet_line: &Path, color: Color) {
    frame.stroke(
        planet_line,
        Stroke {
            style: stroke::Style::Solid(color),
            width: 1.0,
            line_dash: canvas::LineDash {
                offset: 0,
                segments: &[3.0, 6.0],
            },
            ..Stroke::default()
        },
    );
}

fn draw_planet(frame: &mut canvas::Frame, center: Point, rotation: f32, planet: &Planet, moon_rotation: f32) {
    frame.with_save(|frame| {
        frame.translate(Vector::new(center.x, center.y));
        frame.rotate(rotation);
        frame.translate(Vector::new(planet.orbit, 0.0));
        let planet_draw = Path::circle(Point::ORIGIN, planet.radius);

        let mut text = Text::from(format!("{}", planet.name));
        text.position =  Point::ORIGIN;
        text.horizontal_alignment = Horizontal::Center;
        text.vertical_alignment = Vertical::Bottom;
        text.line_height = LineHeight::Relative(3.0);
        text.color = planet.color;

        if planet.lune {
            frame.with_save(|frame| {
                frame.rotate(moon_rotation * planet.moon_speed);
                frame.translate(Vector::new(0.0, 28.0));
                let moon = Path::circle(Point::ORIGIN, 4.0);
                frame.fill(&moon, Color::WHITE);
            });
        }

        let earth_fill = gradient::Linear::new(Point::new(-planet.radius, 0.0), Point::new(planet.radius, 0.0), )
            .add_stop(0.4, planet.color)
            .add_stop(0.7, Color::from_rgb(0.0, 0.20, 0.20));

        frame.fill_text(text);
        frame.fill(&planet_draw, earth_fill);

    });
}