# uiscan

把"**屏幕上真的出现了哪些文案**"抓出来，用来查静态扫描查不到的那类缺陷：硬编码/漏翻、
截断、重叠、以及**缺字画成方框（豆腐块）**。Rust 写的，判据全是纯函数 + 真机语料。

## 为什么需要它（一句话）
静态 i18n 扫描器看的是源码，可及性树看的是"作者声明的逻辑字符串"，两边都不能告诉你
**用户实际看见了什么**。本项目的主判据就是这两者的差集。

## 三条已实测的表面事实（本仓判据的地基，`tests/` 里逐条钉住）
| 事实 | 实测口径 | 后果 |
|---|---|---|
| `::before`/`::after` 的 computed content **不带引号**，`::marker` 的**带引号** | Chromium 150（Qoder 内置 webview）真跑 `getComputedStyle` | 剥引号逻辑不能只按规范写，两边不对称 |
| `counter(i) ". "` 回来是**未解析令牌**；`attr()`/`var()` 回来是**已解析值** | 同上 | 只有还带函数括号的才算噪音，`counter` 必须丢 |
| CSS `::placeholder{content:…}` 实测被忽略（回来 `normal`） | 同上 | 占位文案只走 `placeholder` **属性**通道 |
| `display:none` 宿主的伪元素 content 照样给值 | 同上 | 不过滤几何就会报"用户看不见的文案" |
| `→ ← ✓ ✗` 在终端里判**单宽**，中文/全角判双宽 | vt100 0.16.2 真跑 | 宽字符续格必须跳过；符号也要进覆盖检查 |
| UIA 顶层窗口里有 `(-9999,-9999)` 屏外哨兵与 `(0,0)` 零尺寸 | `windows 0.62.2` on **x86_64-pc-windows-gnu** 真跑 | 矩形不可信的一律不进结果 |

## 两条口径分开的门（不是冗余）
- `Harvest::candidates()` —— 漏翻比对用：可见 **且** 像文案（有字母、非生成器令牌）。
- `Harvest::rendered_codepoints()` —— 豆腐块/字体覆盖用：**所有可见**条目，含纯符号。
  这条分开是测试逼出来的：拿候选集查覆盖会漏掉 `✓ ✗ →` 整类缺字。

## 现状（不含糊）
- ✅ `uiscan-core`：模型 + 判据（9 条测试）
- ✅ `uiscan-tui`：vt100 采集（5 条测试 + CLI 端到端跑通，21 个非 ASCII 码点与探针对得上）
- 🟡 `uiscan-web`：采集脚本 + 解析层就位，语料是真跑回来的 JSON（5 条测试）；**CDP 实跑还没接**。
  本机没有 `msedgedriver`，但 Edge 155 的 `--headless=new --remote-debugging-port` 已验通
  （`/json/version` 返 `Edg/155.0.4283.45`）⇒ 走 CDP 直连，不走 WebDriver。
  ⚠ `HARVEST_JS` **整段还没在浏览器里跑过**（里面每个 API 单独实测过），接上后第一件事是拿它跑
  `tests/fixtures/i18n-pseudo.html` 和 `pseudo-chromium.json` 逐条对账。
- ⬜ `uiscan-gui`（UIA）：适配器未写，编译链接可行性已在本机证过。
- ⬜ 截断/重叠判据：需要**有尺寸的窗口**。本次采集环境的 `innerWidth` 实测为 0 ⇒ 几何不可信，
  判据此刻正确地什么都不产出（宁可不报）。

## 跑
```
cargo run --bin uiscan -- tui-feed --file <终端字节捕获> --rows 24 --cols 80
cargo test --workspace
```
未知子命令、未接的面、缺参数一律 **退 2 并点名**，不返回空结果假装成功。
