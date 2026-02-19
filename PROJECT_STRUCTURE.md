# Fooocus 配置管理器 - 项目结构

```
fooocus-config-manager/
├── MyFooocus.md                    # 项目需求文档
├── PROJECT_STRUCTURE.md            # 项目结构说明（本文件）
│
└── fooocus-config-manager/         # 主项目目录
    ├── src/                        # 前端源代码 (Vue 3 + TypeScript)
    │   ├── components/             # Vue 组件
    │   │   ├── MainLayout.vue      # 主布局组件（侧边栏 + 内容区）
    │   │   ├── ModelCard.vue       # 模型卡片组件
    │   │   ├── ModelEditor.vue     # 模型编辑器弹窗
    │   │   ├── ModelManagement.vue # 模型管理页面
    │   │   ├── PresetCard.vue      # 配置卡片组件
    │   │   └── PresetEditor.vue    # 配置编辑器弹窗
    │   │
    │   ├── stores/                 # Pinia 状态管理
    │   │   ├── modelStore.ts       # 模型状态管理
    │   │   └── presetStore.ts      # 配置状态管理
    │   │
    │   ├── types/                  # TypeScript 类型定义
    │   │   └── index.ts            # 所有类型定义导出
    │   │
    │   ├── utils/                  # 工具函数
    │   │   └── presetConverter.ts  # Fooocus 配置解析/导出
    │   │
    │   ├── assets/                 # 静态资源
    │   │   └── vue.svg
    │   │
    │   ├── App.vue                 # 根组件
    │   ├── main.ts                 # 应用入口
    │   ├── style.css               # 全局样式
    │   └── vite-env.d.ts           # Vite 类型声明
    │
    ├── src-tauri/                  # 后端源代码 (Rust + Tauri)
    │   ├── src/
    │   │   ├── commands.rs         # Tauri 命令定义
    │   │   ├── database.rs         # 数据库模型和初始化
    │   │   ├── lib.rs              # 库入口，注册命令
    │   │   └── main.rs             # 程序入口
    │   │
    │   ├── capabilities/           # Tauri 权限配置
    │   │   └── default.json
    │   │
    │   ├── icons/                  # 应用图标（各平台）
    │   │
    │   ├── Cargo.toml              # Rust 依赖配置
    │   ├── Cargo.lock
    │   ├── build.rs                # 构建脚本
    │   └── tauri.conf.json         # Tauri 配置文件
    │
    ├── public/                     # 公共静态资源
    │   ├── tauri.svg
    │   └── vite.svg
    │
    ├── .vscode/                    # VS Code 配置
    │   └── extensions.json
    │
    ├── index.html                  # HTML 入口
    ├── package.json                # npm 依赖配置
    ├── package-lock.json
    ├── tsconfig.json               # TypeScript 配置
    ├── tsconfig.node.json
    ├── vite.config.ts              # Vite 配置
    ├── tailwind.config.js          # Tailwind CSS 配置
    ├── postcss.config.js           # PostCSS 配置
    ├── .gitignore
    └── README.md
```

## 核心模块说明

### 前端架构

| 目录/文件 | 职责 |
|-----------|------|
| `components/` | UI 组件，按功能划分 |
| `stores/` | Pinia 状态管理，封装 Tauri 命令调用 |
| `types/` | TypeScript 接口定义，与后端结构对应 |
| `utils/` | 工具函数，如配置文件解析 |

### 后端架构

| 文件 | 职责 |
|------|------|
| `database.rs` | SQLite 数据库表结构、Rust 数据模型 |
| `commands.rs` | Tauri IPC 命令，前后端通信桥梁 |
| `lib.rs` | 命令注册、应用初始化 |

## 数据流

```
Vue 组件 → Pinia Store → Tauri invoke() → Rust commands → SQLite
    ↑                                                        │
    └────────────────── 返回数据 ←───────────────────────────┘
```

## 技术栈

- **前端**: Vue 3 + TypeScript + Pinia + Naive UI + Tailwind CSS
- **后端**: Rust + Tauri 2.x + SQLite (rusqlite)
- **构建**: Vite + vue-tsc
