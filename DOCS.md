# 文档说明

本文档介绍 `ruspahy` 的核心算法和代码实现细节。

## 代码总览

所有逻辑均位于 `src/main.rs`：

- `Particle` 结构保存位置、速度、密度和压力【F:src/main.rs†L5-L11】。
- 常量定义平滑长度、参考密度等参数【F:src/main.rs†L13-L16】。
- `kernel` 与 `grad_kernel` 实现三次样条核函数及其梯度【F:src/main.rs†L18-L43】。
- `init_sphere` 在球体区域生成粒子分布【F:src/main.rs†L45-L70】。
- `apply_boundaries` 处理桶壁与盖子的碰撞【F:src/main.rs†L72-L98】。
- `write_vtk` 输出粒子状态为 VTK 文件【F:src/main.rs†L100-L132】。
- `main` 函数组织完整的模拟流程【F:src/main.rs†L134-L189】。

## 计算流程

1. **初始化**：通过 `init_sphere` 创建两个球状粒子群。
2. **密度与压力**：遍历其他粒子使用 `kernel` 累加密度，并由状态方程得到压力。
3. **受力与积分**：使用 `grad_kernel` 求压力梯度，进而更新粒子的速度和位置。
4. **边界处理**：`apply_boundaries` 限制粒子在圆柱容器内运动。
5. **数据输出**：每个时间步调用 `write_vtk` 生成可视化文件。

## 进一步改进

当前实现使用 \(O(N^2)\) 的全对遍历，适合少量粒子的演示。
未来可考虑引入 KD-tree 或均匀网格以加速邻域搜索，便于处理更大规模的场景。
