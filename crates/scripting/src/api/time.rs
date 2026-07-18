use crate::coroutine_manager::SharedCoroutineManager;
use crate::scheduler::SharedScheduler;
use danmaku::SharedDanmaku;
use mlua::prelude::*;

pub(crate) fn create_table(
    lua: &Lua,
    shared_danmaku: &SharedDanmaku,
    coroutine_manager: &SharedCoroutineManager,
    shared_scheduler: &SharedScheduler,
) -> Result<LuaTable, LuaError> {
    todo!()
}
