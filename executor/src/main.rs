use std::io::{self, Write};

use executor::{BusExecutor, Executor, Pose, SportsCarExecutor};

// main() 提供一个简单的命令行入口，用来手动选择车辆类型并执行命令。
fn main() {
    println!("Executor 初始状态为 (0, 0, N)。");
    println!("请输入车辆类型：");
    println!("1: 普通车");
    println!("2: 跑车");
    println!("3: 巴士");

    let mut executor = select_executor();

    println!("请输入命令序列，可包含 M、L、R、B、F，或输入 Q 退出程序。");
    println!("M: 移动，L: 左转，R: 右转，B: 切换倒车状态，F: 切换加速状态。");
    print_pose(&executor);

    // 循环读取用户输入，直到用户输入 Q 退出程序。
    loop {
        print!("> ");
        if let Err(err) = io::stdout().flush() {
            eprintln!("刷新输出失败: {err}");
            break;
        }

        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("读取输入失败: {err}");
            break;
        }

        // trim() 去掉换行等空白字符，to_uppercase() 允许用户输入小写命令。
        let cmd = input.trim().to_uppercase();
        if cmd.is_empty() {
            continue;
        }

        if cmd == "Q" {
            println!("程序已退出。");
            break;
        }

        // 检查命令是否仅包含 M、L、R、B、F。
        if !cmd
            .chars()
            .all(|ch| matches!(ch, 'M' | 'L' | 'R' | 'B' | 'F'))
        {
            println!("无效命令: {cmd}。请输入仅包含 M、L、R、B、F 的命令序列，或输入 Q 退出程序。");
            continue;
        }

        // 执行整串命令，并在每次执行后展示当前位置和朝向。
        executor.execute(&cmd);
        print_pose(&executor);
    }
}

// select_executor() 用来选择普通车、跑车或巴士，并返回对应执行器。
fn select_executor() -> Executor {
    loop {
        print!("vehicle> ");
        if let Err(err) = io::stdout().flush() {
            eprintln!("刷新输出失败: {err}");
            return Executor::with_pose(Pose::new(0, 0, 'N'));
        }

        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("读取输入失败: {err}");
            return Executor::with_pose(Pose::new(0, 0, 'N'));
        }

        match input.trim() {
            "1" => return Executor::with_pose(Pose::new(0, 0, 'N')),
            "2" => {
                return SportsCarExecutor::with_pose(Pose::new(0, 0, 'N'));
            }
            "3" => return BusExecutor::with_pose(Pose::new(0, 0, 'N')),
            _ => println!("请输入 1、2、3。"),
        }
    }
}

// 查询当前 Pose 状态并打印。
fn print_pose(executor: &Executor) {
    let pose = executor.query();
    println!("当前位置: ({}, {}, {})", pose.x, pose.y, pose.heading);
}
