// executor 模块属于接口层，负责提供普通车、跑车和巴士的对外执行器入口。
pub(crate) mod bus_executor;
#[allow(clippy::module_inception)]
pub(crate) mod executor;
pub(crate) mod sports_car_executor;
