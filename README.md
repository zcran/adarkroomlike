# adarkroomlike

git add --all; git commit -m "normal update"; git push -u origin main



组织结构：
adarkroom-bevy/
├── Cargo.toml               # 项目依赖和元数据
├── src/                     # 源代码目录
│   ├── main.rs              # 程序入口点
│   ├── app_state.rs         # 应用状态定义
│   ├── resources/           # 全局资源
│   │   ├── game_state.rs    # 游戏状态
│   │   ├── world_config.rs  # 世界配置
│   │   └── audio_manager.rs # 音频管理器
│   ├── components/          # 游戏组件
│   │   ├── player.rs        # 玩家组件
│   │   ├── tile.rs          # 地图瓦片组件
│   │   ├── item.rs          # 物品组件
│   │   └── building.rs      # 建筑组件
│   ├── systems/             # 游戏系统
│   │   ├── movement.rs      # 移动系统
│   │   ├── combat.rs        # 战斗系统
│   │   ├── ui.rs            # UI系统
│   │   ├── world_gen.rs     # 世界生成系统
│   │   └── resource_management.rs # 资源管理系统
│   ├── scenes/              # 游戏场景
│   │   ├── main_menu.rs     # 主菜单场景
│   │   ├── village.rs       # 村庄场景
│   │   ├── outside.rs       # 外部场景
│   │   ├── ship.rs          # 飞船场景
│   │   └── space.rs         # 太空场景
│   ├── utils/               # 工具函数
│   │   ├── math.rs          # 数学工具
│   │   └── serialization.rs # 序列化工具
│   └── events/              # 游戏事件
│       ├── combat_event.rs  # 战斗事件
│       └── resource_event.rs # 资源事件
└── assets/                  # 游戏资源
    ├── images/              # 图像资源
    ├── sounds/              # 音频资源
    └── fonts/               # 字体资源