// assembler 模块属于指令编排层，负责把不同车辆的 M/L/R 指令拆成 Action 序列。
#[allow(clippy::module_inception)]
pub(crate) mod assembler;
pub(crate) mod bus_state;
pub(crate) mod sports_car_state;
pub(crate) mod state;
