use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Atmosphere>()
        .register_type::<Tank>()
        .register_type::<ResourceOutput>()
        .register_type::<ResourceInput>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (provide, solve_network).chain());
}

fn setup(mut commands: Commands) {
    let fuel_tank = commands
        .spawn((
            Name::new("Tank"),
            Tank {
                capacity: 100.0,
                pressure: 1.0,
                maximum_pressure: 4.0,
                contents: Mixture {
                    contents: vec![(Substance::OXYGEN, 0.3), (Substance::HYDROGEN, 0.7)],
                },
            },
            ResourceOutput {
                max_flow_rate: 10.0,
                currently_available: 0.0,
            },
        ))
        .id();

    let generator = commands
        .spawn((
            Name::new("Generator"),
            Generator {
                input: Mixture {
                    contents: vec![(Substance::OXYGEN, 0.3), (Substance::HYDROGEN, 0.7)],
                },
            },
            ResourceInput { max_flow_rate: 5.0 },
        ))
        .id();

    commands.spawn((
        Name::new("Connection"),
        Connection {
            source: fuel_tank,
            target: generator,
        },
    ));
}

fn provide(time: Res<Time>, mut tanks: Query<(&mut Tank, &mut ResourceOutput)>) {
    for (mut tank, mut output) in &mut tanks {
        let removed = (tank.capacity - output.currently_available).min(output.max_flow_rate)
            * time.delta_secs();
        tank.capacity -= removed;
        output.currently_available += removed;
    }
}

fn solve_network(
    time: Res<Time>,
    mut outputs: Query<(Entity, &mut ResourceOutput)>,
    inputs: Query<(Entity, &ResourceInput)>,
    connections: Query<&Connection>,
) {
    for (input, resource_input) in &inputs {
        for (output, mut resource_output) in &mut outputs {
            let is_connected = connections
                .iter()
                .any(|connection| connection.target == input && connection.source == output);
            if !is_connected {
                info!("{input} is not connected");
                continue;
            }

            let available = resource_output.currently_available;
            let delivered = available.min(resource_input.max_flow_rate) * time.delta_secs();
            resource_output.currently_available -= delivered;
            if delivered > 0.0 {
                info!(
                    "delivered {:.2} kg of fuel from tank {:?} to generator {:?}",
                    delivered, output, input
                );
            }
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ResourceOutput {
    max_flow_rate: f32,
    currently_available: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ResourceInput {
    max_flow_rate: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Connection {
    source: Entity,
    target: Entity,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Generator {
    input: Mixture,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Pipe {
    flow_rate: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Pump {
    power_usage: f32,
    flow_rate: f32,
    input: Port,
    output: Port,
}

#[derive(Reflect)]
enum Port {
    Input,
    Output,
    Mixed,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Tank {
    /// An empty tank has the atmosphere in it
    contents: Mixture,
    /// Capacity in liters
    capacity: f32,
    /// current pressure
    pressure: f32,
    /// maximum pressure
    maximum_pressure: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Atmosphere {
    /// Volume of the atmosphere in m³
    volume: f32,
    /// current temperature
    temperature: Temperature,
    /// current pressure in bar
    pressure: f32,
    mixture: Mixture,
}

#[derive(Reflect)]
enum Phase {
    Gas,
    Liquid,
    Solid,
    Plasma,
}

#[derive(Reflect)]
struct Mixture {
    contents: Vec<(Substance, f32)>,
}

impl Default for Mixture {
    fn default() -> Self {
        Mixture {
            contents: vec![(Substance::OXYGEN, 0.21), (Substance::NITROGEN, 0.79)],
        }
    }
}

#[derive(Reflect)]
struct Substance {
    name: &'static str,
    density: f32,
    phase: Phase,
}

impl Substance {
    const OXYGEN: Substance = Substance {
        name: "Oxygen",
        density: 1.429,
        phase: Phase::Gas,
    };

    const NITROGEN: Substance = Substance {
        name: "Nitrogen",
        density: 1.2506,
        phase: Phase::Gas,
    };

    const HYDROGEN: Substance = Substance {
        name: "Hydrogen",
        density: 0.08988,
        phase: Phase::Gas,
    };
}

#[derive(Reflect)]
/// Temperature in Kelvin
pub struct Temperature(f32);

impl Temperature {
    pub const ABSOLUTE_ZERO: Temperature = Temperature(0.0);
    const KELVIN_TO_CELSIUS: f32 = 273.15;

    /// Convert temperature to celsius
    pub fn as_celsius(&self) -> f32 {
        self.0 + Self::KELVIN_TO_CELSIUS
    }

    pub fn from_celsius(value: f32) -> Self {
        Temperature(value - Self::KELVIN_TO_CELSIUS)
    }
}
