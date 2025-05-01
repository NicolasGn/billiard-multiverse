use bevy::prelude::*;

use crate::billiard::{Ball, Billiard, InitialConditions, Real};
use crate::visualization::VisualizationPlugin;

#[derive(Resource)]
pub struct Simulation {
    initial_conditions: InitialConditions,
    billiard_1: Billiard,
    billiard_2: Billiard,
    is_running: bool,
}

impl Simulation {
    pub fn init(initial_conditions: InitialConditions) -> Self {
        let billiard_1 = Billiard::new(&initial_conditions);
        let billiard_2 = Billiard::new(&initial_conditions);

        Self {
            initial_conditions,
            billiard_1,
            billiard_2,
            is_running: false,
        }
    }

    #[inline(always)]
    pub fn size(&self) -> Real {
        self.initial_conditions.size()
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.initial_conditions.count()
    }

    #[inline]
    pub fn balls(&self, billiard_number: u8) -> &Vec<Ball> {
        match billiard_number {
            1 => self.billiard_1.balls(),
            2 => self.billiard_2.balls(),
            _ => panic!("invalid billiard number"),
        }
    }

    pub fn toggle_pause(&mut self) {
        self.is_running = !self.is_running;
    }

    pub fn run(self, visualize: bool) {
        let mut app: App = App::new();
        let size = self.size();

        app.insert_resource(self);

        app.add_plugins((
            MinimalPlugins,
            bevy::app::PanicHandlerPlugin,
            bevy::app::TerminalCtrlCHandlerPlugin,
        ));

        if visualize {
            app.add_plugins(VisualizationPlugin::init(size as f32));
        }

        app.add_systems(Update, (update_billiard_1, update_billiard_2));

        app.run();
    }
}

fn update_billiard_1(mut simulation: ResMut<Simulation>) {
    if simulation.is_running {
        simulation.billiard_1.update();
    }
}

fn update_billiard_2(mut simulation: ResMut<Simulation>) {
    if simulation.is_running {
        simulation.billiard_2.update();
    }
}
