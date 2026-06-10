use executor::{Pose, SportsCarExecutor as Executor};

#[test]
fn should_return_x_plus_2_given_command_is_m_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("M");

    let expected_pose = Pose::new(2, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_2_given_command_is_bm_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("BM");

    let expected_pose = Pose::new(-2, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_4_given_command_is_fm_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("FM");

    let expected_pose = Pose::new(4, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_4_given_command_is_fbm_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("FBM");

    let expected_pose = Pose::new(-4, 0, 'E');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_y_plus_1_and_facing_n_given_command_is_l_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("L");

    let expected_pose = Pose::new(0, 1, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_y_plus_1_and_facing_s_given_command_is_bl_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("BL");

    let expected_pose = Pose::new(0, 1, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_1_y_plus_1_and_facing_n_given_command_is_fl_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("FL");

    let expected_pose = Pose::new(1, 1, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_1_y_plus_1_and_facing_s_given_command_is_fbl_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("FBL");

    let expected_pose = Pose::new(-1, 1, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_y_minus_1_and_facing_s_given_command_is_r_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("R");

    let expected_pose = Pose::new(0, -1, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_y_minus_1_and_facing_n_given_command_is_br_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("BR");

    let expected_pose = Pose::new(0, -1, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_1_y_minus_1_and_facing_s_given_command_is_fr_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("FR");

    let expected_pose = Pose::new(1, -1, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_1_y_minus_1_and_facing_n_given_command_is_fbr_and_facing_is_e() {
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    executor.execute("FBR");

    let expected_pose = Pose::new(-1, -1, 'N');
    assert_eq!(expected_pose, executor.query());
}
