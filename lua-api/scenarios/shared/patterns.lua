return {
    type = "module",
    -- in module you can define shared code. can be loaded to other scenes
    -- using danmaku.import("module_path")

    blooming_flower = function(danmaku, x, y, bullets, lifetime)
        local base_angle = 0
        for wave = 1, 8 do
            for petal = 0, 11 do
                local sprite = (wave % 3 == 0) and bullets.bullet_red or (wave % 3 == 1) and bullets.bullet_purple or bullets.bullet_pink
                danmaku.bullet.spawn({
                    sprite = sprite,
                    x = x,
                    y = y,
                    angle = base_angle + petal * math.pi / 6,
                    speed = 2.0 + wave * 0.3,
                    angular_vel = 0.002 * wave,
                    lifetime = lifetime,
                })
            end
            base_angle = base_angle + 0.2
            danmaku.time.wait(3)
        end
    end,

    spinning_cross = function(danmaku, x, y, bullets, lifetime)
        for _ = 1, 90 do
            for _ = -5, 5 do
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_blue,
                    x = x, y = y,
                    angle = 0,
                    speed = 2,
                    angular_vel = 0.04,
                    lifetime = lifetime,
                })
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_cyan,
                    x = x, y = y,
                    angle = math.pi,
                    speed = 2,
                    angular_vel = 0.04,
                    lifetime = lifetime,
                })
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_green,
                    x = x, y = y,
                    angle = math.pi / 2,
                    speed = 2,
                    angular_vel = 0.04,
                    lifetime = lifetime,
                })
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_light_green,
                    x = x, y = y,
                    angle = math.pi * 1.5,
                    speed = 2,
                    angular_vel = 0.04,
                    lifetime = lifetime,
                })
            end
            danmaku.time.wait(1)
        end
    end,

    sakura_scatter = function(danmaku, x, y, bullets, lifetime)
        for burst = 1, 12 do
            local base = (burst - 1) * math.pi / 6
            for ring = 1, 3 do
                local count = 8 + ring * 4
                local sprite = (ring == 1 and bullets.bullet_pink) or (ring == 2 and bullets.bullet_red) or bullets.bullet_dark_red
                for i = 0, count - 1 do
                    danmaku.bullet.spawn({
                        sprite = sprite,
                        x = x, y = y,
                        angle = base + i * 2 * math.pi / count,
                        speed = 1.0 + ring * 0.8,
                        angular_vel = (ring - 2) * 0.01,
                        lifetime = lifetime,
                    })
                end
            end
            danmaku.time.wait(4)
        end
    end,

    danmaku_wheel = function(danmaku, x, y, bullets, lifetime)
        local angle = 0
        for _ = 1, 120 do
            for arm = 0, 5 do
                local a = angle + arm * math.pi / 3
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_yellow,
                    x = x, y = y,
                    angle = a,
                    speed = 2.0,
                    angular_vel = 0.03,
                    lifetime = lifetime,
                })
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_orange,
                    x = x, y = y,
                    angle = a,
                    speed = 2.5,
                    angular_vel = 0.03,
                    lifetime = lifetime,
                })
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_red,
                    x = x, y = y,
                    angle = a,
                    speed = 3.0,
                    angular_vel = 0.03,
                    angular_accel = -0.0003,
                    lifetime = lifetime,
                })
            end
            angle = angle + 0.12
            danmaku.time.yield()
        end
    end,

    expanding_rings = function(danmaku, x, y, bullets, lifetime)
        for wave = 1, 6 do
            local count = 16 + wave * 4
            for i = 0, count - 1 do
                danmaku.bullet.spawn({
                    sprite = (wave % 2 == 0 and bullets.bullet_blue) or bullets.bullet_cyan,
                    x = x, y = y,
                    angle = i * 2 * math.pi / count,
                    speed = 1.5,
                    angular_vel = 0.015,
                    lifetime = lifetime,
                })
            end
            danmaku.time.wait(8)
        end
    end,

    spiral_galaxy = function(danmaku, x, y, bullets, lifetime)
        local a = 0
        for _ = 1, 100 do
            for i = 1, 3 do
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_purple,
                    x = x, y = y,
                    angle = a + i * 0.5,
                    speed = 2,
                    angular_vel = 0.05,
                    lifetime = lifetime,
                })
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_blue,
                    x = x, y = y,
                    angle = -a + i * 0.5,
                    speed = 2,
                    angular_vel = -0.05,
                    lifetime = lifetime,
                })
            end
            a = a + 0.18
            danmaku.time.yield()
        end
    end,

    butterfly_wings = function(danmaku, x, y, bullets, lifetime)
        for phase = 1, 60 do
            local s = math.sin(phase * 0.1) * math.pi / 3
            local w = math.sin(phase * 0.1) * 0.01
            for i = 0, 5 do
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_green,
                    x = x, y = y,
                    angle = -math.pi / 2 - s + i * 0.2,
                    speed = 2.2,
                    angular_vel = w,
                    lifetime = lifetime,
                })
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_light_green,
                    x = x, y = y,
                    angle = -math.pi / 2 + s - i * 0.2,
                    speed = 2.2,
                    angular_vel = -w,
                    lifetime = lifetime,
                })
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_yellow,
                    x = x, y = y,
                    angle = math.pi / 2 + s - i * 0.2,
                    speed = 2.2,
                    angular_vel = w,
                    lifetime = lifetime,
                })
                danmaku.bullet.spawn({
                    sprite = bullets.bullet_orange,
                    x = x, y = y,
                    angle = math.pi / 2 - s + i * 0.2,
                    speed = 2.2,
                    angular_vel = -w,
                    lifetime = lifetime,
                })
            end
            danmaku.time.yield()
        end
    end,
}