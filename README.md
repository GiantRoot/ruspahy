# ruspahy

Rust Smoothed Particle Hydrodynamics (SPH) 示例。

该仓库提供一个用 Rust 编写的极简 SPH 程序，
通过两个球体在圆柱容器中的压缩演示基本的流体/固体模拟流程。

## 快速开始

1. 安装 [Rust](https://www.rust-lang.org/) 工具链。
2. 在仓库根目录执行

   ```bash
   cargo run --release
   ```

3. 模拟过程中会在 `output/` 目录生成 `step_xxxx.vtk`，可在 Paraview 中加载查看。

## 代码结构

- `src/main.rs` — 主程序及全部核心函数。
- `Cargo.toml` — 依赖和版本说明。
- `output/` — 模拟生成的数据（已在 `.gitignore` 中排除）。

## 算法概述

1. 在球体内按固定间距初始化粒子。
2. 每个时间步计算密度和压力，依据核函数求得压力梯度产生的加速度。
3. 更新粒子速度与位置，并在桶壁和盖子处应用边界条件。
4. 输出 VTK 文件以便可视化。

详细函数说明见 [`DOCS.md`](DOCS.md)。

## 许可

本项目遵循 GNU GPL v3 许可。
