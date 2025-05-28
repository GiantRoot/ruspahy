# ruspahy

Short for **Rust Smoothed Particle Hydrodynamics**.

使用Rust语言编写的SPH(光滑粒子流体动力学)模拟原型。该项目目前处于
早期阶段，主要用于实验和学习目的。


## 使用方法

1. 安装 [Rust](https://www.rust-lang.org/) 工具链。
2. 在项目目录下运行 `cargo run --release` 启动模拟。
3. 配置文件位于 `assets/config.toml`，可调整粒子间距、时间步长等参数。
   当配置文件中包含 `[[spheres]]` 条目时，程序将根据给定的中心、
   半径和初速度生成多个球体，可用于模拟颗粒碰撞等场景。

生成的 VTK 文件保存在 `output/` 目录，可用 Paraview 等软件查看。

## 项目结构

```
src/            源码实现
assets/         默认配置文件
output/         模拟输出（已在 .gitignore 中排除）
```

主要模块包括 `particle` (粒子与系统定义)、`force` (力计算)、
`integrator` (时间积分)、`output` (结果输出) 以及 `sph_kernel`
 (核心核函数实现)。
新增的 `material` 模块用于描述多种材料及其界面属性，核心计算模块现已使用
`rayon` 进行并行加速。
最近的更新进一步扩展了材料定义，可在配置文件中为弹塑性材料指定屈服应力、
强化模量以及损伤阈值，以便后续实现更复杂的本构行为。
界面定义现在可通过 `bond_strength` 数值来描述连接强度，
从弱到强连续可调，避免仅局限于三种离散类型。
粒子结构新增 `plastic_strain` 与 `damage` 字段，分别记录累积塑性应变
和损伤程度，输出文件中也会给出相应结果。

## TODO

- 完善材料本构模型的实现
- 提供更易用的配置项和示例
- 增加单元测试，保证核心模块的正确性
