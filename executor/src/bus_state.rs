use crate::{Action, Assembler};

// BusState 记录巴士当前的两个运动标志，是否为倒车状态，是否为加速状态。
// - is_reverse: 倒车状态
// - is_fast: 加速状态
// 巴士车身较长，M 指令与普通车一致，L/R 指令需要先移动再转向。
#[derive(Default, Copy, Clone)]
pub(crate) struct BusState {
    is_reverse: bool,
    is_fast: bool,
}

impl BusState {
    // 巴士执行 M/L/R 时都会先移动。
    // 加速状态下移动两格，普通状态下移动一格。
    fn move_prefix(&self) -> Vec<Action> {
        let mut actions = Vec::new();

        actions.push(Action::Forward(self.direction()));

        if self.is_fast {
            actions.push(Action::Forward(self.direction()));
        }

        actions
    }

    // 根据倒车状态，确定 L 指令对应的实际转向动作。
    fn turn_left_action(&self) -> Action {
        if self.is_reverse {
            Action::TurnRight
        } else {
            Action::TurnLeft
        }
    }

    // 根据倒车状态，确定 R 指令对应的实际转向动作。
    fn turn_right_action(&self) -> Action {
        if self.is_reverse {
            Action::TurnLeft
        } else {
            Action::TurnRight
        }
    }

    // 当前移动方向。
    //  1. 前进状态：1
    //  2. 倒车状态：-1
    fn direction(&self) -> i32 {
        if self.is_reverse {
            -1
        } else {
            1
        }
    }
}

impl Assembler for BusState {
    // 编排巴士 M 指令，共四种分别为：
    // 1. 普通状态：Forward(1)
    // 2. 倒车状态：Forward(-1)
    // 3. 加速状态：Forward(1), Forward(1)
    // 4. 倒车加速：Forward(-1), Forward(-1)
    fn move_assemble(&self) -> Vec<Action> {
        self.move_prefix()
    }

    // 编排巴士 L 指令，共四种分别为：
    // 1. 普通状态：Forward(1), TurnLeft
    // 2. 倒车状态：Forward(-1), TurnRight
    // 3. 加速状态：Forward(1), Forward(1), TurnLeft
    // 4. 倒车加速：Forward(-1), Forward(-1), TurnRight
    fn turn_left_assemble(&self) -> Vec<Action> {
        let mut actions = self.move_prefix();
        actions.push(self.turn_left_action());
        actions
    }

    // 编排巴士 R 指令，共四种分别为：
    // 1. 普通状态：Forward(1), TurnRight
    // 2. 倒车状态：Forward(-1), TurnLeft
    // 3. 加速状态：Forward(1), Forward(1), TurnRight
    // 4. 倒车加速：Forward(-1), Forward(-1), TurnLeft
    fn turn_right_assemble(&self) -> Vec<Action> {
        let mut actions = self.move_prefix();
        actions.push(self.turn_right_action());
        actions
    }

    // 切换倒车状态。
    // 第一次 B 标志从 false 反转为 true，进入倒车状态；第二次 B 标志从 true 反转为 false，恢复正常状态。
    fn be_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    // 切换加速状态。
    // 第一次 F 标志从 false 反转为 true，进入加速状态；第二次 F 标志从 true 反转为 false，恢复正常状态。
    fn be_fast(&mut self) {
        self.is_fast = !self.is_fast;
    }
}
