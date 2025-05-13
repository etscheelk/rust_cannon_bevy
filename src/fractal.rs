use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::tasks::{block_on, poll_once, AsyncComputeTaskPool, Task};
use bevy::sprite::Anchor;
use bevy::render::render_resource::{Extent3d, TextureFormat};
use bevy::asset::RenderAssetUsages;

use bevy_egui::{EguiContextPass, EguiContexts};
use bevy_egui::egui;

use RustFractal::my_grid::grid_32::MyColorImage;
use RustFractal::fractal::{Fractalize, FractalizeParameters};

pub struct FractalPlugin;

impl Plugin for FractalPlugin
{
    fn build(&self, app: &mut App) 
    {
        app
        .add_event::<FractalEvent>()
        .add_systems(EguiContextPass, fractal_gui)
        .add_systems(Startup, fractal_setup)
        .add_systems(Update, (fractal_event, handle_compute_fractal))
        ;
    }
}

fn fractal_setup(
    mut commands: Commands,
)
{
    let fractal = MyColorImage::new(4096, 4096);
    let params = 
        FractalizeParameters::default()
        .with_max_points(25_000_000);

    commands.insert_resource(Fractal {
        fractal,
        params,
    });
    commands.insert_resource(FractalSettingsMenu {
        f_theta_offset: params.theta_offset,
        f_rot: params.rot,
        u_num_points: params.max_points,
    });
}

#[derive(Event)]
enum FractalEvent
{
    Render,
    Settings(RustFractal::fractal::FractalizeParameters),
    Display,
}

#[derive(Resource)]
struct FractalSettingsMenu
{
    f_theta_offset: f32,
    f_rot: f32,
    u_num_points: u32,
}

#[derive(Component)]
struct FractalSprite;

#[derive(Resource, Clone)]
struct Fractal
{
    fractal: MyColorImage,
    params: FractalizeParameters,
}

impl Fractal
{
    /// An async implementation of the fractalize function.
    /// It can take a long time so it's good to make sure the rest of the app is running.
    fn compute_fractalize_async(&self, thread_pool: &AsyncComputeTaskPool) -> ComputeFractal
    {
        let mut frac = self.clone();

        let task = thread_pool.spawn(async move {
            frac.fractal.fractalize(frac.params);
            frac.fractal.pixels_mut().for_each(|p| p[3] = 0xff);

            frac
        });
        
        ComputeFractal { task }
    }
}

/// A 
#[derive(Component)]
struct ComputeFractal
{
    task: Task<Fractal>
}

fn handle_compute_fractal(
    mut commands: Commands,
    compute_fractal: Query<(Entity, &mut ComputeFractal)>,
    mut fractal: ResMut<Fractal>,
    mut fractal_ew: EventWriter<FractalEvent>,
)
{
    for (ent, mut task) in compute_fractal
    {
        if let Some(a) = block_on(poll_once(&mut task.task))
        {
            let b = fractal.as_mut();
            *b = a;

            commands.get_entity(ent).unwrap().despawn();
            fractal_ew.write(FractalEvent::Display);

            println!("Fractal rendering complete!!");
        }
    }
}

fn fractal_event(
    mut commands: Commands,
    mut events: EventReader<FractalEvent>,
    fractal_query: ResMut<Fractal>,
    asset_server: Res<AssetServer>,
    mut fractal_sprite: Option<Single<&mut Sprite, With<FractalSprite>>>,
)
{
    let thread_pool = AsyncComputeTaskPool::get();
    let fractal_query = fractal_query.into_inner();

    for event in events.read()
    {
        match event
        {
            FractalEvent::Render => 
            {
                fractal_query.fractal.pixels_mut().for_each(|p| { p[0] = 0; p[1] = 0; p[2] = 0; p[3] = 0; });
                let compute_fractal = Fractal::compute_fractalize_async(fractal_query, thread_pool);
                commands.spawn(compute_fractal);
                println!("Fractal rendering task created!");

                // fractal_query.fractal.fractalize(params.clone());
                // fractal_query.fractal.pixels_mut().for_each(|p| p[3] = 0xff);
                // println!("Render low done!")
            },
            FractalEvent::Settings(params) => 
            {
                println!("Settings: {:?}", params);
                fractal_query.params = params.clone();
            },
            FractalEvent::Display =>
            {
                println!("Display!");

                let tf = TextureFormat::Rgba8Unorm;

                let img = Image::new(
                    Extent3d {
                        depth_or_array_layers: 1,
                        height: 4096,
                        width: 4096,
                    },
                    bevy::render::render_resource::TextureDimension::D2,
                    fractal_query.fractal.clone().into_vec(),
                    tf,
                    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
                );

                let h = asset_server.add(img);

                let mut sprite = Sprite::from_image(h);
                sprite.anchor = Anchor::Center;
                let mut transform = Transform::from_translation([0.0, 0.0, -1.0].into());
                transform.scale = [1.0, 1.0, 1.0].into();

                if let Some(spr) = fractal_sprite.take()
                {
                    let spr = spr.into_inner().into_inner();
                    *spr = sprite;
                }
                else
                {
                    commands.spawn((
                        FractalSprite,
                        sprite,
                        transform,
                    ));
                }
            },
        }
    }
}

fn fractal_gui(
    mut contexts: EguiContexts,
    mut fractal_ew: EventWriter<FractalEvent>,
    settings_menu: ResMut<FractalSettingsMenu>,
    fractal: Res<Fractal>,
)
{
    let FractalSettingsMenu {f_theta_offset, f_rot, u_num_points} = settings_menu.into_inner();

    egui::Window::new("Hello").show(
        contexts.ctx_mut(), 
        |ui|
        {
            let mut params = fractal.params.clone();

            // ui.checkbox(checked, text)
            // if ui.button("button").clicked()
            // {
            //     println!("Button clicked!");
            // }
            if ui.button("Render").clicked()
            {
                fractal_ew.write(FractalEvent::Render);
            }
            if ui.button("Display").clicked()
            {
                fractal_ew.write(FractalEvent::Display);
            }
            
            // ui.label("theta offset value:");

            let num_points_slider = egui::Slider::new(u_num_points, 1_000_000..=500_000_000).logarithmic(true);
            ui.add(num_points_slider.text("Number of points"));

            let theta_offset_slider = egui::Slider::new(f_theta_offset, -PI..=PI);
            ui.add(theta_offset_slider.text("theta_offset slider"));

            let rot_slider = egui::Slider::new(f_rot, -PI..=PI);
            ui.add(rot_slider.text("Rot slider"));

            params.theta_offset = *f_theta_offset;
            params.rot = *f_rot;
            params.max_points = *u_num_points;

            if params != fractal.params
            {
                println!("Fractalize params changed: {:?}", &params);

                fractal_ew.write(FractalEvent::Settings(params));
            }

            if ui.button("save image").clicked()
            {
                let _ = fractal.fractal.save("my_image.png");
            }
        }
    );
}