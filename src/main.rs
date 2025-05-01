mod billiard;
mod simulation;
mod visualization;

use billiard::InitialConditions;
use simulation::Simulation;

fn main() {
    let size = 1024;
    let count = 64;
    let radius = 32;
    let epsilon = 2.0e-10;

    println!("=== Initial conditions ===");
    println!("Billiard size: {}", size);
    println!("Balls count: {}", count);
    println!("Balls radius: {}", radius);
    println!("Epsilon: {:.64}", epsilon);

    let initial_conditions = InitialConditions::new(size, count, radius, epsilon);

    let simulation = Simulation::init(initial_conditions);

    simulation.run(true);
}
