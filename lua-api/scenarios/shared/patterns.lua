return {
    type = "module"
    -- in module you can define shared code. can be loaded to other scenes
    -- using danmaku.import("module_path")
    circle = function(danmaku, bullet, x, y, count, speed, angle_offset)
            angle_offset = angle_offset or 0
            for i = 1, count do
                danmaku.bullet.spawn({
                    sprite = bullet,
                    x = x,
                    y = y,
                    speed = speed,
                    angle = angle,
                })
            end
        end
}