# Fooocus 配置管理器

一个用于管理 Fooocus 配置的桌面应用程序，基于 Tauri + Vue 3 + TypeScript 构建。

## 功能特性

- 配置的创建、编辑、删除
- 配置导入/导出（Fooocus JSON 格式）
- 标签管理
- 收藏功能
- 搜索和筛选
- 本地 SQLite 数据库存储

## 技术栈

- **前端**: Vue 3 + TypeScript + Pinia + Naive UI + Tailwind CSS
- **后端**: Tauri 2.x + Rust
- **数据库**: SQLite (rusqlite)

## 数据存储

应用数据存储在本地 SQLite 数据库中：

- **数据库文件**: `fooocus_config.db`
- **Windows 位置**: `C:\Users\<用户名>\AppData\Roaming\com.xuan.fooocus-config-manager\`

## 开发

### 环境要求

- Node.js 18+
- Rust 1.70+
- pnpm/npm/yarn

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建发布

```bash
npm run tauri build
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
