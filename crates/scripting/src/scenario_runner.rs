use crate::coroutine_manager::{CoroutineManager, SharedCoroutineManager};
use crate::scheduler::{Scheduler, SharedScheduler};
use danmaku::SharedDanmaku;
use mlua::prelude::{LuaError, LuaTable};
use mlua::{Lua, LuaOptions, StdLib};

pub struct ScenarioRunner {
    lua: Lua,
    danmaku: SharedDanmaku,
    danmaku_table: LuaTable,

    scheduler: SharedScheduler,
    coroutine_manager: SharedCoroutineManager,
}

#[derive(Debug, thiserror::Error)]
pub enum ScenarioError {
    #[error("lua error: {0}")]
    Lua(#[from] LuaError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("scenario not found")]
    ScenarioNotFound,
}

impl ScenarioRunner {
    pub fn new(scenario_name: &str, danmaku: SharedDanmaku) -> Result<Self, ScenarioError> {
        let lua = Lua::new_with(
            StdLib::MATH | StdLib::TABLE | StdLib::STRING | StdLib::COROUTINE,
            LuaOptions::default(),
        )?;

        let coroutine_manager = CoroutineManager::shared();
        let scheduler = Scheduler::shared();

        let danmaku_table = lua.create_table()?;

        let mut runner = Self {
            lua,
            danmaku: danmaku.clone(),
            danmaku_table,
            coroutine_manager,
            scheduler,
        };

        runner.fill_danmaku_table()?;
        runner.load_scenario(scenario_name)?;

        Ok(runner)
    }

    fn fill_danmaku_table(&mut self) -> Result<(), ScenarioError> {
        self.danmaku_table
            .set("log", crate::api::log::log(&self.lua, &self.danmaku)?)?;
        self.danmaku_table.set(
            "import",
            crate::api::import::import(&self.lua, &self.danmaku)?,
        )?;

        self.danmaku_table.set(
            "bullet",
            crate::api::bullet::create_table(&self.lua, &self.danmaku)?,
        )?;
        self.danmaku_table.set(
            "content",
            crate::api::content::create_table(&self.lua, &self.danmaku)?,
        )?;
        self.danmaku_table.set(
            "motion",
            crate::api::motion::create_table(&self.lua, &self.danmaku)?,
        )?;
        self.danmaku_table.set(
            "rng",
            crate::api::rng::create_table(&self.lua, &self.danmaku)?,
        )?;
        self.danmaku_table.set(
            "stage",
            crate::api::stage::create_table(&self.lua, &self.danmaku)?,
        )?;
        self.danmaku_table.set(
            "task",
            crate::api::task::create_table(&self.lua, &self.danmaku, &self.coroutine_manager)?,
        )?;
        self.danmaku_table.set(
            "time",
            crate::api::time::create_table(
                &self.lua,
                &self.danmaku,
                &self.coroutine_manager,
                &self.scheduler,
            )?,
        )?;

        let math = self
            .lua
            .load(include_str!("api/lua/danmaku.math.lua"))
            .set_name("danmaku.math")
            .eval::<mlua::Table>()?;
        self.danmaku_table.set("math", math)?;

        let utils = self
            .lua
            .load(include_str!("api/lua/danmaku.utils.lua"))
            .set_name("danmaku.utils")
            .eval::<mlua::Table>()?;
        self.danmaku_table.set("utils", utils)?;

        Ok(())
    }

    fn load_scenario(&mut self, scenario_name: &str) -> Result<(), ScenarioError> {
        todo!()
    }

    pub fn update(&self) {
        todo!()
    }

    pub fn danmaku(&self) -> &SharedDanmaku {
        &self.danmaku
    }
}
