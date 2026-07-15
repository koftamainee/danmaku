return {
    run = function(danmaku) 
        -- global namespace - project structure function
        danmaku.log("prints the message")
        local module = danmaku.import("path/to/module") -- imports module type file
        
        -- stage - stage management
        danmaku.stage.load("path/to/stage") -- loads stage (blocking)


        -- content - loads and manage assets
        -- NOTE: spritesheet internally using one texture and it is more preferrable to optimize draw calls
        -- NOTE: though everywhere you use sprite, you can use spritesheet.sprite_name, they are the same handle
        local spritesheet = danmaku.content.load_spritesheet("path/to/spritesheet") -- load spritesheet from editor-generated file
        local sprite = danmaku.content.load_sprite("path/to/sprite") -- load sprite from editor-generated file

        -- time - coroutine management function
        -- NOTE: time correlate with task modules, it is abstraction from coroutines
        -- NOTE: we separate it for clearer dev exp
        -- NOTE: danmaku.time.every is automatically killed after stage end
        local time = danmaku.time.current() -- get time in frames from stage start
        danmaku.time.yield() -- gives control flow to engine
        danmaku.time.wait(5) -- gives control flow to engine and waits 5 frames
        local id1 = danmaku.time.at(60, function() end) -- runs callback at 60 frame
        local id2 = danmaku.time.after(60, function() end) -- runs callback after 60 frame from danmaku.time.current()
        local id3 = danmaku.time.every(60, function() end) -- runs callback every 60 frames
        danmaku.time.cancel(id1) -- cancels scheduled callback

        -- task - spawns another coroutines
        local task_id = danmaku.task.spawn(function() end) -- spawns new coroutine and launch passed function
        danmaku.task.join(task_id) -- wait for coroutine with task_id to finish (blocking)
        danmaku.task.cancel(task_id) -- kill coroutine

        -- rng - seed-deterministic random number generation
        local unit = danmaku.rng.float() -- generates random float from 0 to 1
        local fnum = danmaku.rng.rangef(0.0, 10.0) -- generates float from
        local inum = danmaku.rng.rangei(0, 10) -- generates random int
        local angle = danmaku.rng.angle() -- generates random angle from 0 to 2 pi
        local angle = danmaku.rng.angle_range(0, math.pi) -- generates random angle in range
        local boolean = danmaku.rng.bool() -- generates random bool
        local element = danmaku.rng.choice({1, 2, 3}) -- choose random element from list
        danmaku.rng.shuffle({1, 2, 3}) -- shuffles list in place
        local sample = danmaku.rng.sample({1, 2, 3}, 2) -- returns new table with 2 random elements from list
        local direction = danmaku.rng.direction() -- returns random normalized direction
        local sign = danmaku.rng.sign() -- generates 1 or -1


        -- math - math utils that are absent in standard math module
        local clamp = danmaku.math.clamp(5, 1, 10) -- clamps value between min and max
        local lerp = danmaku.math.lerp(0, 10, 0.5) -- lerps value
        local distance = danmaku.math.distance(0, 0, 10, 15) -- get distance between two points
        local diff = danmaku.math.diff(math.pi, 5) -- normalized distance between angles
        local angle = danmaku.math.normalize_angle(5) -- normalize angle
        local deg = danmaku.math.deg(math.pi) -- radians -> degree
        local rad = danmaku.math.rad(180) -- degree -> radians
        local angle = danmaku.math.angle(10, 2, 3, 5) -- angle between two points
        local num = danmaku.math.map_range(3, 0, 5, 0, 10) -- maps value from one range to another - (value, from_min, from_max, to_min, to_max)

        -- tween - interpolation
        local t1 = danmaku.tween.linear(0.5)
        local t2 = danmaku.tween.quad_in(0.5)
        local t3 = danmaku.tween.quad_out(0.5)
        local t4 = danmaku.tween.quad_in_out(0.5)
        local t5 = danmaku.tween.cubic_in(0.5)
        local t6 = danmaku.tween.cubic_out(0.5)
        local t7 = danmaku.tween.bounce(0.5)
        local t8 = danmaku.tween.elastic(0.5)

        -- bullet - bullet management functions
        local bullet = danmaku.bullet.spawn({--[[ bullet table ]]}) -- spawn and return new bullet 
        local bullet = danmaku.bullet.spawn_controlled({--[[ spawn controlled table ]]}) -- spawn bullet with custom per-frame update callback
        local bullets = danmaku.bullet.spawn_batch({--[[ array of bullet tables ]]}) -- spawn and return array of bullets
        local bullets_count = danmaku.bullet.count() -- current bullet count in bullet system
        local bullets_get = danmaku.bullet.get_all(); -- returns all active bullets
        -- motion - movement modifiers applied on top of base motion
        -- passed as `motion` field in bullet spawn table
        local m1 = danmaku.motion.sinusoidal(30, 0.1) -- amplitude (px), frequency (osc/frame)
        local m2 = danmaku.motion.sinusoidal(30, 0.1, math.pi / 2) -- + phase offset (rad)
        local m3 = danmaku.motion.lerp({ speed = 5, angle = math.pi / 4 }, 60) -- target values, duration (frames)
        local m4 = danmaku.motion.lerp({ speed = 5 }, 60, "quad_out") -- + easing function name

        -- utils
        danmaku.utils.map({1, 2, 3}, function(x) return x + 1 end) -- apply function on all values in list
        local table1 = danmaku.utils.table_copy({1, 2, 3}) -- shallow copy table
        local table2 = danmaku.utils.table_deep_copy({1, {2, 3, 3}, 3}) -- deep copy table
        local keys = danmaku.utils.table_keys({a = 5, b = 4}) -- returns keys array
        local values = danmaku.utils.table_values({a = 4, b = 5}) -- returns values array
        local merged = danmaku.utils.table_merge({a = 4, b = 5}, {a = 3, c = 7}) -- merges the tables into new one, second takes priority
        danmaku.utils.table_merge_into({a = 4, b = 5}, {a = 3, c = 7}) -- merges second table in first, overrides values 

    end
}