## todo 列表
重要🔴🔴-紧急🟡🟡
略微重要🔴-略微紧急🟡
不重要🟢-不紧急✅
现实阻碍❌


 - 捋清原代码最合理的照抄路径，将各个模块需要涉及的bevy源码做到心中有数。🔴🟡
 - 将武器词条中的enun枚举结构声明在应该在的文件中🟡🟢
 - 越使用豆包，越发现它的局限性，它并不能把js代码完美转换到bevy的rust和ecs逻辑。所以需要自己细化这种转换。🔴🔴🟡
 - 逐个文件总结原js代码的层次结构，归纳总结出逻辑。🔴🔴🟡🟡
 - 正在缕清原文件的engine.js文件，作为main.rs的逻辑基础。🔴🔴🟡
 - 查询bevy里实现存档的方法，实现默认存储游戏状态的功能，并实现“重新开始”的功能，作为模块存起来常用。目前暂时没有实现的例子可供参考，目前考虑事件驱动的本地保存方案。🔴🟡❌
 - js代码可以实现动态文本替换，但是bevy我还没发现这个功能的例子，只能自己探索了。这个功能做好之后还是挺常驻的。有两种方案，小游戏可以选择一键切换，大游戏通过文件加载翻译。另外还需要注意利用 Bevy 的 ECS 将翻译逻辑与 UI 分离，实现松耦合。🔴✅
 - 游戏暂停与继续功能🔴🔴🟡
 - 游戏初始菜单，我甚至觉得不应该有，因为《a dark room》就没有，我也不喜欢有初始菜单了，太老土。所以最好做进游戏内部的某处角落。理论上应该有"重新开始"、“选择语言”、“加载存档/开始”、“音量”、“难度”等。🔴✅
 - 正在捋audio.js的逻辑，在bevy中实现。不过得先温习0.16版本的audio例子。🔴🔴🟡🟡
 - 温习0.16的bevy的ui例子🔴🔴🟡🟡