# alga2 — 设计路线图

## 1. 愿景

一个现代、维护中、零依赖的 Rust 抽象代数层级——作为 [alga](https://docs.rs/alga)（自 2020-03 起不再维护，通过其依赖方仍每季度约 160 万次下载）的继任者。实现矩阵由 [batch-impl](https://docs.rs/batch-impl) 从少量 DSL 行生成——这个 crate 既是产品，也是 batch-impl 的旗舰展示。

## 2. 战略目标

| 维度     | 3 个月（v0.2）                              | 12 个月（v1.0）                                                      |
|----------|---------------------------------------------|----------------------------------------------------------------------|
| 产品     | 发布 v0.2，零依赖 + no_std，README 展示横幅 | v1.0：alga 0.9 能力超集，被 ≥3 个 crate 采用                         |
| 迁移市场 | alga API 差异表 + 迁移兼容层设计            | ≥1 个活跃的 alga 依赖方（优先 nalgebra 0.35 / sprs）已迁移或进入讨论 |
| 下载量   | 自身 5k+                                    | 自身 100k+；batch-impl 随之突破 50k                                  |
| 推广     | 博客文章 + r/rust                           | TWiR ≥1；batch-impl README 增加真实案例章节                          |
| 质量     | 覆盖完整数值矩阵的定律 proptest             | 为下游类型导出 `laws`；MSRV CI；零 unsafe                            |

## 3. 里程碑

| #                         | 范围                                                         | 验收标准                                       | 状态 |
|---------------------------|--------------------------------------------------------------|------------------------------------------------|------|
| **M1 设计冻结（1–2 周）** | 层级 trait 签名、运算符系统、命名、alga API 差异表           | 关闭本文档 §5 的待决问题                       | ✅ 完成（差异表见 `docs/ALGA-DIFF.md`） |
| **M2 v0.1（3–5 周）**     | 核心塔 + 数值矩阵 + 定律骨架 + README 横幅                   | 已发布；定律测试通过；记录与 alga 的编译期对比 | ✅ 代码完成（未发布） |
| **M3 v0.2（6–8 周）**     | 元组/Option/Vec/Complex 矩阵 + no_std 打磨 + num-traits 桥接 | 已发布；5k 下载量                              | ✅ 代码完成（num-traits 桥接搁置：零依赖优先） |
| **M4 v0.3（9–12 周）**    | Module/VectorSpace + nalgebra/sprs 推广 + 博客/TWiR          | ≥1 次迁移对话落地                              | 🚧 Module/VectorSpace ✅；推广/迁移未开始 |
| **M5 持续进行**           | 维护 + batch-impl 自用反馈 + 文档                            | 季度指标表滚动更新                             | ✅ 进行中（已向 batch-impl 反馈并落地两个特性：where 值位置 `@N`、开放范围 `@1..`） |

> 当前实现（2026-08）：**280 impls** 全部由 batch-impl 生成——15 种基础
> 类型（`@num` + `bool`/F₂）的全阶梯 + module 层级、元组 1–4（含模块/
> 向量空间，变长段 + repeat 块生成）、`Option`、`Complex<T>`、Vec/String/
> Box（alloc）、HashMap/HashSet（std）。35 个定律测试全绿；`no_std` 裸核心
> 256 impls。batch-impl 依赖暂为本地 path（两个未发布特性），发布后切回
> crates.io。

## 4. 已记录的设计决策

- **层级范围（v0.1）**：加法阶梯 `Magma → Semigroup → Monoid →
  Group → AbelianGroup`；乘法阶梯 `Semiring → Ring →
  CommutativeRing → Field`。**设计上排除**：Quasigroup、Loop、Band、
  Lattice（alga 过度设计；仅列入待办）。
- **运算符系统**：一个类型在 `+` 和 `*` 下都是 `Monoid`，因此
  trait 通过运算符标记进行参数化（`Monoid<Additive<T>>`），
  并提供符合人体工学的别名（`AdditiveMonoid<T>`）。刻意避免 alga 的
  `AbstractMagma` 风格命名。
- **定律是差异化优势**：`src/laws` 导出 proptest 策略，使下游用户
  能针对层级定律测试其自定义类型——alga 从未提供过这一点。
- **no_std 姿态**：裸核心（完整塔 + 数值/元组矩阵）为 `no_std`，
  且不依赖 libm；`alloc`（Vec/String/Box）和 `std`
  （HashMap/HashSet）通过 cargo feature 分层启用；`default = ["std"]`。
  `proptest` 仅限 std，通过 feature gate 供下游使用。
- **零依赖核心**：batch-impl 仅在构建时使用；生成的
  impl 不引入任何运行时依赖。
- **batch-impl 角色（展示品）**：每个层级 × 类型族一个
  `#[batch_impl(...)]` 块，在指令体中设置每个类型的单位元（`0`/`1`/`0.0`/
  `1.0`）——这是非泛型的按类型差异，泛型 impl 无法表达。新类型只需
  添加一行即可加入矩阵。
- **依赖策略**：manifest 中使用 `batch-impl = "0"`（crates.io）；若要
  针对树内开发版本测试，临时切换为
  `batch-impl = { path = "../macro-test" }`（绝不提交）。

## 5. 待决问题（在 M1 中关闭）

1. 每个层级的精确 trait 签名（方法名、`PartialEq` 超 trait、
   `Output` 风格关联类型 vs GAT）。
2. 运算符标记形态：`Additive<T>`/`Multiplicative<T>` newtype vs
   标记类型；`Ring` 如何将两者结合。
3. 对照 alga 进行命名审计（公开 API 差异表是 v0.1 的交付物）。
4. `Complex` 策略（crate 内迷你类型 vs num-complex 桥接 feature）。
5. 定律测试人体工学：每个层级的 blanket `law` 函数、策略复用。

## 6. 测试策略

- **每个 impl 的定律**：每个生成的 impl × 其层级定律（结合律、
  单位元、逆元、分配律）在 CI 中通过 proptest 运行。
- **no_std 回归**：CI 中运行 `cargo check --no-default-features` 和
  `--features alloc`（参见 `test-no-std` 任务）。
- **内部门槛**：fmt、clippy `--all-targets -- -D warnings`、MSRV 1.93。

## 7. 失败模式与回滚

| 失败                         | 信号                              | 回滚                                                             |
|------------------------------|-----------------------------------|------------------------------------------------------------------|
| 社区拒绝 API 设计            | v0.1 发布后 4 周无外部 issue/采用 | 收敛到 alga 兼容层（相同 trait 名 + 迁移 shim），仅服务迁移市场  |
| 层级过度设计拖延发布         | M2 延期                           | v0.1 范围在此冻结；其余全部进入待办                              |
| 生成期间发现 batch-impl 缺口 | DSL 无法表达按类型常量体          | macro_rules 回退 + 提交 batch-impl issue（自用反馈本身就是收获） |
| 定律测试发现我们自己的 bug   | 测试失败                          | 修复它们——这验证了定律层                                         |
| nalgebra/sprs 拒绝迁移       | 两轮讨论无进展                    | 转向新项目（泛型数学 / 密码学 / ML）；alga2 独立存在             |

## 8. 发布检查清单（每个版本）

1. `batch-impl` 切换到 crates.io 版本（已是默认）。
2. CHANGELOG 条目；刷新 README 横幅计数。
3. `cargo package` 验证；`cargo publish`。
4. `git tag vX.Y.Z`；GitHub 发布。
5. 更新 §2 中的指标行。