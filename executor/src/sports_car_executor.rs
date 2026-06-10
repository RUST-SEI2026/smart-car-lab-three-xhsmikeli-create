use crate::{Executor, Pose, SportsCarState};

// SportsCarExecutor 表示跑车指令执行器入口，通过注入 SportsCarState 复用 Executor 的执行流程。
pub struct SportsCarExecutor;

impl SportsCarExecutor {
    // 跑车构造函数，允许调用者指定车辆初始位置和朝向。
    pub fn with_pose(pose: Pose) -> Executor {
        Executor::with_state(pose, Box::new(SportsCarState::default())) // 这里调用跑车的 State，创建跑车执行器。
    }

    // 跑车执行器的构造函数，内部复用 with_pose() 创建指定初始状态的 Executor。
    #[allow(clippy::new_ret_no_self)]
    pub fn new(pose: Pose) -> Executor {
        Self::with_pose(pose)
    }
}
