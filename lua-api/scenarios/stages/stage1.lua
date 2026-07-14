return {
    type = "stage",
    
    prepare = function(danmaku, ctx)
        -- all bullets types available like bullets.bullet_red
        -- assets.EoSD_bullets.lua file is generated in the editor
        ctx.bullets = danmaku.content.load_spritesheet("assets/EoSD_bullets.lua")
        ctx.patterns = danmaku.import("shared/patterns.lua")
    end

    run = function(danmaku, ctx)
        local current_angle = 0.0

        while true do
            danmaku.time.wait(10) -- waits 10 frames
            current_angle = current_angle + danmaku.rng.range(10.0, 20.0)
            current_angle = current_angle % 360
            ctx.patterns.circle(danmaku, ctx.bullets.bullet_red, 0, 0, 10, 100, current_angle)
        end
    end
}