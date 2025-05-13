use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

use crate::ReceivesInput;

pub struct CannonPlugin;

impl Plugin for CannonPlugin
{
    fn build(&self, app: &mut App) 
    {
        app.add_systems(Startup, cannon_setup);
        app.add_systems(
            Update, (cannon_change_power, calculate_cannon_arc, draw_cannon_arc)
        );
        app.add_systems(
            FixedUpdate, (cannon_action, apply_angular_vel).chain()
        );
    }
}

fn cannon_setup(mut commands: Commands)
{
    commands.insert_resource(CannonPoller(Timer::from_seconds(1.0, TimerMode::Repeating)));
    commands.spawn(CannonBundle::default())
    .with_children(|cannon|
    {
        cannon.spawn(
            CannonAimSpotBundle {
                cannon_aim_spot: CannonAimSpot,
                sprite: Sprite::sized([3.0, 3.0].into()),
                transform: Transform::from_translation([0.0, 0.0, 0.0].into()),
            },
            // (LinePath { points: vec![] }, Transform::default()),
        );
        cannon.spawn((
            LinePath { points: vec![] },
            Transform::default(),
            InheritedVisibility::default()
        ));
    });
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

#[derive(Component)]
struct CannonAimSpot;

#[derive(Resource)]
struct CannonPoller(Timer);

#[derive(Bundle)]
struct CannonAimSpotBundle
{
    cannon_aim_spot: CannonAimSpot,
    sprite: Sprite,
    transform: Transform,
}

#[derive(Component, Debug, Copy, Clone)]
struct Power(f32);

#[derive(Component)]
struct AngularVelocity(f32);

#[derive(Component)]
struct LinePath
{
    points: Vec<Vec3>,
}

#[derive(Component)]
#[require(Transform, Sprite)]
struct LinePathPoint;

#[derive(Component)]
struct LinePoint;

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
                power.0 = power.0.clamp(30.0, 90.0);
            }

            // println!("Cannon power: {:?}", power.0);
        }
    }
}

fn calculate_cannon_arc(
    // mut commands: Commands,
    cannon_query: Single<(&Children, &Power), With<Cannon>>,
    mut line_path: Query<&mut LinePath>,
    mut aim_spot: Query<&mut Transform, With<CannonAimSpot>>,
)
{
    // println!("Calculating cannon arc...");

    const VERT_ANGLE: f32 = 35.0_f32.to_radians();

    let (ch, &Power(p)) = cannon_query.into_inner();

    let dx = p * p * VERT_ANGLE.cos() * VERT_ANGLE.sin() / 4.9;
    
    let n_points: i32 = 50;
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

    // Gather children, provided they are found
    let mut lp = None;
    let mut aim_spot_transform = None;
    for &child in ch
    {
        if lp.is_none()
        {
            lp = line_path.get_mut(child).ok();
        }
        if aim_spot_transform.is_none()
        {
            aim_spot_transform = aim_spot.get_mut(child).ok();
        }
        // aim_spot_transform = aim_spot.get_mut(child).ok();
    }
    
    if let Some(mut lp) = lp
    {
        lp.points = points;

        // const ANGLE_UP: f32 = 60.0;
        // const ANGLE_UP_RAD: f32 = ANGLE_UP.to_radians();
        // const ANGLE_UP_COMP_RAD: f32 = (90.0 - ANGLE_UP).to_radians();
    
        // let transformed_points = 
        // lp.points.clone().into_iter()
        // .map(
        // |mut v|
        // {
        //     let dy = v.z * ANGLE_UP_COMP_RAD.sin() / ANGLE_UP_RAD.sin();
        //     v.y += dy;
    
        //     v
        // });
    
        // for pt in transformed_points
        // {
        //     let mut sprite = Sprite::sized([3.0, 3.0].into());
        //     let color = Color::Srgba(Srgba::RED);
        //     sprite.color = color;
        //     let transform = Transform::from_translation(pt);
    
        //     commands.spawn((
        //         sprite, transform, MiddlePoint
        //     ));
        // }
    }

    if let Some(mut aim_spot_transform) = aim_spot_transform
    {
        // println!("aim_spot_transform: {:?}", aim_spot_transform);

        aim_spot_transform.translation = Vec3::new(dx, 0.0, 0.0);
    }


    // aim_spot.transform.translation = Vec3::new(dx, 0.0, 0.0);
    

    // let mut aim_spot_t = aim_spot.into_inner();
    // aim_spot_t.translation = Vec3::new(dx, 0.0, 0.0);
    // commands.spawn((
    //     Sprite::sized([3.0, 3.0].into()),
    //     Transform::from_translation(pos + Vec3::new(dx, 0.0, 0.0)),
    //     CannonAimSpot,
    // ));
}

fn draw_cannon_arc(
    mut commands: Commands,
    line_path: Query<(Entity, Option<&Children>, &LinePath)>,
)
{
    // println!("line_path: {:?}", line_path);

    // println!("Drawing cannon arc...");

    // commands.entity(entity).

    for (entity, children, lp) in line_path
    {
        if let Some(children) = children
        {
            for &child in children
            {
                commands.entity(child).despawn();
            }
        }

        else
        {
            for pt in &lp.points
            {
                const ANGLE_UP: f32 = 60.0;
                const ANGLE_UP_RAD: f32 = ANGLE_UP.to_radians();
                const ANGLE_UP_COMP_RAD: f32 = (90.0 - ANGLE_UP).to_radians();
             
                // let transformed_points = 
                // lp.points.clone().into_iter()
                // .map(
                // |mut v|
                // {
                //     let dy = v.z * ANGLE_UP_COMP_RAD.sin() / ANGLE_UP_RAD.sin();
                //     v.y += dy;
            
                //     v
                // });

                let transformed_point = {
                    let dy = pt.z * ANGLE_UP_COMP_RAD.sin() / ANGLE_UP_RAD.sin();
                    Vec3::new(pt.x, pt.y + dy, pt.z)
                };

                let mut sprite = Sprite::sized([3.0, 3.0].into());
                let color = Color::Srgba(Srgba::RED);
                sprite.color = color;
                let transform = Transform::from_translation(transformed_point);


                commands.entity(entity).with_child((
                    sprite, transform, LinePathPoint
                ));
            }

            // const ANGLE_UP: f32 = 60.0;
            // const ANGLE_UP_RAD: f32 = ANGLE_UP.to_radians();
            // const ANGLE_UP_COMP_RAD: f32 = (90.0 - ANGLE_UP).to_radians();
        
            // let transformed_points = 
            // lp.points.clone().into_iter()
            // .map(
            // |mut v|
            // {
            //     let dy = v.z * ANGLE_UP_COMP_RAD.sin() / ANGLE_UP_RAD.sin();
            //     v.y += dy;
        
            //     v
            // });
        
            // for pt in transformed_points
            // {
            //     let mut sprite = Sprite::sized([3.0, 3.0].into());
            //     let color = Color::Srgba(Srgba::RED);
            //     sprite.color = color;
            //     let transform = Transform::from_translation(pt);
        
            //     commands.spawn((
            //         sprite, transform, MiddlePoint
            //     ));
            // }
        }


        // println!("Line path: {:?}", lp.points);

        // // clear all children of this line_path
        // for &child in children
        // {
        //     commands.entity(child).despawn();
        // }

        // // spawn new children of this line_path, with type LinePathPoint
        // for pt in &lp.points
        // {
        //     let mut sprite = Sprite::sized([3.0, 3.0].into());
        //     let color = Color::Srgba(Srgba::RED);
        //     sprite.color = color;
        //     let transform = Transform::from_translation(*pt);
            

        //     let id = commands.spawn((
        //         LinePathPoint, sprite, transform
        //     )).id();

        //     commands.entity(entity).add_child(id);
        //     // commands.spawn((
        //     //     sprite, transform, LinePathPoint
        //     // ));
        // }
    }
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

        if !q && !e && av.0 != 0.0
        {
            av.0 -= 0.1 * av.0.signum();
        }

        if av.0.abs() < 0.001
        {
            av.0 = 0.0;
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