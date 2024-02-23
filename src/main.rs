use bevy::{gizmos, input::keyboard, prelude::*};

//插件结构体
struct StartPlugins;

fn setup(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle::from_section("Text", TextStyle::default()));
}



//添加组件以及资源
impl Plugin for StartPlugins {
    /*
       为StartPlugins实现traint Plugin
    */
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, draw_example)
            .add_systems(Update, update_line_location);

    }
}

fn draw_example(mut gizmos: Gizmos, time: Res<Time>) {
    let sin = time.elapsed_seconds_wrapped().sin();
    gizmos.line_2d(Vec2::Y-sin, Vec2::splat(-80.), Color::RED);
}

fn update_line_location(mut config_store:ResMut<GizmoConfigStore>,keyboard:Res<ButtonInput<KeyCode>>, time: Res<Time>){
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    if keyboard.pressed(KeyCode::ArrowRight) {
        config.line_width += 5.;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        config.line_width -= 5.;
    }
}


fn main() {
    App::new().add_plugins((DefaultPlugins, StartPlugins)).run();
}
