# Sayooshi-iOS

《さよならを教えて／对你说再见》旧版 AVG32 中文资源的原生运行器兼容性分支，基于 [xmoezzz/siglus_rs](https://github.com/xmoezzz/siglus_rs)。包含 Windows 统一运行器、iOS 启动器，以及中文替换字库、历史记录和菜单字体修正。

**本仓库只提供引擎源码、测试和构建工具，不提供游戏本体、汉化资源、游戏字库、存档或 Apple 签名凭据。** 使用者需自行合法取得匹配的游戏和补丁。项目与游戏权利人、原汉化组无隶属或授权背书关系。

## 当前验证结果

- 静态检查：匹配旧版 AVG32 修订号 1714；本地资源 217 个 PACL 场景严格解析通过。
- Windows：标题、新游戏、中文正文、菜单主体、一次图形分支、历史记录、快进、手动存档及退出进程后读档推进通过；实际扬声器输出未确认。
- iPhone：使用者已确认中文、声音、轻点推进、手动存档，以及彻底关闭应用后读回原位置并继续推进。
- 尚未验证：完整游戏全篇、所有机型；手机端分支、历史、快进及全部视频场景未逐项验收。

真机验收包为 **1.0.1 (2)**，源码提交 `69462ec26829dca9e26b01fd76a8a6a3f6c8a5bf`。包内 arm64 和最低系统 14.0 已检查；**iOS 17 满足部署版本要求，但本轮实际测试系统为 iOS 27.0.1，并非 iOS 17 实机测试。** 完整证据边界见 [TESTING.md](TESTING.md)。

## 资源版本与编码

需要旧版 AVG32 数据：`GAMEEXE.INI` 含 `#REGNAME="CRAFT\ADIEU"`，`DAT/SEEN.TXT` 以 `PACL` 开头，配合 `DAT/FN.DAT`、`PDT`、音频和视频目录。保留原 `AVG3217D.EXE` 供修订号识别；原生运行器不执行它。

只有 `Scene.pck`、`Gameexe.dat` 的 Siglus 下载版不是本兼容性测试所用的数据，不能直接套用同一汉化。汉化包也不等于完整本体。

本次萌你妹 V1.01 实体展开资源使用 **SJIS 原始字形码位**；启动器选择 **Shift-JIS (Japanese)**。正文可显示中文，这个选项不是语言开关。保留自备补丁的 `FN.DAT`；配置文件需正确解码后无损保存为 UTF-8，并使用可选的 `#TITLE_NLS="gbk"` 处理章节/存档标题。不要把所有内容统一按 GBK 解码。

少数流传补丁的 PACL 场景尾部截断；本地验收使用与合法原版逐条对照恢复后的工作副本。**本仓库不附恢复后的游戏脚本，也不保证任意网络资源包可直接启动。** 对截断资源应报错并核对原件，不能盲目补零或跳过未知指令。

## 构建和使用

见 [BUILDING.md](BUILDING.md)。Windows 的 AVG32 入口是：

```powershell
cargo build --locked --release -p game_launcher --bin game_launcher_run
./target/release/game_launcher_run.exe --probe "D:\Games\Sayooshi-CHS"
./target/release/game_launcher_run.exe "D:\Games\Sayooshi-CHS" --nls sjis
```

预发布的 `siglus.exe` 是 Siglus 专用入口，不应拿它来判断 AVG32 是否兼容。

iOS 使用从本仓库构建的 **unsigned IPA**，通过 [Sideloadly](https://sideloadly.io/) 在自己的电脑上签名。账号、密码和验证码只在本机界面输入。将完整工作目录放入“文件 → 我的 iPhone → Siglus → siglus → Sayooshi-CHS”，刷新启动器并选择 SJIS。也可通过启动器 Import 导入已解压的文件夹。

- 轻点：推进；双指轻点：菜单/返回；双指竖向滑动：历史滚动。
- 手机存档：游戏目录中的 `SAVE.INI`。备份前退出游戏，并保留同目录其他新增存档文件。
- 续签使用相同 Apple ID 和应用标识覆盖安装；不要先删除应用。

## 修改与上游

上游固定提交：`5f358ef1360b0439e92aed2b749e8f25d8721954`。此分支最初从官方源码归档导入，不声称保留完整上游 Git 历史。修改范围、构建来源见 [SAYOOSHI_BUILD.md](SAYOOSHI_BUILD.md)，版本变化见 [CHANGELOG.md](CHANGELOG.md)。原上游说明保留为 [README.upstream.md](README.upstream.md)，其中截图链接可能不适用于本精简分支。

## 许可证

引擎及本分支修改按 [MPL-2.0](LICENSE-MPL-2.0) 提供。保留原作者及第三方版权声明；字体等资产按各自许可提供，见 [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)。游戏名称仅用于说明兼容目标，不授予游戏内容的分发权。

分发 IPA 时同时提供对应提交源码、许可声明和摘要；不能把本仓库根目录的许可解释为对游戏/汉化资源的授权。
