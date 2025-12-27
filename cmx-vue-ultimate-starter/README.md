# 🚀 Vue Ultimate Starter

<div align="center">

**开箱即用的 Vue 3 企业级项目脚手架**

集成最新技术栈与最佳实践，让您快速投入业务开发

[![TypeScript](https://img.shields.io/badge/TypeScript-5.9-blue)](https://www.typescriptlang.org/)
[![Vue](https://img.shields.io/badge/Vue-3.5-green)](https://vuejs.org/)
[![Vite](https://img.shields.io/badge/Vite-Latest-purple)](https://vitejs.dev/)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-orange)](https://tauri.app/)
[![License](https://img.shields.io/badge/License-MIT-yellow)](./LICENSE)

</div>

---

## ✨ 核心特性

### 🎯 前端框架
- ⚡ **Vue 3.5** - Composition API + `<script setup>`
- 🔥 **Vite (Rolldown)** - 极速构建工具
- 🏃 **Bun** - 超快的包管理器与运行时
- 📦 **TypeScript 5.9** - 完整类型支持

### 🎨 UI 与样式
- 🌈 **Tailwind CSS 4** - 原子化 CSS 框架
- 🎭 **Reka UI** - 无头组件库
- 🖌️ **Lucide Vue Next** - 优雅的图标库
- 🌗 **多主题切换** - 内置深色/浅色主题系统
- 📐 **响应式设计** - 移动端完美适配

### 🛠️ 工程化能力
- 🖥️ **Tauri 2.x** - 构建轻量、安全的桌面应用
- 🌐 **Vue I18n** - 完整的国际化方案
- 🗂️ **文件路由** - unplugin-vue-router 自动生成路由
- 📑 **布局系统** - vite-plugin-vue-layouts
- 🔌 **自动导入** - 组件、API、Composables 自动注册
- 💾 **状态管理** - Pinia + 持久化插件

### 📊 数据管理
- 🔄 **TanStack Query** - 强大的异步状态管理
- 📡 **Axios** - HTTP 请求库
- 🔐 **认证系统** - 完整的登录/注册流程
- 📝 **表单验证** - VeeValidate + Zod

### 🧪 质量保障
- ✅ **Vitest** - 单元测试框架
- 🎭 **Playwright** - E2E 测试解决方案
- 🔍 **ESLint** - 代码检查（@antfu/eslint-config）
- 💅 **Prettier** - 代码格式化
- 📏 **Oxlint** - 快速代码检查
- 🪝 **Husky + Lint-staged** - Git Hooks 自动化
- 📋 **Commitlint** - 规范化提交信息
- 🎨 **Commitizen** - 交互式提交工具

### 🎁 增强功能
- ✨ **自动动画** - @formkit/auto-animate
- ⏰ **日期处理** - Day.js
- 🎉 **粒子效果** - TSParticles
- 📊 **图表库** - Unovis + Vue Chrts
- 🍞 **消息提示** - Vue Sonner
- 📤 **进度条** - NProgress
- 🎪 **Markdown 支持** - unplugin-vue-markdown

---

## 📦 技术栈详情

| 类别 | 技术 | 版本 |
|------|------|------|
| **核心框架** | Vue | ^3.5.22 |
| **构建工具** | Vite (Rolldown) | Latest |
| **包管理器** | Bun | Latest |
| **类型系统** | TypeScript | ~5.9.2 |
| **桌面应用** | Tauri | ^2.x |
| **样式方案** | Tailwind CSS | ^4.1.12 |
| **状态管理** | Pinia | ^3.0.3 |
| **路由** | Vue Router | ^4.5.1 |
| **HTTP 客户端** | Axios | ^1.11.0 |
| **数据查询** | TanStack Query | ^5.85.5 |
| **表单验证** | VeeValidate + Zod | ^4.15.1 |
| **国际化** | Vue I18n | ^11.1.12 |
| **UI 组件** | Reka UI | ^2.5.1 |
| **图标库** | Lucide Vue Next | 0.545.0 |
| **测试框架** | Vitest + Playwright | Latest |

---

## 📁 项目结构

```
vue-ultimate-starter/
├── 📂 src/                         # 源代码目录
│   ├── 📂 assets/                  # 静态资源
│   │   ├── index.css              # 全局样式
│   │   ├── themes.css             # 主题样式
│   │   └── theme.json             # 主题配置
│   ├── 📂 components/              # 可复用组件
│   │   ├── 📂 ui/                  # 基础 UI 组件
│   │   ├── 📂 app-sidebar/         # 侧边栏组件
│   │   ├── 📂 data-table/          # 数据表格组件
│   │   ├── 📂 custom-theme/        # 主题定制组件
│   │   ├── 📂 marketing/           # 营销页组件
│   │   └── 📂 global-layout/       # 全局布局组件
│   ├── 📂 composables/             # 组合式函数
│   │   ├── use-auth.ts            # 认证逻辑
│   │   ├── use-axios.ts           # HTTP 请求
│   │   └── use-sidebar.ts         # 侧边栏逻辑
│   ├── 📂 constants/               # 常量定义
│   │   └── themes.ts              # 主题常量
│   ├── 📂 enums/                   # 枚举类型
│   │   └── global.ts              # 全局枚举
│   ├── 📂 layouts/                 # 布局模板
│   │   ├── default.vue            # 默认布局
│   │   ├── marketing.vue          # 营销页布局
│   │   └── blank.vue              # 空白布局
│   ├── 📂 pages/                   # 页面组件（自动路由）
│   │   ├── 📂 dashboard/           # 仪表盘
│   │   ├── 📂 auth/                # 认证页面
│   │   ├── 📂 settings/            # 设置页面
│   │   ├── 📂 tasks/               # 任务管理
│   │   ├── 📂 users/               # 用户管理
│   │   └── 📂 errors/              # 错误页面
│   ├── 📂 plugins/                 # Vue 插件
│   │   ├── 📂 i18n/                # 国际化配置
│   │   ├── 📂 pinia/               # 状态管理
│   │   ├── 📂 nprogress/           # 进度条
│   │   └── index.ts               # 插件入口
│   ├── 📂 router/                  # 路由配置
│   │   ├── 📂 guard/               # 路由守卫
│   │   ├── index.ts               # 路由实例
│   │   └── public-routes.ts       # 公开路由
│   ├── 📂 services/                # API 服务
│   │   └── api/                   # API 接口定义
│   ├── 📂 stores/                  # Pinia Stores
│   │   ├── auth.ts                # 认证状态
│   │   ├── theme.ts               # 主题状态
│   │   └── counter.ts             # 计数器示例
│   ├── 📂 types/                   # 类型定义
│   │   ├── auto-import.d.ts       # 自动导入类型
│   │   ├── typed-router.d.ts      # 路由类型
│   │   └── vue-router-meta.d.ts   # 路由元信息类型
│   ├── 📂 utils/                   # 工具函数
│   │   └── env.ts                 # 环境变量工具
│   ├── App.vue                    # 根组件
│   └── main.ts                    # 入口文件
├── 📂 src-tauri/                   # Tauri 桌面应用后端
│   ├── 📂 src/                     # Rust 源码
│   │   ├── main.rs                # Rust 入口
│   │   └── lib.rs                 # 库文件
│   ├── 📂 capabilities/            # Tauri 权限配置
│   ├── Cargo.toml                 # Rust 依赖配置
│   └── tauri.conf.json            # Tauri 配置文件
├── 📂 e2e/                         # E2E 测试
│   └── vue.spec.ts                # 测试用例
├── 📄 vite.config.ts               # Vite 配置
├── 📄 tsconfig.json                # TypeScript 配置
├── 📄 tailwind.config.js           # Tailwind 配置
├── 📄 eslint.config.ts             # ESLint 配置
├── 📄 playwright.config.ts         # Playwright 配置
├── 📄 vitest.config.ts             # Vitest 配置
├── 📄 commitlint.config.js         # Commitlint 配置
└── 📄 package.json                 # 项目依赖
```

---

## 🚀 快速开始

### 📌 环境准备

#### 1️⃣ 安装 Bun

Bun 是一个超快的 JavaScript 运行时和包管理器,比 npm/yarn/pnpm 更快。

**Windows:**
```powershell
powershell -c "irm bun.sh/install.ps1 | iex"
```

**Linux & macOS:**
```bash
curl -fsSL https://bun.sh/install | bash
```

验证安装:
```bash
bun --version
```

#### 2️⃣ 安装 Rust(可选,仅需构建桌面应用)

如果需要构建 Tauri 桌面应用,需要安装 Rust。

**访问:** [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

**Windows:**
- 下载并运行 `rustup-init.exe`

**Linux & macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

验证安装:
```bash
rustc --version
cargo --version
```

---

### 💾 安装依赖

```bash
# 克隆项目(或下载源码)
git clone <your-repository-url>
cd vue-ultimate-starter

# 安装所有依赖
bun install
```

---

## 💻 开发指南

### 🔥 启动开发服务器

```bash
bun dev
```

默认在 `http://localhost:1420` 运行,支持热模块替换 (HMR)。

### 🛠️ 类型检查

```bash
bun run type-check
```

### 📝 代码检查与修复

```bash
# 运行所有 linter(oxlint + eslint)
bun lint

# 仅运行 ESLint
bun lint:eslint

# 仅运行 Oxlint
bun lint:oxlint
```

### 💅 代码格式化

```bash
bun format
```

### 🏗️ 构建生产版本

```bash
# 完整构建(类型检查 + 打包)
bun run build

# 仅打包(跳过类型检查)
bun run build-only
```

构建产物将生成在 `dist/` 目录。

### 🔍 预览生产构建

```bash
bun preview
```

---

## 🖥️ Tauri 桌面应用

### 前置条件

确保已安装 **Rust** 和 **Tauri CLI**。

### 🛠️ 开发模式

```bash
bun tauri dev
```

这将:
1. 启动 Vite 开发服务器
2. 启动 Tauri 桌面应用窗口
3. 支持热更新,修改代码后自动刷新

### 🏭 构建桌面应用

```bash
bun tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`:

| 平台 | 输出格式 |
|------|----------|
| **Windows** | `.msi`, `.exe` |
| **macOS** | `.app`, `.dmg` |
| **Linux** | `.deb`, `.AppImage`, `.rpm` |

> **注意:** 跨平台构建需在对应的目标平台上执行,或使用 CI/CD 管道。

---

## 🧪 测试

### ✅ 单元测试 (Vitest)

```bash
# 运行所有单元测试
bun test:unit

# 监视模式
bun test:unit --watch

# 生成覆盖率报告
bun test:unit --coverage
```

### 🎭 E2E 测试 (Playwright)

#### 首次使用:安装浏览器

```bash
npx playwright install
```

#### 运行测试

```bash
# 构建项目(E2E 测试需要)
bun run build

# 运行所有 E2E 测试
bun test:e2e

# 仅在 Chromium 上测试
bun test:e2e --project=chromium

# 调试模式
bun test:e2e --debug

# 测试特定文件
bun test:e2e tests/example.spec.ts
```

---

## 📝 Git 提交规范

本项目使用 **Commitizen** + **Commitlint** 实现规范化提交。

### 🎉 使用 Commitizen 提交

```bash
# 交互式提交(推荐)
bun run commit

# 快速提交
bun run commit:quick "修复: 修复登录问题"

# 提交并推送到远程仓库
bun run commit-and-push
```

### 📋 提交信息格式

遵循 **Conventional Commits** 规范:

```
<类型>[(范围)]: <描述>

[详细描述]

[尾注]
```

**类型示例:**
- `feat`: 新功能
- `fix`: 修复 Bug
- `docs`: 文档更新
- `style`: 代码格式(不影响功能)
- `refactor`: 代码重构
- `perf`: 性能优化
- `test`: 测试相关
- `build`: 构建系统或外部依赖
- `ci`: CI 配置
- `chore`: 其他修改

**示例:**
```bash
feat(认证): 添加谷歌登录功能
fix(ui): 修复按钮在暗色主题下的颜色问题
docs(readme): 更新安装文档
```

### 🪝 自动化 Git Hooks

本项目配置了以下 Git Hooks:

- **pre-commit**: 自动运行 ESLint 和 Prettier
- **commit-msg**: 验证提交信息格式

---

## 🎨 主题与国际化

### 🌓 主题切换

项目内置多套主题,支持亮色/暗色模式切换:

- 使用 [`toggle-theme.vue`](src/components/toggle-theme.vue) 组件切换主题
- 主题数据定义在 [`constants/themes.ts`](src/constants/themes.ts)
- 主题状态由 [`stores/theme.ts`](src/stores/theme.ts) 管理

### 🌐 国际化 (i18n)

项目使用 **Vue I18n** 实现多语言支持:

- 语言文件位于 `src/plugins/i18n/locales/`
- 使用 [`language-change.vue`](src/components/language-change.vue) 组件切换语言
- 在代码中使用:
  ```vue
  <template>
    <p>{{ $t('welcome.message') }}</p>
  </template>
  
  <script setup>
  import { useI18n } from 'vue-i18n'
  const { t } = useI18n()
  </script>
  ```

---

## 📦 部署

### 🌐 Web 部署

1. 构建生产版本:
   ```bash
   bun run build
   ```

2. 部署 `dist/` 目录到:
   - **Vercel** / **Netlify**: 自动检测 Vite 项目
   - **Nginx** / **Apache**: 配置 SPA 路由支持
   - **GitHub Pages**: 使用 GitHub Actions

### 🖥️ 桌面应用部署

1. 构建桌面应用:
   ```bash
   bun tauri build
   ```

2. 产物位于 `src-tauri/target/release/bundle/`

3. 分发方式:
   - **Windows**: 发布 `.msi` 安装包
   - **macOS**: 发布 `.dmg` 镜像或提交到 App Store
   - **Linux**: 发布 `.deb` / `.AppImage`

> **提示:** 跨平台构建推荐使用 GitHub Actions 或其他 CI/CD 工具。

---

## 🔧 常见问题

### Q: 为什么使用 Bun 而不是 npm/yarn?
A: Bun 的安装和运行速度比传统工具快 10-100 倍,同时兼容 Node.js 生态。

### Q: Tauri vs Electron?
A: Tauri 使用系统 WebView,体积更小(~3MB vs ~100MB),性能更好,更安全。

### Q: 如何自定义主题?
A: 修改 `src/assets/themes.css` 和 `src/constants/themes.ts`,添加新的主题变量。

### Q: 生产构建失败怎么办?
A: 先运行 `bun run type-check` 检查类型错误,再运行 `bun lint` 检查代码规范。

---

## 🤝 贡献指南

欢迎贡献代码! 请遵循以下步骤:

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`bun run commit`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

请确保:
- ✅ 所有测试通过 (`bun test:unit`)
- ✅ 代码通过 Lint 检查 (`bun lint`)
- ✅ 遵循提交规范

---

## 📚 相关资源

- [Vue 3 文档](https://vuejs.org/)
- [Vite 文档](https://vitejs.dev/)
- [Tauri 文档](https://tauri.app/)
- [Tailwind CSS 文档](https://tailwindcss.com/)
- [Pinia 文档](https://pinia.vuejs.org/)
- [TanStack Query 文档](https://tanstack.com/query/latest)

---

## 📄 开源协议

MIT License © 2024

本项目基于 MIT 协议开源,您可以自由使用、修改和分发。

---

<div align="center">

**⭐ 如果这个项目对你有帮助,请给个 Star! ⭐**

 Made with ❤️ by Vue Ultimate Starter Team

</div>