# 可重复构建

## Windows

需要 Rust 1.99.1（本次验证版本）、Visual Studio 2022 C++ 桌面构建工具和 Windows SDK；依赖需要时使用 CMake。建议在 Developer PowerShell 中执行。构建产物和缓存位置可通过 `CARGO_TARGET_DIR`、`CARGO_HOME`、`RUSTUP_HOME` 自行指定。

```powershell
cargo test --locked -p avg32 --lib
cargo build --locked --release -p game_launcher --bin game_launcher_run
./target/release/game_launcher_run.exe --probe "D:\Games\Sayooshi-CHS"
./target/release/game_launcher_run.exe "D:\Games\Sayooshi-CHS" --nls sjis
```

AVG32 专用入口也可使用：

```powershell
cargo run --locked -p avg32 --release --features desktop-player --bin avg32_player -- "D:\Games\Sayooshi-CHS" --nls sjis
```

不要直接运行原游戏 EXE 来验证本引擎修正。不要把游戏目录放进源码仓库。

## iOS / macOS

需要完整 Xcode、Rust 1.99.1、XcodeGen。本项目从 Rust `game_launcher` 构建统一运行器，包含 AVG32 模块；不是下载上游 IPA 再打包。

```sh
rustup toolchain install 1.99.1 --profile minimal
rustup default 1.99.1
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
brew install xcodegen
cargo test --locked -p avg32 --lib
python3 tools/ci/provenance.py
bash platform/scripts/build_ios_xcframework.sh
bash platform/scripts/package_ios_altstore_ipa.sh
python3 tools/ci/verify_ipa.py dist/ios/SiglusLauncher.ipa --commit "$(git rev-parse HEAD)"
```

输出 `dist/ios/SiglusLauncher.ipa`。脚本构建设备 arm64 及模拟器静态库，设备 IPA 不带开发者签名。项目最低部署系统 14.0；这不等于所有 14.0 以上系统均经过实机测试。

## GitHub Actions

工作流 [ios.yml](.github/workflows/ios.yml) 仅手动触发。Actions → Build fixed AVG32 unsigned iOS IPA → Run workflow。使用标准 `macos-15`，先做源码审计和 AVG32 单元测试，再构建、核验架构/提交来源并上传产物。构建号使用 GitHub run number，因此后续包未必仍是构建 2。

产物 `SiglusLauncher-fixed-unsigned` 含 IPA、SHA256SUMS、verification.json、source-provenance.json，保留 3 天。下载后自行备份。发布长期 Release 前应核对源码提交、摘要及许可附件。

执行前检查自己的 Actions 免费额度和计费设置。工作流不需要 Apple ID、Apple 证书或额外令牌；不要为构建上传游戏或启用付费 runner。

## 发布检查

```sh
python3 tools/ci/audit_index.py
python3 tools/ci/audit_publication.py
```

第一个脚本检查实际 Git 暂存区；第二个检查所有本地引用可达的历史对象与提交元数据。自动扫描只是辅助，还要人工检查新增资产来源。不要把本地游戏验收截图、设备日志、存档、签名后的 IPA 或个人配置提交到仓库。

## 当前发布准备边界

现有私有仓库历史含上游来源未单独说明的 shader.cfx 样本，不建议直接改为 Public。开源发布副本应从当前源码导出，排除该样本且不携带旧 .git，再建立新历史。源码可按保留的 MPL/OFL 等条款发布；新的公开二进制 Release 还需收集实际链接依赖的许可文本。
