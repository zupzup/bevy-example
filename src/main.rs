use bevy::{
    input::keyboard::ElementState, input::mouse::MouseButtonInput, prelude::*,
    render::pass::ClearColor,
};

struct Unit;
struct Name(String);
struct Selected(bool);
struct MouseBox;

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
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

#[derive(Default)]
struct MouseState {
    pos: Vec2,
    pressed_at: Option<Vec2>,
    released_at: Option<Vec2>,
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
        .spawn(SpriteComponents {
            material: materials.add(minka_texture.into()),
            translation: Translation(Vec3::new(350.0, 350.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(100.0, 73.0),
            },
            ..Default::default()
        })
        .with(Unit)
        .with(Name("Minka".to_string()))
        .with(Position { x: 10.0, y: 10.0 })
        .with(Collider::Solid)
        .with(Selected(false))
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
            .add_system(mouse_box_system.system())
            .add_system(print_name_and_position_system.system());
    }
}

fn mouse_box_system(
    mut cmd: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    mouse_state: Res<MouseState>,
    mut q: Query<(Entity, &MouseBox)>,
) {
    let window = windows.get_primary();
    let win_height = window.expect("there is a window").height;
    let win_width = window.expect("there is a window").width;
    // println!("height: {}, width: {}", win_height, win_width);

    // TODO: pass in win_height and width
    // TODO: don't spawn/despawn, but update entities
    // TODO: pass in material, don't always add new one
    if mouse_state.pressed_at.is_some() {
        for (e, _mb) in &mut q.iter() {
            cmd.despawn(e);
        }

        let mouse_box_material = materials.add(Color::rgb(0.2, 0.8, 0.2).into());
        let mouse_box_thickness = 1.0;
        let current = mouse_state.pos;
        let pressed_pos = mouse_state.pressed_at.expect("is there");

        let start_pt_line: f32 =
            pressed_pos.x() + ((current.x() - pressed_pos.x()) / 2.0) - win_width as f32 / 2.0;
        // top
        cmd.spawn(SpriteComponents {
            material: mouse_box_material,
            translation: Translation(Vec3::new(
                start_pt_line,
                pressed_pos.y() - (win_height as f32 / 2.0),
                0.0,
            )),
            sprite: Sprite {
                size: Vec2::new(current.x() - pressed_pos.x(), mouse_box_thickness),
            },
            ..Default::default()
        })
        .with(MouseBox);
        // bottom
        cmd.spawn(SpriteComponents {
            material: mouse_box_material,
            translation: Translation(Vec3::new(
                start_pt_line,
                current.y() - (win_height as f32 / 2.0),
                0.0,
            )),
            sprite: Sprite {
                size: Vec2::new(current.x() - pressed_pos.x(), mouse_box_thickness),
            },
            ..Default::default()
        })
        .with(MouseBox);

        let start_pt_line: f32 =
            pressed_pos.y() + ((current.y() - pressed_pos.y()) / 2.0) - win_height as f32 / 2.0;
        // right
        cmd.spawn(SpriteComponents {
            material: mouse_box_material,
            translation: Translation(Vec3::new(
                current.x() - (win_width as f32 / 2.0),
                start_pt_line,
                0.0,
            )),
            sprite: Sprite {
                size: Vec2::new(mouse_box_thickness, current.y() - pressed_pos.y()),
            },
            ..Default::default()
        })
        .with(MouseBox);
        // left
        cmd.spawn(SpriteComponents {
            material: mouse_box_material,
            translation: Translation(Vec3::new(
                pressed_pos.x() - (win_width as f32 / 2.0),
                start_pt_line,
                0.0,
            )),
            sprite: Sprite {
                size: Vec2::new(mouse_box_thickness, current.y() - pressed_pos.y()),
            },
            ..Default::default()
        })
        .with(MouseBox);
    } else {
        for (e, _mb) in &mut q.iter() {
            cmd.despawn(e);
        }
    }
}

fn mouse_input_system(
    mut event_state: ResMut<MouseEventsState>,
    mut state: ResMut<MouseState>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_cursor_events: Res<Events<CursorMoved>>,
) {
    for event in event_state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        // println!(
        //     "mouse button event {:?}, position: x: {}, y: {}",
        //     event, state.x, state.y
        // );
        match event.state {
            ElementState::Pressed => {
                state.pressed_at = Some(Vec2::new(state.pos.x(), state.pos.y()));
                state.released_at = None;
            }
            ElementState::Released => {
                state.released_at = Some(Vec2::new(state.pos.x(), state.pos.y()));
                println!(
                    "made a box from {:?} to {:?}",
                    state.pressed_at, state.released_at
                );
                state.pressed_at = None;
            }
        }
    }

    for event in event_state
        .cursor_moved_event_reader
        .iter(&mouse_cursor_events)
    {
        // println!("{:?}", event);
        state.pos.set_x(event.position.x());
        state.pos.set_y(event.position.y());
    }
}
