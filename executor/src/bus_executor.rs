use crate::{Executor, Pose};

// BusExecutor 表示巴士指令执行器入口，先复用普通车 Executor 的对外接口。
pub struct BusExecutor;

impl BusExecutor {
    // 巴士构造函数，允许调用者指定车辆初始位置和朝向。
    pub fn with_pose(pose: Pose) -> Executor {
        Executor::with_pose(pose)
    }

    // 巴士执行器的构造函数，内部复用 with_pose() 创建指定初始状态的 Executor。
    #[allow(clippy::new_ret_no_self)]
    pub fn new(pose: Pose) -> Executor {
        Self::with_pose(pose)
    }
}
