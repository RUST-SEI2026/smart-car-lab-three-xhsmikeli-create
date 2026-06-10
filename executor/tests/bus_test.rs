use executor::{BusExecutor as Executor, Pose};

// 验证巴士 M 指令在普通、加速、倒车、倒车加速四种状态下的移动行为。
mod move_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_1_given_command_is_m_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次巴士移动指令 M。
        executor.execute("M");

        // 巴士普通状态下 M 表示前进一格，因此 x 坐标应加 1。
        let expected_pose = Pose::new(1, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_2_given_command_is_fm_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 F 进入加速状态，再执行 M。
        executor.execute("FM");

        // 巴士加速状态下 M 表示前进两格，因此 x 坐标应加 2。
        let expected_pose = Pose::new(2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_given_command_is_bm_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 B 进入倒车状态，再执行 M。
        executor.execute("BM");

        // 巴士倒车状态下 M 表示后退一格，因此 x 坐标应减 1。
        let expected_pose = Pose::new(-1, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_2_given_command_is_fbm_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 F 和 B 进入倒车加速状态，再执行 M。
        executor.execute("FBM");

        // 巴士倒车加速状态下 M 表示后退两格，因此 x 坐标应减 2。
        let expected_pose = Pose::new(-2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }
}
