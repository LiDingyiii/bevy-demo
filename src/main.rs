use bevy::{prelude::*, gizmos};

//插件结构体
struct StartPlugins;


fn setup(mut commands:Commands,assert_server:Res<AssetServer>){
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle::from_section("Text", TextStyle::default()));
}

//添加组件以及资源
impl Plugin for StartPlugins {
    /*
       为StartPlugins实现traint Plugin
    */
    fn build(&self,app:&mut App){
        app.add_systems(Startup, setup).add_systems(Update, draw_example);
    }
}

fn draw_example(mut gizmos:Gizmos,time:Res<Time>){
    gizmos.line_2d(Vec2::Y, Vec2::splat(-80.), Color::RED);
}

fn main() {
    App::new().add_plugins((DefaultPlugins, StartPlugins)).run();
}
