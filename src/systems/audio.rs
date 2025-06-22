// mod components;

use bevy::prelude::*;
use crate::components::mymusic::*;

// 播放背景音乐
pub fn playgroundmusic(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("正在播放背景音乐");
    // 背景音乐
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/tiny-village.flac")),
        MyMusic,
    ));
}
// 暂停背景音乐
pub fn pausepgmusic() {
    println!("按‘space’已经暂停背景音乐");
}
// 静音背景音乐，静音而不暂停，在实现后续某些功能有用。
pub fn mutepgmusic() {
    println!("按‘M’背景音乐已静音");
}

// 播放事件音乐

// 播放音效

// 总音量控制


