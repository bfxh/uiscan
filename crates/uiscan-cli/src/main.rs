//! `uiscan` —— 把"屏幕上真的出现了哪些文案"抓出来。
//!
//! 当前只接了 TUI 面（`tui-feed`）。Web 面解析层已就位但 **CDP 实跑还没接**，
//! GUI 面（UIA）已在本机验证可编译可运行、适配器未写：两者都不做假命令，
//! 传了就报"未接"并退 2，宁可不出结果也不给一个看着像成功的空输出。

use std::io::Read;
use std::process::exit;

const USAGE: &str =
    "用法：uiscan tui-feed [--file <捕获文件，缺省读 stdin>] [--rows N=24] [--cols N=80]";

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(cmd) = args.next() else {
        eprintln!("{USAGE}");
        exit(2);
    };
    if cmd != "tui-feed" {
        eprintln!("未接的面：{cmd}（Web 的 CDP 通道与 GUI 的 UIA 适配器还没写）\n{USAGE}");
        exit(2);
    }

    let mut file: Option<String> = None;
    let mut rows: u16 = 24;
    let mut cols: u16 = 80;
    while let Some(a) = args.next() {
        let mut take = |name: &str| -> Option<String> {
            args.next()
                .ok_or_else(|| {
                    eprintln!("{name} 后面缺值");
                    exit(2);
                })
                .ok()
        };
        match a.as_str() {
            "--file" => file = take("--file"),
            "--rows" => {
                rows = take("--rows")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or_else(|| {
                        eprintln!("--rows 要正整数");
                        exit(2);
                    })
            }
            "--cols" => {
                cols = take("--cols")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or_else(|| {
                        eprintln!("--cols 要正整数");
                        exit(2);
                    })
            }
            other => {
                eprintln!("不认得的参数：{other}\n{USAGE}");
                exit(2);
            }
        }
    }

    let bytes = match file {
        Some(p) => std::fs::read(&p).unwrap_or_else(|e| {
            eprintln!("读不了 {p}：{e}");
            exit(2);
        }),
        None => {
            let mut buf = Vec::new();
            std::io::stdin().read_to_end(&mut buf).unwrap_or_else(|e| {
                eprintln!("读 stdin 失败：{e}");
                exit(2);
            });
            buf
        }
    };

    let h = uiscan_tui::harvest(&bytes, rows, cols);
    println!("surface={}", h.surface);
    for it in h.candidates() {
        println!("{}  {:?}  [{:?}]", it.loc, it.text, it.channel);
    }
    let cps = h.rendered_codepoints();
    println!("非 ASCII 码点 {} 个：{:?}", cps.len(), cps);
}
