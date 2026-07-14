return {
    type = "stage",
    
    prepare = function(danmaku, ctx)
        -- all bullets types available like bullets.bullet_red
        -- assets.EoSD_bullets.lua file is generated in the editor
        ctx.bullets = danmaku.content.load_spritesheet("assets/EoSD_bullets.lua")
        local patterns = danmaku.import("shared/patterns.lua")
        ctx.patterns = {
            patterns.blooming_flower,
            patterns.spinning_cross,
            patterns.sakura_scatter,
            patterns.danmaku_wheel,
            patterns.expanding_rings,
            patterns.spiral_galaxy,
            patterns.butterfly_wings,
        }
    end

    run = function(danmaku, ctx)
        local base_lifetime = 500

        while true do
            danmaku.rng.shuffle(ctx.patterns)

            for _, pattern in ctx.patterns do
                pattern(danmaku, 320, 100, ctx.bulltes, base_lifetime)
                danmaku.time.wait(20)
            end
            danmaku.time.wait(40)
        end

    end
}