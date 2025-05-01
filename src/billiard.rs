use rand::prelude::*;

use nalgebra::Vector2;

pub type Real = f64;
type Vector = Vector2<Real>;

/// Minimal time before the next collision (avoid redetecting last collision)
const TOI_EPSILON: Real = 2.0e-15;

#[derive(Clone)]
pub struct Ball {
    pub n: usize,
    pub position: Vector,
    pub velocity: Vector,
    pub radius: Real,
}

impl Ball {
    #[inline(always)]
    const fn new(n: usize, position: Vector, velocity: Vector, radius: Real) -> Self {
        Ball {
            n,
            position,
            velocity,
            radius: radius as Real,
        }
    }

    #[inline]
    fn clone_with_precision(&self, precision: Real) -> Self {
        let mut rng = rand::rng();

        let delta = Vector::new(rng.random_range(-1.0..=1.0), rng.random_range(-1.0..=1.0))
            .normalize()
            * precision;

        Self {
            n: self.n,
            position: self.position + delta,
            radius: self.radius,
            velocity: self.velocity,
        }
    }
}

enum WallOrientation {
    Horizontal,
    Vertical,
}

enum Collision {
    BallToBall {
        n1: usize,
        n2: usize,
    },
    BallToWall {
        n: usize,
        orientation: WallOrientation,
    },
}

pub struct InitialConditions {
    size: Real,
    count: usize,
    balls: Vec<Ball>,
    epsilon: Real,
}

impl InitialConditions {
    pub fn new(size: u32, count: usize, radius: u32, epsilon: Real) -> Self {
        let mut rng = rand::rng();
        let mut balls = Vec::new();

        let size_r = size as Real;
        let radius_r = radius as Real;

        let row_size_r = (count as Real).sqrt().ceil();
        let row_size = row_size_r as u32;

        let step = size_r / (row_size_r + 1.0);
        let mut x: Real = step;
        let mut y: Real = step;
        let mut c = 1;

        for n in 0..count {
            balls.push(Ball::new(
                n,
                Vector::new(x, y),
                Vector::new(rng.random_range(-1.0..=1.0), rng.random_range(-1.0..=1.0)).normalize(),
                radius_r,
            ));

            x += step;

            if c == row_size {
                c = 1;
                x = step;
                y += step;
            } else {
                c += 1;
            }
        }

        InitialConditions {
            size: size_r,
            count,
            balls,
            epsilon,
        }
    }

    #[inline(always)]
    pub fn size(&self) -> Real {
        self.size
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.count
    }

    #[inline(always)]
    pub fn epsilon(&self) -> Real {
        self.epsilon
    }

    fn instanciate_balls(&self) -> Vec<Ball> {
        self.balls
            .iter()
            .map(|ball| ball.clone_with_precision(self.epsilon))
            .collect()
    }
}

pub struct Billiard {
    time: Real,
    size: Real,
    balls: Vec<Ball>,
    toi: Real,
    collisions: Vec<Collision>,
}

impl Billiard {
    pub fn new(initial_conditions: &InitialConditions) -> Self {
        Self {
            size: initial_conditions.size,
            time: 0.0,
            balls: initial_conditions.instanciate_balls(),
            toi: 0.0,
            collisions: Vec::new(),
        }
    }

    #[inline(always)]
    pub fn balls(&self) -> &Vec<Ball> {
        &self.balls
    }

    #[inline(always)]
    pub fn time(&self) -> Real {
        self.time
    }

    pub fn update(&mut self) {
        self.compute_collisions();

        for ball in self.balls.iter_mut() {
            ball.position += ball.velocity * self.toi;
        }

        for collision in self.collisions.iter() {
            match collision {
                Collision::BallToWall { n, orientation } => {
                    let ball = &mut self.balls[*n];

                    match orientation {
                        WallOrientation::Horizontal => {
                            ball.velocity.y = -ball.velocity.y;
                        }
                        WallOrientation::Vertical => {
                            ball.velocity.x = -ball.velocity.x;
                        }
                    };
                }
                Collision::BallToBall { n1, n2 } => {
                    let p1 = &self.balls[*n1];
                    let p2 = &self.balls[*n2];

                    let pos12 = p1.position - p2.position;
                    let pos21 = p2.position - p1.position;

                    let vel12 = p1.velocity - p2.velocity;
                    let vel21 = p2.velocity - p1.velocity;

                    let next_vel1 =
                        p1.velocity - (vel12.dot(&pos12) / pos12.norm_squared()) * pos12;

                    let next_vel2 =
                        p2.velocity - (vel21.dot(&pos21) / pos21.norm_squared()) * pos21;

                    self.balls[*n1].velocity = next_vel1;
                    self.balls[*n2].velocity = next_vel2;
                }
            };

            self.time += self.toi;
        }
    }

    #[inline]
    fn compute_collisions(&mut self) {
        let mut min_toi = Real::MAX;
        self.collisions.clear();

        for p1 in self.balls.iter() {
            // Check collisions with walls
            let min = p1.radius;
            let max = self.size - p1.radius;

            let toi_min_x = (min - p1.position.x) / p1.velocity.x;
            let toi_min_y = (min - p1.position.y) / p1.velocity.y;
            let toi_max_x = (max - p1.position.x) / p1.velocity.x;
            let toi_max_y = (max - p1.position.y) / p1.velocity.y;

            if toi_min_x > TOI_EPSILON && toi_min_x <= min_toi {
                let collision = Collision::BallToWall {
                    n: p1.n,
                    orientation: WallOrientation::Vertical,
                };

                if toi_min_x < min_toi {
                    self.collisions.clear();
                }

                self.collisions.push(collision);
                min_toi = toi_min_x;
            } else if toi_max_x > TOI_EPSILON && toi_max_x <= min_toi {
                let collision = Collision::BallToWall {
                    n: p1.n,
                    orientation: WallOrientation::Vertical,
                };

                if toi_max_x < min_toi {
                    self.collisions.clear();
                }

                self.collisions.push(collision);
                min_toi = toi_max_x;
            }

            if toi_min_y > TOI_EPSILON && toi_min_y <= min_toi {
                let collision = Collision::BallToWall {
                    n: p1.n,
                    orientation: WallOrientation::Horizontal,
                };

                if toi_min_y < min_toi {
                    self.collisions.clear();
                }

                self.collisions.push(collision);
                min_toi = toi_min_y;
            } else if toi_max_y > TOI_EPSILON && toi_max_y <= min_toi {
                let collision = Collision::BallToWall {
                    n: p1.n,
                    orientation: WallOrientation::Horizontal,
                };

                if toi_max_y < min_toi {
                    self.collisions.clear();
                }

                self.collisions.push(collision);
                min_toi = toi_max_y;
            }

            // Check collisions with other balls
            for n2 in p1.n + 1..self.balls.len() {
                let p2 = &self.balls[n2];

                let pos12 = p1.position - p2.position;
                let vel12 = p1.velocity - p2.velocity;

                // We get the time of impact by solving a simple quadratic equation obtained from:
                // |p1.position - p2.position| = p1.radius + p2.radius
                let a = vel12.dot(&vel12);
                let b = 2.0 * pos12.dot(&vel12);
                let c = pos12.dot(&pos12) - (p1.radius + p2.radius).powi(2);

                let discriminant = b.powi(2) - 4.0 * a * c;

                if discriminant >= 0.0 {
                    let discriminant_sqrt = discriminant.sqrt();

                    let t1 = 0.5 * (-b - discriminant_sqrt) / a;
                    let t2 = 0.5 * (-b + discriminant_sqrt) / a;

                    // The highest solution is always to discard as it represents the theoritical
                    // time where balls would stop overlapping if the collision was not handled.
                    let toi = Real::min(t1, t2);

                    // Time of impact can be in the past, dicard it if it's the case
                    if toi > TOI_EPSILON && toi <= min_toi {
                        if toi < min_toi {
                            self.collisions.clear();
                        }

                        self.collisions
                            .push(Collision::BallToBall { n1: p1.n, n2: p2.n });

                        min_toi = toi;
                    }
                }
            }
        }

        self.toi = min_toi;
    }
}
