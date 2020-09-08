use bevy::{
    input::keyboard::ElementState, input::mouse::MouseButtonInput, prelude::*,
    render::pass::ClearColor,
};

struct Unit;
struct Name(String);

#[derive(Debug)]
struct Selected(bool);

enum MouseBox {
    Left,
    Right,
    Top,
    Bottom,
}

enum Collider {
    Solid,
}

#[derive(Default)]
struct MouseEventsState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

#[derive(Debug)]
struct MouseBoxBounds {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

#[derive(Default)]
struct MouseState {
    pos: Vec2,
    pressed_at: Option<Vec2>,
    last_bounds: Option<MouseBoxBounds>,
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
    mut query: Query<(&Unit, &Name, &Translation, &Selected)>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (_unit, name, translation, selected) in &mut query.iter() {
            println!(
                "name: {}, position: {} {}, selected: {:?}",
                name.0,
                translation.x(),
                translation.y(),
                selected
            );
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
    let nacho_texture = asset_server
        .load("assets/textures/nacho.png")
        .expect("can load minka");
    let chip_texture = asset_server
        .load("assets/textures/chippi.png")
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
        .with(Collider::Solid)
        .with(Selected(false))
        .spawn(SpriteComponents {
            material: materials.add(nacho_texture.into()),
            translation: Translation(Vec3::new(100.0, 100.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(75.0, 100.0),
            },
            ..Default::default()
        })
        .with(Unit)
        .with(Name("Nacho".to_string()))
        .with(Collider::Solid)
        .with(Selected(false))
        .spawn(SpriteComponents {
            material: materials.add(chip_texture.into()),
            translation: Translation(Vec3::new(200.0, 200.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(100.0, 100.0),
            },
            ..Default::default()
        })
        .with(Unit)
        .with(Name("Chip".to_string()))
        .with(Collider::Solid)
        .with(Selected(false));

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
            .add_system(selection_system.system())
            .add_system(print_name_and_position_system.system());
    }
}

fn mouse_box_system(
    mut cmd: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    mouse_state: ResMut<MouseState>,
    mut q: Query<(&mut Translation, &mut Sprite, Entity, &MouseBox)>,
) {
    let window = windows.get_primary();
    let win_height = window.expect("there is a window").height;
    let win_width = window.expect("there is a window").width;

    if mouse_state.pressed_at.is_some() {
        let mouse_box_material = materials.add(Color::rgb(0.2, 0.8, 0.2).into());
        let mouse_box_thickness = 1.0;
        let current = mouse_state.pos;
        let pressed_pos = mouse_state.pressed_at.expect("is there");

        let entity_count = q.iter().iter().count();
        if entity_count > 0 {
            for (mut translation, mut sprite, _e, mb) in &mut q.iter() {
                match mb {
                    MouseBox::Top => {
                        translation.set_x(
                            pressed_pos.x() + ((current.x() - pressed_pos.x()) / 2.0)
                                - win_width as f32 / 2.0,
                        );
                        translation.set_y(pressed_pos.y() - (win_height as f32 / 2.0));
                        sprite.size = Vec2::new(current.x() - pressed_pos.x(), mouse_box_thickness);
                    }
                    MouseBox::Bottom => {
                        translation.set_x(
                            pressed_pos.x() + ((current.x() - pressed_pos.x()) / 2.0)
                                - win_width as f32 / 2.0,
                        );
                        translation.set_y(current.y() - (win_height as f32 / 2.0));
                        sprite.size = Vec2::new(current.x() - pressed_pos.x(), mouse_box_thickness);
                    }
                    MouseBox::Right => {
                        translation.set_x(current.x() - (win_width as f32 / 2.0));
                        translation.set_y(
                            pressed_pos.y() + ((current.y() - pressed_pos.y()) / 2.0)
                                - win_height as f32 / 2.0,
                        );
                        sprite.size = Vec2::new(mouse_box_thickness, current.y() - pressed_pos.y());
                    }
                    MouseBox::Left => {
                        translation.set_x(pressed_pos.x() - (win_width as f32 / 2.0));
                        translation.set_y(
                            pressed_pos.y() + ((current.y() - pressed_pos.y()) / 2.0)
                                - win_height as f32 / 2.0,
                        );
                        sprite.size = Vec2::new(mouse_box_thickness, current.y() - pressed_pos.y());
                    }
                }
            }
        } else {
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
            .with(MouseBox::Top);
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
            .with(MouseBox::Bottom);

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
            .with(MouseBox::Right);
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
            .with(MouseBox::Left);
        }
    } else {
        for (_t, _s, e, _mb) in &mut q.iter() {
            cmd.despawn(e);
        }
    }
}

fn selection_system(
    windows: Res<Windows>,
    mut mouse_state: ResMut<MouseState>,
    mut query: Query<(&Unit, &mut Selected, &Translation)>,
) {
    let window = windows.get_primary();
    let win_height = window.expect("there is a window").height as f32;
    let win_width = window.expect("there is a window").width as f32;

    match mouse_state.last_bounds {
        Some(ref bounds) => {
            // TODO: if the bounding box is fully inside a unit, select that unit
            for (_unit, mut selection, translation) in &mut query.iter() {
                let unit_x = translation.x();
                let unit_y = translation.y();

                selection.0 = unit_x < (bounds.right - win_width / 2.0)
                    && unit_x > (bounds.left - win_width / 2.0)
                    && unit_y < (bounds.top - win_height / 2.0)
                    && unit_y > (bounds.bottom - win_height / 2.0);
            }
        }
        None => {}
    };
    mouse_state.last_bounds = None;
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
        match event.state {
            ElementState::Pressed => {
                state.last_bounds = None;
                state.pressed_at = Some(Vec2::new(state.pos.x(), state.pos.y()));
            }
            ElementState::Released => {
                let released_at = Vec2::new(state.pos.x(), state.pos.y());
                let pressed_at = state.pressed_at.expect("is there");
                println!(
                    "made a box from {:?} to {:?}",
                    state.pressed_at, released_at
                );
                state.last_bounds = Some(MouseBoxBounds {
                    top: std::primitive::f32::max(pressed_at.y(), released_at.y()),
                    bottom: std::primitive::f32::min(pressed_at.y(), released_at.y()),
                    left: std::primitive::f32::min(pressed_at.x(), released_at.x()),
                    right: std::primitive::f32::max(pressed_at.x(), released_at.x()),
                });
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
