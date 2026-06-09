use crate::Action;

// Assembler 表示指令编排接口，负责把 M/L/R 指令转换成一组动作 Vec<Action>。
// 第一阶段先让普通车 State 实现该接口，后续新增车辆时复用相同接口扩展差异逻辑。
pub(crate) trait Assembler {
    // 根据当前车辆状态，把控制指令 M/L/R 编排成原子动作序列。
    fn assemble(&self, cmd: char) -> Vec<Action> {
        match cmd {
            'M' => self.move_assemble(),
            'L' => self.turn_left_assemble(),
            'R' => self.turn_right_assemble(),
            _ => Vec::new(),
        }
    }

    // 编排 M 指令。
    fn move_assemble(&self) -> Vec<Action>;

    // 编排 L 指令。
    fn turn_left_assemble(&self) -> Vec<Action>;

    // 编排 R 指令。
    fn turn_right_assemble(&self) -> Vec<Action>;

    // 切换倒车状态。
    fn be_reverse(&mut self);

    // 切换加速状态。
    fn be_fast(&mut self);
}
