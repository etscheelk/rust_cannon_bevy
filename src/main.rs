use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

use bevy_egui::EguiPlugin;

use std::ops::RangeInclusive;

mod fractal;
mod cannon;
mod player;

use fractal::FractalPlugin;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins, 
        CameraMover, 
        EguiPlugin { enable_multipass_for_primary_context: true },
        FractalPlugin,
        // player::PlayerPlugin,
        // cannon::CannonPlugin,
    ))
    .run();
}

pub struct CameraMover;

impl Plugin for CameraMover
{
    fn build(&self, app: &mut App) 
    {
        app.add_systems(Startup, setup);
        app.add_systems(Update, camera_movement);
    }
}

#[derive(Component)]
struct MyMainCamera;

#[derive(Component, Deref, Debug, Clone, Copy)]
struct Zoom(f32);

#[derive(Component, Deref, Debug, Clone)]
struct AllowedZooms(RangeInclusive<f32>);

#[derive(Component)]
struct ReceivesInput
{
    active: bool
}

fn setup(
    mut commands: Commands,
    mut window: Single<&mut Window>,
)
{
    commands.spawn((
        MyMainCamera, 
        Camera2d::default(), 
        Transform::default(), 
        ReceivesInput {active: true}, 
        Zoom(1.0), 
        AllowedZooms(0.25..=2.0)
    ));

    window.resolution.set_physical_resolution(1024, 1024);
}

fn camera_movement(
    query: Query<(&mut Transform, &ReceivesInput, &mut Zoom, &AllowedZooms), With<MyMainCamera>>,
    _keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_movement: EventReader<MouseMotion>,
    mut scroll_wheel: EventReader<MouseWheel>,
)
{
    let mut delta_movement = Vec2::ZERO;
    if mouse.pressed(MouseButton::Left)
    {
        for ev in mouse_movement.read()
        {  
            delta_movement += ev.delta;
        }
    }

    let mut delta_zoom = 0.0;
    for scroll in scroll_wheel.read()
    {
        delta_zoom += 0.1 * scroll.y;
    }

    for (mut transform, ri, mut zoom, azs) in query
    {
        if ri.active
        {
            zoom.0 += delta_zoom;
            zoom.0 = zoom.0.clamp(*azs.start(), *azs.end());
            
            transform.scale = Vec3::ONE / zoom.0;
            
            transform.translation += Vec3::from([-delta_movement.x / zoom.0, delta_movement.y / zoom.0, 0.0]);
        }
    }

    // for mut transform in transform_query
    // {
    //     if keys.pressed(KeyCode::ArrowUp)
    //     {
    //         transform.translation.y += 5.0;
    //     }
    //     if keys.pressed(KeyCode::ArrowDown)
    //     {
    //         transform.translation.y -= 5.0;
    //     }
    //     if keys.pressed(KeyCode::ArrowLeft)
    //     {
    //         transform.translation.x -= 5.0;
    //     }
    //     if keys.pressed(KeyCode::ArrowRight)
    //     {
    //         transform.translation.x += 5.0;
    //     }
    // }
}