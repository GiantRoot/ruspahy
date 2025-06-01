# 项目文档

本文档介绍 `ruspahy` 项目的代码结构、主要函数以及涉及的数学原理。

## 总览

该项目实现了一个基于 **光滑粒子流体动力学 (Smoothed Particle Hydrodynamics, SPH)** 的三维粒子模拟框架。粒子系统通过邻域搜索构建局部交互关系，随后使用 SPH 核函数计算密度、压力及相互作用力，并采用显式 Euler 方法更新运动状态。最终结果可以导出为 VTK 文件进行可视化。

目录结构大致如下：

```
src/          主要源码实现
assets/       默认配置文件及示例
output/       保存模拟过程中产生的 VTK 文件
```

下面将按模块说明每个文件及其关键函数的作用。

## 核心算法流程

典型的模拟过程可以用如下伪代码表示：

```text
load_config()
init_particle_system()
while time < end_time {
    build_neighbor_list()
    compute_density_pressure()
    compute_forces()
    integrate()
    write_vtk()  # 每个时间步均输出 VTK 文件

其中 `build_neighbor_list` 负责邻域搜索，`compute_forces` 调用 `sph_kernel` 计算压力、
粘性以及材料界面带来的作用力，`integrate` 则使用显式 Euler 更新粒子状态。

## 模块关系概览

```
Config -> Main -> ParticleSystem
             |       |-- Neighbor
             |       |-- Force --+-- SPH Kernel
             |       |           \-- Material
             |       \-- Integrator
             \-- Output
```

主程序(`main.rs`)加载配置后创建 `ParticleSystem`，粒子系统依次调用 `neighbor` 构建邻域，
通过 `force` 和 `sph_kernel` 计算物理量，在 `integrator` 中更新粒子位置，最终由 `output` 写出结果。

## src/config.rs

- **作用**：负责从 `TOML` 格式的配置文件读取模拟参数。
- **核心结构和函数**：
  - `SphereConfig`：描述初始球体分布【F:src/config.rs†L11-L20】。
  - `SimConfig`：保存网格尺寸、时间步长、材料等信息【F:src/config.rs†L22-L36】。
  - `load_config`：读取文件并通过 `serde` 反序列化为 `SimConfig`【F:src/config.rs†L39-L42】。

## src/material.rs

- **作用**：定义材料及界面（材料之间的连接）属性。
- **主要内容**：
  - `MaterialType` 枚举用于区分弹性、弹塑性等材料模型【F:src/material.rs†L8-L16】。
  - `Material` 结构存储密度、杨氏模量及屈服应力等属性【F:src/material.rs†L19-L36】。
  - `Interface` 结构描述两种材料之间的界面类型及结合强度【F:src/material.rs†L48-L57】。

## src/particle.rs

- **作用**：定义粒子 (`Particle`) 与粒子系统 (`ParticleSystem`)，并提供构建与计算相关的成员函数。
- **关键点**：
  - `Particle` 结构保存位置、速度、力以及压力、应力等物理量【F:src/particle.rs†L1-L11】。
  - `ParticleSystem::new` 根据配置生成粒子分布，可在规则网格或球体中布置粒子【F:src/particle.rs†L25-L65】。
  - `build_neighbor_list` 调用邻域模块以平均粒子间距为半径构建邻接表【F:src/particle.rs†L94-L101】。
  - `compute_forces` 创建 SPH 核函数后计算密度、压力和外力【F:src/particle.rs†L103-L111】。
  - `find_interface` 用于查询两种材料之间的界面属性【F:src/particle.rs†L114-L124】。

## src/neighbor.rs

- **作用**：实现邻域搜索算法，避免全对遍历带来的高复杂度。
- **算法原理**：使用均匀网格 (hash grid) 将粒子划分到立方体单元。对每个粒子，仅在其所在单元及相邻 26 个单元中搜索可能的邻居，计算平方距离判断是否在半径内。
- **关键函数**：
  - `build_neighbor_list` 根据给定半径返回每个粒子的邻居索引列表【F:src/neighbor.rs†L13-L57】。

## src/sph_kernel.rs

- **作用**：提供常见的 SPH 平滑核函数及其系数。
- **数学背景**：
  - **Poly6 核**：用于密度估计，形式为
    $$
    W_{\text{poly6}}(r,h) = \frac{315}{64\pi h^9}(h^2 - r^2)^3.
    $$
  - **Spiky 核梯度**：用于压力项，公式
    $$
    \nabla W_{\text{spiky}}(r,h) = -\frac{45}{\pi h^6}(h-r)^2 \frac{\mathbf{r}}{r}.
    $$
  - **Viscosity 核拉普拉斯**：用于粘性力计算，表达式
    $$
    \nabla^2 W_{\text{visc}}(r,h) = \frac{45}{\pi h^6}(h - r).
    $$
- **主要接口**：
  - `SPHKernel::new` 根据平滑长度预计算常数【F:src/sph_kernel.rs†L12-L26】。
  - `w_poly6`、`grad_w_spiky` 和 `lap_w_viscosity` 分别实现上述核函数【F:src/sph_kernel.rs†L28-L55】。

## src/force.rs

- **作用**：根据 SPH 理论计算粒子的密度、压力、粘性及界面作用力。
- **实现细节**：
  - `compute_density_pressure` 首先利用
    $$
    \rho_i = \sum_j m_j W_{ij}, \qquad
    p_i = k(\rho_i - \rho_0)
    $$
    对邻域粒子求和得到密度与压力【F:src/force.rs†L11-L29】。
  - `compute_forces` 综合压力梯度、粘性项和界面粘结力求得总受力，其中
    $$
    \mathbf{f}_i = -\sum_j m_j\Bigl(\tfrac{p_i}{\rho_i^2}+\tfrac{p_j}{\rho_j^2}\Bigr)\nabla W_{ij}
    +\mu \sum_j m_j \tfrac{\mathbf{v}_j-\mathbf{v}_i}{\rho_j}\nabla^2 W_{ij}
    +\mathbf{f}_{\text{interface}}.
    $$
    【F:src/force.rs†L32-L97】。
  - `compute_stress` 根据压力值计算等效应力并与屈服强度比较，常用公式为
    $$
    \sigma_{\text{eq}} = \sqrt{\tfrac{3}{2}\, \mathbf{s}:\mathbf{s}},
    $$
    超出屈服后按材料模型进行修正【F:src/force.rs†L100-L113】。

## src/integrator.rs

- **作用**：执行时间积分更新粒子运动。
- **数学原理**：显式 Euler 积分，对每个粒子的运动方程 \( \frac{d\mathbf{v}}{dt} = \frac{\mathbf{f}}{\rho} \) 进行离散化。
- **函数**：`integrate` 逐粒子更新速度和位置【F:src/integrator.rs†L8-L16】。

时间离散形式可写为
$$
\mathbf{v}_i^{n+1} = \mathbf{v}_i^n + \Delta t\,\frac{\mathbf{f}_i}{\rho_i},\qquad
\mathbf{x}_i^{n+1} = \mathbf{x}_i^n + \Delta t\,\mathbf{v}_i^{n+1}.
$$

## src/main.rs

- **作用**：程序入口，同时定义 `write_vtk` 函数用以输出 VTK 文件。每个时间步都会调用该函数生成 `output/step_XXXX.vtk`，可在 Paraview 中观察模拟过程。

## 数学原理概述

本项目基于 SPH 方法求解固体动力学问题，其关键思想为：

1. **核函数逼近**：任意物理量 \( A(\mathbf{r}) \) 可写成周围粒子的加权和 \( A(\mathbf{r}) \approx \sum_j m_j \frac{A_j}{\rho_j} W(|\mathbf{r}-\mathbf{r}_j|, h) \)。文中的 `w_poly6`、`grad_w_spiky` 和 `lap_w_viscosity` 分别提供密度估计、压力梯度和粘性项所需的核函数。
2. **动量方程离散化**：粒子受力由压力梯度、粘性和界面粘结等项组成，其离散形式可写为
   $$
   \frac{d\mathbf{v}_i}{dt} = \frac{\mathbf{f}_i}{\rho_i}.
   $$
   计算完成后通过 `integrate` 更新速度与位置。
3. **邻域搜索**：为提高效率，采用均匀网格在局部范围内查找相互作用的粒子，避免 \(O(N^2)\) 级别的全对计算。

## 结语

以上概述了各个源码文件及函数在 SPH 框架中的作用，并简要解释了背后的数学公式与实现思路。结合源代码及配置文件，可根据自身需求扩展更多材料模型或数值方法。

