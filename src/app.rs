use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, Pipeline, RenderBundle, Stage, DrawFlat2D,
               Camera, Projection,
               Mesh, MeshBuilder},
    utils::application_root_dir,
    assets::{AssetStorage, Loader},
    core::transform::{Transform, TransformBundle},
};

struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        set_camera(data.world);
    }
}

fn set_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(5.0, 5.0, 5.0);
    world.create_entity()
        .with(Camera::from(Projection::perspective(4.0 / 3.0, 50.0)))
        .with(transform)
        .build();
}

pub fn run() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.15, 0.3, 0.3, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::new("./", MainState, game_data)?;

    game.run();

    Ok(())
}
