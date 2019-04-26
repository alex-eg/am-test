use amethyst::{
    assets::{
        Prefab, PrefabLoader, PrefabLoaderSystem, RonFormat,
        Handle
    },
    controls::{FlyControlBundle, FlyControlTag, HideCursor},
    ecs::{Component, DenseVecStorage},
    prelude::*,
    input::{is_close_requested, is_key_down, InputBundle},
    renderer::{DisplayConfig, Pipeline, RenderBundle, Stage,
               Camera, Projection, PosNormTex, DrawShaded,
               VirtualKeyCode},
    core::transform::{Transform, TransformBundle},
    utils::{
        application_root_dir,
        scene::BasicScenePrefab,
    },
    core::math::Vector3,
    Error,
};

type CubePrefabData = BasicScenePrefab<Vec<PosNormTex>>;

#[derive(Default)]
struct CubePrefabDataSet(pub Vec<Handle<Prefab<CubePrefabData>>>);

impl Component for CubePrefabDataSet
{
    type Storage = DenseVecStorage<Self>;
}

struct MainState;

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.register::<CubePrefabDataSet>();
        world.add_resource(CubePrefabDataSet::default());
        set_camera(world);
        let handle = world.exec(|loader: PrefabLoader<CubePrefabData>| {
            loader.load("cube.ron", RonFormat, (), ())
        });
        world.create_entity().with(handle.clone()).build();
        world.write_resource::<CubePrefabDataSet>().0.push(handle);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { world, .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = false;
            }
        }

        Trans::None
    }
}

fn set_camera(world: &mut World) {
    let mut transform = Transform::default();
    let pos = Vector3::new(0.0, 2.0, 3.0);
    transform.set_translation(pos);
    transform.face_towards(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
    world.create_entity()
        .with(FlyControlTag)
        .with(Camera::from(Projection::perspective(4.0 / 3.0, 0.87)))
        .with(transform)
        .build();
}

pub fn run() -> Result<(), Error> {

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    println!("App root: {}", app_root.as_path().display());

    let assets_dir = app_root.join("assets/");
    println!("assets dir: {}", assets_dir.as_path().display());

    let display_config_path = app_root.join("resources/display_config.ron");
    let config = DisplayConfig::load(&display_config_path);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.15, 0.3, 0.3, 1.0], 1.0)
            .with_pass(DrawShaded::<PosNormTex>::new()),
    );

    let keys_config = app_root.join("resources/input.ron");

    let game_data = GameDataBuilder::default()
        .with(PrefabLoaderSystem::<CubePrefabData>::default(), "", &[])
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(FlyControlBundle::<String, String>::new(
            Some(String::from("move_x")),
            Some(String::from("move_y")),
            Some(String::from("move_z")),
        )
                     .with_speed(5.0)
                     .with_sensitivity(0.1, 0.1))?
        .with_bundle(TransformBundle::new().with_dep(&["fly_movement"]))?
        .with_bundle(InputBundle::<String, String>::new()
                     .with_bindings_from_file(&keys_config)?)?;

    let mut game = Application::new(assets_dir, MainState, game_data)?;
    game.run();
    Ok(())
}
