return {
    type = "scenario",

    -- Main file, entrypoint for scenario. Used to configure stages, and
    -- other global params
    run = function(danmaku)
        danmaku.stage.load("stages/stage1")
    end
}