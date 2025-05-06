use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiContextPass};

// use bevy::bevy_window::PrimaryWindow;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, HelloPlugin, EguiPlugin { enable_multipass_for_primary_context: true }))
    .add_systems(EguiContextPass, ui_example_system)
    .run();
}

fn ui_example_system(mut contexts: EguiContexts, query: Query<&Name, With<Person>>)
{
    egui::Window::new("Hello").show(
        contexts.ctx_mut(), 
        |ui|
        {
            // ui.checkbox(checked, text)
            if ui.button("button").clicked()
            {
                println!("Button clicked!");
            }
            ui.label("World!");
            for name in &query
            {
                ui.label(format!("Hello, {}!", name.0));
            }
        }
    );
}

#[derive(Component)]
struct CannonAimSpot;

#[derive(Resource)]
struct CannonPoller(Timer);

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
        app.add_systems(Update, (window_draw, player_movement, camera_movement, (cannon_change_power, cannon_action, apply_angular_vel, calculate_and_draw_cannon_arc).chain()));
    }
}

fn window_draw(
    mut commands: Commands,
    mut primary_window: Query<&mut Window>,
    mut cam: Single<&mut Camera2d>
) -> ()
{
    let Ok(mut window) = primary_window.single_mut() else { return; };
    let window = &mut *window;
    
    let pos = window.cursor_position();

    // if let Some(pos) = pos
    // {
    //     println!("Cursor position: {:?}", pos);
    // }
    // else
    // {
    //     println!("Cursor position: None");
    // }

    window.title = String::from("New Window Title");
    // window.transparent = true;

    // window.set_cursor_visible(false);
    // window.set_cursor_grab_mode(CursorGrabMode::Confined);
    // window.set_cursor_grab_mode(CursorGrabMode::Locked);
    // window.set_cursor_position(Vec2::new(0.0, 0.0));
    // cam.set_position(Vec3::new(0.0, 0.0, 0.0));
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

#[derive(Component)]
struct Cannon;

#[derive(Bundle)]
struct CannonBundle
{
    cannon: Cannon,
    sprite: Sprite,
    angular_velocity: AngularVelocity,
    transform: Transform,
    receives_input: ReceivesInput,
    power: Power,
}

#[derive(Component)]
struct LinePath
{
    points: Vec<Vec3>,
}

#[derive(Component, Debug, Copy, Clone)]
struct Power(f32);

#[derive(Component)]
struct Angle(f32);

#[derive(Component)]
struct AngularVelocity(f32);

impl Default for CannonBundle
{
    fn default() -> Self
    {
        use bevy::sprite::Anchor;

        let mut sprite = Sprite::sized([30.0, 10.0].into());
        sprite.color = Color::Srgba(Srgba::BLUE);
        sprite.anchor = Anchor::CenterLeft;

        Self
        {
            cannon: Cannon,
            sprite,
            angular_velocity: AngularVelocity(0.0),
            transform: Transform::default(),
            receives_input: ReceivesInput {active: true},
            power: Power(100.0),
        }
    }
}

fn setup(mut commands: Commands)
{
    commands.spawn((MyMainCamera, Camera2d::default(), Transform::default(), ReceivesInput {active: true} ));
    commands.spawn((Player, Sprite::default(), Transform::default(), ReceivesInput {active: true} ));
    
    let pos2 = Transform::from_translation([5.0, 5.0, 0.0].into());
    commands.spawn((Sprite::default(), pos2, Player, ReceivesInput {active: false}));
    commands.insert_resource(CannonPoller(Timer::from_seconds(1.0, TimerMode::Repeating)));
    commands.spawn(CannonBundle::default());
    commands.spawn((
        Sprite::sized([3.0, 3.0].into()),
        Transform::from_translation([0.0, 0.0, 0.0].into()),
        CannonAimSpot,
    ));
}

fn cannon_change_power(
    query: Query<(&mut Power, &ReceivesInput), With<Cannon>>,
    mut mouse_wheel: EventReader<MouseWheel>,
)
{
    for (mut power, ri) in query
    {
        if ri.active
        {
            for ev in mouse_wheel.read()
            {
                power.0 += ev.y;
                power.0 = power.0.clamp(40.0, 160.0);
            }

            println!("Cannon power: {:?}", power.0);
        }
    }
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

fn calculate_and_draw_cannon_arc(
    mut commands: Commands,
    query: Single<(&Transform, &Power), With<Cannon>>,
    aim_spot: Single<&mut Transform, (With<CannonAimSpot>, Without<Cannon>)>,
)
{
    // let a = commands.entity(aim_spot);
    // let aim_spot = aim_spot.iter().next();
    // if let Some(aim_spot) = aim_spot
    // {
    //     // commands.entity(aim_spot).despawn();
    // }

    const VERT_ANGLE: f32 = 35.0_f32.to_radians();

    let (transform, &Power(p)) = query.into_inner();

    // let angle = transform.rotation.angle_between(Quat::from_rotation_z(0.0));

    // let facing_vec = transform.forward();
    let pos = transform.translation;

    let facing_vec: Vec2 = [1.0, 0.0].into();

    let dx = p * p * VERT_ANGLE.cos() * VERT_ANGLE.sin() / 4.9;

    for i in 0..100
    {
        let t = i as f32 / 100.0 * p * VERT_ANGLE.sin() / 4.9;
        let z = -4.9 * t * t + p * VERT_ANGLE.sin() * t;
    }
    
    let n_points: i32 = 100;
    let points = 
    (0..n_points).into_iter()
    .map( |i| i as f32 / n_points as f32)
    .map(
    |t|
    {
        let t = t * p * VERT_ANGLE.sin() / 4.9;
        let z = -4.9 * t * t + p * VERT_ANGLE.sin() * t;
        let x = p * VERT_ANGLE.cos() * t;
        // println!("t: {:?}, x: {:?}, z: {:?}", t, x, z);
        Vec3::new(x, 0.0, z)
    })
    .collect::<Vec<Vec3>>();

    const ANGLE_UP: f32 = 60.0;
    const ANGLE_UP_RAD: f32 = ANGLE_UP.to_radians();
    const ANGLE_UP_COMP_RAD: f32 = (90.0 - ANGLE_UP).to_radians();

    let transformed_points = 
    points.clone().into_iter()
    .map(
    |mut v|
    {
        let dy = v.z * ANGLE_UP_COMP_RAD.sin() / ANGLE_UP_RAD.sin();
        v.y += dy;

        v
    });

    for pt in transformed_points
    {
        let mut sprite = Sprite::sized([3.0, 3.0].into());
        let color = Color::Srgba(Srgba::RED);
        sprite.color = color;
        let transform = Transform::from_translation(pt);

        commands.spawn((
            sprite, transform, MiddlePoint
        ));
    }






    let line_segments = 
    points.windows(2)
    .filter_map(
    |window| -> Option<LineSegmentBundle>
    {
        let [a, b] = window else { return None };

        let mid = a.midpoint(*b);
        let d = a.distance(*b);

        let mut sprite = Sprite::sized([d, 1.0].into());
        sprite.color = Color::Srgba(Srgba::RED);
        // sprite.anchor = bevy::sprite::Anchor::CenterLeft;

        let mut transform = Transform::default();
        transform.translation = mid;
        transform.look_at(Vec3::Z, Vec3::Z);

        let ls = LineSegmentBundle
        {
            line_segment: LineSegment,
            sprite,
            transform,
        };

        Some(ls)
    })
    .collect::<Vec<_>>();



    for ls in line_segments.into_iter()
    {
        // commands.spawn(ls);
    }
    


    // {

    // }

    let mut aim_spot_t = aim_spot.into_inner();
    aim_spot_t.translation = pos + Vec3::new(dx, 0.0, 0.0);
    // commands.spawn((
    //     Sprite::sized([3.0, 3.0].into()),
    //     Transform::from_translation(pos + Vec3::new(dx, 0.0, 0.0)),
    //     CannonAimSpot,
    // ));
}

fn cannon_action(
    mut query: Query<(&mut AngularVelocity, &ReceivesInput), With<Cannon>>,
    keys: Res<ButtonInput<KeyCode>>,
)
{
    let query = 
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

    let q = keys.pressed(KeyCode::KeyQ);
    let e  = keys.pressed(KeyCode::KeyE);

    for mut av in query
    {
        if keys.pressed(KeyCode::KeyQ)
        {
            av.0 += 0.05
        }
        if keys.pressed(KeyCode::KeyE)
        {
            av.0 -= 0.05;
        }

        av.0 = av.0.clamp(-3.0, 3.0);

        if !q && !e
        {
            av.0 -= 0.1 * av.0.signum();
        }
    }
}

fn apply_angular_vel(
    query: Query<(&mut Transform, &AngularVelocity)>,
    mut timer: ResMut<CannonPoller>,
    time: Res<Time>
)
{
    for (mut transform, av) in query
    {
        // angle.0 += av.0 * time.delta_secs();
        
        // let angle = transform.rotation.angle_between(Quat::from_rotation_z(0.0));
        // let new_angle = angle + av.0 * time.delta_secs();

        let delta_angle = av.0 * time.delta_secs();
        // let angle = angle.lerp(angle + av.0, time.delta_secs());
        
        
        if timer.0.tick(time.delta()).just_finished()
        {
            // println!("angle: {:?}", angle);
            // println!("new angle: {:?}", new_angle);
            println!("angular velocity: {:?}", av.0);
        }
        // angle.0 = angle.0.clamp(0.0, 2.0 * std::f32::consts::PI);
        
        // transform.rotation = Quat::from_rotation_z(new_angle);
        transform.rotate_z(delta_angle);
        // transform.rotation = transform.rotation.rotate_towards(, max_angle)
        // transform.rotation = Quat::from_rotation_z(angle.0);
        // transform.rotation = transform.rotation.lerp(Quat::from_rotation_z(angle), time.delta_secs());
        // transform.rotation = transform.rotation.slerp(Quat::from_rotation_z(angle), time.delta_secs());
    }

    // for (mut transform, av) in query
    {
        // transform.rotation = Quat::from_rotation_z(av.0 * time.delta_seconds());
        // transform.rotation.lerp(Quat::from_rotation_z(angle), time.delta_secs())
    }
}


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