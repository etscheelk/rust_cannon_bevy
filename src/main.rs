use bevy::prelude::*;
use bevy::window::WindowResolution;

use bevy_egui::EguiPlugin;

mod fractal;
mod cannon;

use fractal::FractalPlugin;
use cannon::CannonPlugin;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins, 
        HelloPlugin, 
        EguiPlugin { enable_multipass_for_primary_context: true },
        FractalPlugin,
        // CannonPlugin,
    ))
    .run();
}

#[derive(Bundle)]
struct MyPlayer
{   
    player: Player,
    sprite: Sprite,
    transform: Transform,
    receives_input: ReceivesInput,
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin
{
    fn build(&self, app: &mut App) 
    {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, (setup, add_people, hello_world));
        app.add_systems(Update, (update_people, greet_people).chain());
        app.add_systems(Update, (window_draw, player_movement, camera_movement));
        // app.add_event::<FractalEvent>();
        // app.add_systems(Update, (fractal_event, handle_compute_fractal));
    }
}

fn window_draw(
    mut commands: Commands,
    mut primary_window: Query<&mut Window>,
    mut cam: Single<&mut Camera2d>
)
{
    let Ok(mut window) = primary_window.single_mut() else { return; };
    let window = &mut *window;

    window.resolution = WindowResolution::new(1024., 1024.);
    window.title = String::from("New Window Title");
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MyMainCamera;

#[derive(Component)]
struct ReceivesInput
{
    active: bool
}

fn setup(mut commands: Commands)
{
    commands.spawn((MyMainCamera, Camera2d::default(), Transform::default(), ReceivesInput {active: true} ));
    commands.spawn((Player, Sprite::default(), Transform::default(), ReceivesInput {active: true} ));
    
    let pos2 = Transform::from_translation([5.0, 5.0, 0.0].into());
    commands.spawn((Sprite::default(), pos2, Player, ReceivesInput {active: false}));
    // commands.insert_resource(CannonPoller(Timer::from_seconds(1.0, TimerMode::Repeating)));
    // commands.spawn(CannonBundle::default())
    // .with_children(|cannon|
    // {
    //     cannon.spawn(
    //         CannonAimSpotBundle {
    //             cannon_aim_spot: CannonAimSpot,
    //             sprite: Sprite::sized([3.0, 3.0].into()),
    //             transform: Transform::from_translation([0.0, 0.0, 0.0].into()),
    //         },
    //         // (LinePath { points: vec![] }, Transform::default()),
    //     );
    //     cannon.spawn((
    //         LinePath { points: vec![] },
    //         Transform::default(),
    //         InheritedVisibility::default()
    //     ));
    // });

    
    // .with_child(LinePath { points: vec![] })
    // .with_child(
    //     CannonAimSpotBundle {
    //         cannon_aim_spot: CannonAimSpot,
    //         sprite: Sprite::sized([3.0, 3.0].into()),
    //         transform: Transform::from_translation([0.0, 0.0, 0.0].into()),
    //     }
    // );
}



#[derive(Bundle)]
struct LineSegmentBundle
{
    line_segment: LineSegment,
    sprite: Sprite,
    transform: Transform,
}

#[derive(Component)]
struct LineSegment;

#[derive(Component)]
struct MiddlePoint;



fn camera_movement(
    mut query: Query<(&mut Transform, &ReceivesInput), With<MyMainCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
)
{
    let transform_query = 
    query.iter_mut()
    .filter_map(
    |(t, ri)|
    {
        if ri.active
        {
            Some(t)
        }
        else
        {
            None
        }
    });

    for mut transform in transform_query
    {
        if keys.pressed(KeyCode::ArrowUp)
        {
            transform.translation.y += 5.0;
        }
        if keys.pressed(KeyCode::ArrowDown)
        {
            transform.translation.y -= 5.0;
        }
        if keys.pressed(KeyCode::ArrowLeft)
        {
            transform.translation.x -= 5.0;
        }
        if keys.pressed(KeyCode::ArrowRight)
        {
            transform.translation.x += 5.0;
        }
    }
}



fn player_movement(
    mut query: Query<(&mut Transform, &ReceivesInput), With<Player>>, 
    keys: Res<ButtonInput<KeyCode>>
)
{
    let transform_query = 
    query.iter_mut()
    .filter_map(
    |(t, ri)|
    {
        if ri.active
        {
            Some(t)
        }
        else
        {
            None
        }
    });

    // for mut transform in query.iter_mut().filter_map(|(t, ri| if ri.active { Some(t) } else { None }))
    for mut transform in transform_query 
    {
        if keys.pressed(KeyCode::KeyW)
        {
            transform.translation.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS)
        {
            transform.translation.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA)
        {
            transform.translation.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD)
        {
            transform.translation.x += 1.0;
        }
    }
}

fn hello_world()
{
    println!("Hello, world!");
}

#[derive(Component)]
#[require(Name)]
struct Person;

#[derive(Component, Debug, Clone, Default)]
struct Name(String);

#[derive(Bundle)]
struct PersonBundle
{
    person: Person,
    name: Name,
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands)
{
    commands.spawn((Person, Name(String::from("Elaina Proctor"))));
    commands.spawn((Person, Name(String::from("Renzo Humo"))));
    commands.spawn((Person, Name(String::from("Zayna Hieves"))));

    // commands.spawn(PersonBundle {
    //     person: Person,
    //     name: Name(String::from("Elaina Proctor")),
    // });
}

fn update_people(mut query: Query<&mut Name, With<Person>>)
{
    for mut name in &mut query
    {
        if name.0 == "Elaina Proctor" 
        {
            name.0 = String::from("Elaina Proctor (Updated)");
            break;
        }
    }
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>)
{
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    // if timer.0.tick(time.delta()).just_finished()
    // {
    //     for name in &query
    //     {
    //         println!("Hello, {}!", name.0);
    //     }
    // }
}