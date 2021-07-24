use bevy::prelude::*;
use heron::prelude::*;

fn main() {
    let mut app = App::build();
    
    app.add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    
    app.add_plugin(PhysicsPlugin::default())
       .add_startup_system(setup.system())
       .add_startup_system(spawn_ship.system())
       .add_system(ship_control.system());

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_ship(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    let texture_handle = asset_server.load("ship.png");

    commands
         .spawn_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(-480.0, 0.0, 0.0)),
            ..Default::default()
        })

        // .spawn_bundle(SpriteBundle::default())

        // Make it a rigid body
        .insert(RigidBody::Dynamic)
        
        // Attach a collision shape
        .insert(CollisionShape::Capsule { radius: 18.0, half_segment: 8.0 })
        // .insert(CollisionShape::Sphere { radius: 10.0 })
        .insert(PlayerShip::default())
        
        // Optionally add other useful components...
        //.insert(Velocity::from_linear(Vec3::X * 2.0))
        //.insert(Acceleration::from_linear(Vec3::X * 1.0))
        .insert(PhysicMaterial { friction: 1.0, density: 10.0, ..Default::default() })
        .insert(RotationConstraints::lock())
        .insert(CollisionLayers::none().with_group(Layer::Player).with_mask(Layer::World));
}

fn ship_control(keys: Res<Input<KeyCode>>, 
    mut commands: Commands,
    ship_query: Query<(Entity, &PlayerShip)>,
    ) {

    let (entity, playership) = ship_query.single().unwrap();
    let mut ship = commands.entity(entity);
    if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
        println!("Key pressed UP!");
        ship.insert(Velocity::from_linear(Vec3::Y * playership.speed));
    }

    if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
        ship.insert(Velocity::from_linear(Vec3::Y * -playership.speed));
    }

}

struct PlayerShip {
    speed: Vec3,
}

impl Default for PlayerShip {
    fn default() -> PlayerShip {
        PlayerShip { speed: Vec3::new(0.0, 128.0, 0.0) }
    }
}


// Define your physics layers
#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
    Enemies,
}
