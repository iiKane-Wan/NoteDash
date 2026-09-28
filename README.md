# NoteDash

一款面向 Windows 的轻量级灵感速记工具。左侧管理记录列表，右侧即时编辑 Markdown，专注于「把灵感立刻记下来」这一件事。

## 功能特性

**核心记录**
- 左右分栏布局（左 1/3 记录列表，右 2/3 编辑区）
- 点击左侧标题，右侧同步加载内容
- 新建、删除（二次确认）记录
- 按标题或正文关键词实时搜索
- 列表按最后修改时间倒序排列

**Markdown 编辑**
- 基于 Vditor 的即时渲染（所见即所得）模式，输入语法立刻呈现格式
- 工具栏支持标题、加粗、列表、引用、代码、表格等常用格式
- 大纲面板：一键提取全文标题，点击跳转并自动关闭
- 内容修改后 500ms 防抖自动保存

**设置中心**
| 设置项 | 说明 |
| --- | --- |
| 开机自启 | 登录 Windows 后自动启动，基于注册表当前用户级，无需管理员权限 |
| 全局快捷键 | 任意程序中按下即可唤起主窗口，支持自定义、重置、清除，冲突有明确提示 |
| 隐藏主界面快捷键 | 焦点在软件内时按下隐藏窗口，默认 `Escape`，可自定义 |
| 关闭窗口后保留托盘 | 关闭按钮隐藏到托盘，托盘菜单可打开主窗口或退出程序 |
| 主题 | 浅色 / 深色 / 跟随系统，切换后全部界面与编辑器实时刷新 |

**其他**
- 单实例运行：重复启动不会创建新进程，而是激活已有窗口并提示「应用已在运行，已为你打开主窗口」，进程崩溃后实例锁自动释放
- 系统托盘常驻，支持左键唤起窗口
- 数据本地存储于 SQLite，随用户隔离

## 技术栈

| 层 | 技术 |
| --- | --- |
| 桌面框架 | Tauri 2.x |
| 后端 | Rust（rusqlite / serde / chrono / thiserror） |
| 前端 | Vue 3 + TypeScript + Vite |
| Markdown | Vditor（ir 即时渲染模式，资源本地化离线可用） |
| 数据库 | SQLite（WAL 模式） |
| 插件 | tauri-plugin-autostart、tauri-plugin-global-shortcut、tauri-plugin-single-instance |

## 目录结构

```
NoteDash/
├─ index.html / vite.config.ts / tsconfig.json / package.json
├─ NoteDash.png                 # 图标源图（1024×1024）
├─ scripts/sync-vditor.mjs      # 同步 Vditor 离线资源到 public/vditor
├─ public/vditor/               # Vditor 运行时资源（构建时生成，不入库）
├─ src/                         # 前端
│  ├─ main.ts / App.vue
│  ├─ api/                      # Tauri 命令封装（notes.ts / settings.ts）
│  ├─ components/               # 界面组件
│  │  ├─ NoteList.vue           # 左侧记录列表
│  │  ├─ SearchBar.vue          # 关键词搜索
│  │  ├─ MarkdownEditor.vue     # 标题输入 + Vditor 编辑器
│  │  ├─ OutlinePanel.vue       # 大纲浮层
│  │  ├─ SettingsPanel.vue      # 设置面板
│  │  ├─ ShortcutInput.vue      # 快捷键录制框
│  │  └─ ToastHost.vue          # 轻量提示
│  ├─ composables/              # 状态与逻辑复用
│  │  ├─ useNotes.ts            # 记录状态与防抖保存
│  │  ├─ useSettings.ts         # 设置读取与乐观更新
│  │  ├─ useTheme.ts            # 主题解析与应用
│  │  ├─ useHideShortcut.ts     # 隐藏主界面快捷键监听
│  │  ├─ useModalLayer.ts       # 模态层计数
│  │  └─ useToast.ts            # 提示消息
│  ├─ utils/                    # 时间、大纲提取、快捷键解析
│  └─ styles/global.css         # 主题变量（浅色/深色）
└─ src-tauri/                   # 后端
   ├─ Cargo.toml / build.rs / tauri.conf.json
   ├─ capabilities/default.json
   ├─ icons/                    # 应用图标（npx tauri icon 生成）
   └─ src/
      ├─ main.rs / lib.rs       # 入口、插件注册、窗口事件
      ├─ db.rs                  # 数据库定位与建表
      ├─ settings.rs            # 设置结构与读写
      ├─ models.rs / error.rs   # 数据模型与错误类型
      ├─ commands.rs            # 记录 CRUD、设置、窗口控制命令
      └─ tray.rs                # 系统托盘
```

## 环境要求

| 依赖 | 说明 |
| --- | --- |
| Node.js ≥ 18 | 前端构建 |
| Rust 工具链 | 建议 stable，目标 `x86_64-pc-windows-msvc` |
| Visual Studio 2022 生成工具 | 必须勾选「使用 C++ 的桌面开发」工作负载，提供 MSVC 链接器与 Windows SDK |
| WebView2 Runtime | Windows 10/11 通常已内置 |

安装 MSVC 工具链（可选，使用 winget）：

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools `
  --override "--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

## 快速开始

```powershell
# 1. 安装前端依赖（会自动同步 Vditor 离线资源）
npm install

# 2. 启动桌面开发模式（热更新）
npm run tauri dev
```

## 打包 Windows 安装包

```powershell
npm run tauri build
```

产物位置：

| 产物 | 路径 |
| --- | --- |
| NSIS 安装包 | `src-tauri\target\release\bundle\nsis\NoteDash_0.1.0_x64-setup.exe` |
| 免安装可执行文件 | `src-tauri\target\release\notedash.exe` |

安装包为当前用户安装模式（`currentUser`），安装时无需管理员权限，内置简体中文与英文界面。

## 可用脚本

| 命令 | 作用 |
| --- | --- |
| `npm run dev` | 仅启动前端开发服务器（端口 1420） |
| `npm run build` | 类型检查 + 前端生产构建 |
| `npm run tauri dev` | 启动完整桌面应用（开发模式） |
| `npm run tauri build` | 打包为 Windows 安装包与可执行文件 |
| `npm run sync:vditor` | 手动同步 Vditor 离线资源 |

## 快捷键

| 快捷键 | 类型 | 说明 |
| --- | --- | --- |
| `Ctrl+Shift+N` | 全局 | 唤起居中的主窗口（可在设置中修改） |
| `Escape` | 应用内 | 隐藏主界面（可在设置中修改）；面板打开时优先关闭面板 |

隐藏窗口后，可通过全局快捷键或托盘菜单「打开主窗口」恢复。

## 数据存储

数据库文件位于系统应用数据目录：

```
%APPDATA%\com.notedash.desktop\notedash.db
```

卸载应用不会删除该文件，重新安装后记录与设置仍然保留。备份时复制该目录即可。

## 图标定制

替换根目录的 `NoteDash.png`（建议 1024×1024、32 位带透明通道）后执行：

```powershell
npx tauri icon "NoteDash.png"

# 若发现 exe 图标未更新，清理 Tauri 构建缓存后重新打包
cargo clean -p notedash --manifest-path src-tauri/Cargo.toml
Remove-Item "src-tauri\target\release\build\notedash-*" -Recurse -Force
npm run tauri build
```

## 许可

本项目仅用于个人学习与使用，请遵循所依赖开源组件的各自许可协议。
