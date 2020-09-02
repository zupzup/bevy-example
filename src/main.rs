use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion},
    prelude::*,
    render::pass::ClearColor,
};

struct Unit;
struct Name(String);

struct Position {
    x: f32,
    y: f32,
}

enum Collider {
    Solid,
}

#[derive(Default)]
struct MouseEventsState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

#[derive(Default)]
struct MouseState {
    x: f32,
    y: f32,
}

struct LogTimer(Timer);

fn main() {
    App::build()
        .add_default_plugins()
        .init_resource::<MouseEventsState>()
        .init_resource::<MouseState>()
        .add_resource(ClearColor(Color::rgb(0.25, 0.0, 0.0)))
        .add_plugin(BasicPlugin)
        .run();
}

fn print_name_and_position_system(
    time: Res<Time>,
    mut timer: ResMut<LogTimer>,
    mut query: Query<(&Unit, &Name, &Position)>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (_unit, name, position) in &mut query.iter() {
            println!("name: {}, position: {} {}", name.0, position.x, position.y);
        }
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let minka_texture = asset_server
        .load("assets/textures/minka.png")
        .expect("can load minka");

    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());

    commands
        .spawn((
            Unit,
            Name("Minka".to_string()),
            Position { x: 10.0, y: 10.0 },
        ))
        .with(Collider::Solid)
        .spawn(SpriteComponents {
            material: materials.add(minka_texture.into()),
            translation: Translation(Vec3::new(50.0, 50.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(100.0, 73.0),
            },
            ..Default::default()
        })
        .spawn((Unit, Name("Nacho".to_string()), Position { x: 0.0, y: 0.0 }))
        .spawn((Unit, Name("Chip".to_string()), Position { x: 5.0, y: 5.0 }));

    commands.spawn(TextComponents {
        text: Text {
            font: asset_server
                .load("assets/fonts/Verdana.ttf")
                .expect("font can be loaded"),
            value: "Welcome to Knödeltown".to_string(),
            style: TextStyle {
                color: Color::rgb(1.0, 1.0, 1.0),
                font_size: 40.0,
            },
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(10.0),
                left: Val::Px(250.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    // walls
    let wall_material = materials.add(Color::rgb(0.0, 0.0, 0.0).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(800.0, 800.0);
    commands
        // left
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(-bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Solid)
        // right
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(wall_thickness, bounds.y() + wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Solid)
        // bottom
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, -bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Solid)
        // top
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + wall_thickness, wall_thickness),
            },
            ..Default::default()
        })
        .with(Collider::Solid);
}

pub struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(LogTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(setup.system())
            .add_system(mouse_input_system.system())
            .add_system(print_name_and_position_system.system());
    }
}

fn mouse_input_system(
    mut event_state: ResMut<MouseEventsState>,
    mut state: ResMut<MouseState>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_cursor_events: Res<Events<CursorMoved>>,
) {
    for event in event_state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        println!(
            "mouse button event {:?}, position: x: {}, y: {}",
            event, state.x, state.y
        );
    }

    // for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
    //     println!("{:?}", event);
    // }

    for event in event_state
        .cursor_moved_event_reader
        .iter(&mouse_cursor_events)
    {
        // println!("{:?}", event);
        state.x = event.position.x();
        state.y = event.position.y();
    }
}
