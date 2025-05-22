use bevy::prelude::*;

use super::ReceivesInput;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App) 
    {
        app.add_systems(Startup, player_setup);
        app.add_systems(Update, player_movement);
    }
}

fn player_setup(mut commands: Commands)
{
    commands.spawn(
        PlayerBundle
        {
            player: Player,
            sprite: Sprite::default(),
            transform: Transform::default(),
            receives_input: ReceivesInput {active: true},
        }
    );
}

#[derive(Bundle)]
struct PlayerBundle
{   
    player: Player,
    sprite: Sprite,
    transform: Transform,
    receives_input: ReceivesInput,
}

#[derive(Component)]
struct Player;

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