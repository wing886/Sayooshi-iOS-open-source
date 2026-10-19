# 署名、致谢与来源 / Credits

## 主要上游：xmoezzz 与 siglus_rs 贡献者

本项目直接基于 **[xmoezzz/siglus_rs](https://github.com/xmoezzz/siglus_rs)** 的源码。Rust 引擎、AVG32 运行模块、跨平台基础设施、统一启动器及 iOS 应用的大部分实现来自上游作者和贡献者。本仓库是针对特定 AVG32 中文资源的兼容性分支，原引擎不由本仓库维护者原创。

- 上游作者/维护者：[xmoezzz](https://github.com/xmoezzz)。其他作者见[上游贡献记录](https://github.com/xmoezzz/siglus_rs/graphs/contributors)及各文件的原始声明。
- 使用基线：[5f358ef1360b0439e92aed2b749e8f25d8721954](https://github.com/xmoezzz/siglus_rs/commit/5f358ef1360b0439e92aed2b749e8f25d8721954)。
- 许可：MPL-2.0；原版权声明和许可文件保留。
- 本仓库从源码归档导入并建立干净历史；GitHub 上本仓库的提交/贡献统计不代表原始代码作者归属。

## 参考实现：xkanon / xkanon_fork

AVG32 行为核对参考了 **[weimingtom/xkanon_fork](https://github.com/weimingtom/xkanon_fork)**，使用提交 [9def18eadba51db9c12eb60904a72f0f7ce89948](https://github.com/weimingtom/xkanon_fork/commit/9def18eadba51db9c12eb60904a72f0f7ce89948)。

- xkanon 原作者：**Kazunori Ueno (JAGARL)**，依据该项目 `xkanon/README` 和源码版权声明。
- fork 维护者：**[weimingtom](https://github.com/weimingtom)** 及该项目贡献者。
- 此处是参考来源致谢；本源码包不收录该参考仓库。引用链接不改变 xkanon 自身许可，也不把其作品重新许可为 MPL。

## 本分支的工作范围

[wing886](https://github.com/wing886) 维护本兼容性分支；本次适配、代码检查和文档整理使用了 AI 编程辅助。分支新增工作集中于：

1. 汉化替换字库的原始 SJIS 码位保留；
2. 小说模式历史记录字形与相关控制指令；
3. 章节/存档标题独立编码配置；
4. 中文菜单字体与按字符缺字回退；
5. 相应回归测试、构建来源验证及 Windows/iPhone 验收记录。

这些增量修正不构成对整个原引擎的原创声明。具体变化和验收边界分别见 [CHANGELOG.md](CHANGELOG.md)、[TESTING.md](TESTING.md)。

## 字体、游戏与中文翻译

- Noto Sans CJK：Adobe、Noto 项目与其贡献者，SIL OFL 1.1；原版权和保留字体名称声明见 [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)。
- 兼容性测试所用中文翻译资源标为“萌你妹 V1.01”，感谢原汉化组的翻译工作；[版本来源说明](https://www.moyu.moe/resource/5953)是归档页面，不把镜像站视为翻译作者。译文和游戏字库不随本仓库分发。
- 《さよならを教えて》游戏及其美术、音频、剧本等内容归原权利人所有；本仓库未获得游戏内容的再分发授权。

感谢上述作者与贡献者。引用和兼容性说明不表示上游或游戏权利人认可、支持或维护本分支。
