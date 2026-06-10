use executor::{Pose, SportsCarExecutor as Executor};

// 验证跑车 M 指令在普通、加速、倒车、倒车加速四种状态下的移动行为。
mod move_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_2_given_command_is_m_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次跑车移动指令 M。
        executor.execute("M");

        // 跑车普通状态下 M 表示前进两格，因此 x 坐标应加 2。
        let expected_pose = Pose::new(2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_4_given_command_is_fm_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 F 进入加速状态，再执行 M。
        executor.execute("FM");

        // 跑车加速状态下 M 表示前进四格，因此 x 坐标应加 4。
        let expected_pose = Pose::new(4, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_2_given_command_is_bm_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 B 进入倒车状态，再执行 M。
        executor.execute("BM");

        // 跑车倒车状态下 M 表示后退两格，因此 x 坐标应减 2。
        let expected_pose = Pose::new(-2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_4_given_command_is_fbm_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 F 和 B 进入倒车加速状态，再执行 M。
        executor.execute("FBM");

        // 跑车倒车加速状态下 M 表示后退四格，因此 x 坐标应减 4。
        let expected_pose = Pose::new(-4, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }
}

// 验证跑车 L 指令在普通、加速、倒车、倒车加速四种状态下的行为。
mod turn_left_tests {
    use super::*;

    #[test]
    fn should_return_y_plus_1_and_facing_n_given_command_is_l_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次跑车左转指令 L。
        executor.execute("L");

        // 跑车普通状态下 L 表示先左转，再前进一格。
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_y_plus_1_and_facing_s_given_command_is_bl_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 B 进入倒车状态，再执行 L。
        executor.execute("BL");

        // 跑车倒车状态下 L 表示右转后再后退一格。
        let expected_pose = Pose::new(0, 1, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_y_plus_1_and_facing_n_given_command_is_fl_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 F 进入加速状态，再执行 L。
        executor.execute("FL");

        // 跑车加速状态下 L 表示先前进一格，左转后再前进一格。
        let expected_pose = Pose::new(1, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_y_plus_1_and_facing_s_given_command_is_fbl_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 F 和 B 进入倒车加速状态，再执行 L。
        executor.execute("FBL");

        // 跑车倒车加速状态下 L 表示先后退一格，右转后再后退一格。
        let expected_pose = Pose::new(-1, 1, 'S');
        assert_eq!(expected_pose, executor.query());
    }
}

// 验证跑车 R 指令在普通、加速、倒车、倒车加速四种状态下的行为。
mod turn_right_tests {
    use super::*;

    #[test]
    fn should_return_y_minus_1_and_facing_s_given_command_is_r_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 执行一次跑车右转指令 R。
        executor.execute("R");

        // 跑车普通状态下 R 表示先右转，再前进一格。
        let expected_pose = Pose::new(0, -1, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_y_minus_1_and_facing_n_given_command_is_br_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 B 进入倒车状态，再执行 R。
        executor.execute("BR");

        // 跑车倒车状态下 R 表示左转后再后退一格。
        let expected_pose = Pose::new(0, -1, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_y_minus_1_and_facing_s_given_command_is_fr_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 F 进入加速状态，再执行 R。
        executor.execute("FR");

        // 跑车加速状态下 R 表示先前进一格，右转后再前进一格。
        let expected_pose = Pose::new(1, -1, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_y_minus_1_and_facing_n_given_command_is_fbr_and_facing_is_e() {
        // 初始位置在原点，车辆朝东方向。
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        // 先执行 F 和 B 进入倒车加速状态，再执行 R。
        executor.execute("FBR");

        // 跑车倒车加速状态下 R 表示先后退一格，左转后再后退一格。
        let expected_pose = Pose::new(-1, -1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
}
