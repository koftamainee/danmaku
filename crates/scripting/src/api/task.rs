use crate::coroutine_manager::SharedCoroutineManager;
use danmaku::SharedDanmaku;
use mlua::prelude::*;

pub(crate) fn create_table(
    lua: &Lua,
    danmaku: &SharedDanmaku,
    coroutine_manager: &SharedCoroutineManager,
) -> Result<LuaTable, LuaError> {
    todo!()
}
