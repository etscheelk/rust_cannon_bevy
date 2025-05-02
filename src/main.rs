use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiContextPass};

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, HelloPlugin, EguiPlugin { enable_multipass_for_primary_context: true }))
    .add_systems(EguiContextPass, ui_example_system)
    .run();
}

fn ui_example_system(mut contexts: EguiContexts)
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
        }
    );
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin
{
    fn build(&self, app: &mut App) 
    {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, (setup, add_people, hello_world));
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

fn setup(mut commands: Commands)
{
    commands.spawn(Camera2d);
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

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands)
{
    commands.spawn((Person, Name(String::from("Elaina Proctor"))));
    commands.spawn((Person, Name(String::from("Renzo Humo"))));
    commands.spawn((Person, Name(String::from("Zayna Hieves"))));
}

fn update_people(mut query: Query<&mut Name, With<Person>>)
{
    for mut name in &mut query
    {
        if name.0 == "Elaina Proctor" {
            name.0 = String::from("Elaina Proctor (Updated)");
            break;
        }
    }
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>)
{
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished()
    {
        for name in &query
        {
            println!("Hello, {}!", name.0);
        }
    }
}