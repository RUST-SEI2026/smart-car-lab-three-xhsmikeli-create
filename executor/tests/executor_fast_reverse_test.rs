use executor::{Executor, Pose};

// 验证 F 和 B 状态叠加后的 M/L/R 行为。
#[test]
fn should_return_x_minus_2_given_command_is_fbm_and_facing_is_e() {
    // 初始位置在原点，车辆朝东方向。
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // 先执行 F 和 B 进入倒车加速状态，再执行 M。
    executor.execute("FBM");

    // 倒车加速状态下 M 表示后退两格，因此 x 坐标应减 2。
    let expected_pose = Pose::new(-2, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_s_and_x_minus_1_given_command_is_fbl_and_facing_is_e() {
    // 初始位置在原点，车辆朝东方向。
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // 先执行 F 和 B 进入倒车加速状态，再执行 L。
    executor.execute("FBL");

    // 倒车加速状态下 L 表示先后退一格，再右转。
    let expected_pose = Pose::new(-1, 0, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_n_and_x_minus_1_given_command_is_fbr_and_facing_is_e() {
    // 初始位置在原点，车辆朝东方向。
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // 先执行 F 和 B 进入倒车加速状态，再执行 R。
    executor.execute("FBR");

    // 倒车加速状态下 R 表示先后退一格，再左转。
    let expected_pose = Pose::new(-1, 0, 'N');
    assert_eq!(expected_pose, executor.query());
}
