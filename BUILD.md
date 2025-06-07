# 构建说明

本项目提供了多种方式来自动化构建和部署WASM模块到webapp目录。

## 方式一：使用PowerShell脚本（推荐Windows用户）

```powershell
# 在项目根目录运行
.\build.ps1
```

## 方式二：使用Bash脚本（推荐Linux/macOS用户）

```bash
# 在项目根目录运行
./build.sh
```

## 方式三：使用Makefile（跨平台）

```bash
# 构建WASM并复制到webapp
make build

# 构建并启动开发服务器
make dev

# 安装webapp依赖
make install

# 清理构建文件
make clean

# 查看帮助
make help
```

## 手动构建（如果自动化脚本不工作）

1. 构建WASM包：
```bash
wasm-pack build --target web --out-dir pkg
```

2. 创建webapp目录（如果不存在）：
```bash
mkdir -p webapp/src/wasm
```

3. 复制文件：
```bash
cp pkg/xcutils_wasm.js webapp/src/wasm/
cp pkg/xcutils_wasm_bg.wasm webapp/src/wasm/
cp pkg/xcutils_wasm.d.ts webapp/src/wasm/
cp pkg/package.json webapp/src/wasm/
```

4. 启动开发服务器：
```bash
cd webapp
npm run dev
```

## 依赖要求

- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) - 用于构建WASM包
- Node.js 和 npm - 用于运行webapp
- Make（可选）- 如果使用Makefile方式

## 故障排除

如果遇到权限问题，请确保脚本有执行权限：

```bash
# Linux/macOS
chmod +x build.sh

# Windows PowerShell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```