return {
    type = "stage",

    prepare = function(danmaku, ctx)
        ctx.bullets =  danmaku.content.load_spritesheet("assets/EoSD_bullets.lua")
    end

	run = function(danmaku, ctx)
		--- Full bullet structure (all optional except sprite, x/y if no parent):
		--
		-- REQUIRED FIELDS:
		-- sprite          = nil                   -- sprite, required: the sprite + collision data used for the bullet
		-- x               = nil                   -- number, required if no parent: initial X position in pixels
		-- y               = nil                   -- number, required if no parent: initial Y position in pixels
		--
		-- VELOCITY (polar coordinates):
		-- speed           = 0                     -- number, px/frame: initial speed along angle
		-- accel           = 0                     -- number, px/frame^2: acceleration along current angle
		-- min_speed       = nil                   -- number, px/frame: minimum speed (nil = no clamp)
		-- max_speed       = nil                   -- number, px/frame: maximum speed (nil = no clamp)
		--
		-- ANGLE & ROTATION:
		-- angle           = 0                     -- number, radians: initial movement angle
		-- angle_type      = danmaku.angle.absolute -- absolute | relative | player: how angle is interpreted
		-- angular_vel     = 0                     -- number, radians/frame: rotational velocity
		-- angular_accel   = 0                     -- number, radians/frame^2: rotational acceleration
		-- min_angular_vel = nil                   -- number, radians/frame: minimum angular velocity (nil = no clamp)
		-- max_angular_vel = nil                   -- number, radians/frame: maximum angular velocity (nil = no clamp)
		--
		-- HIERARCHY:
		-- parent          = nil                   -- optional bullet handle: attach bullet to another bullet
		-- parent_offset   = nil                   -- table {x=0, y=0}: offset relative to parent if attached
		--
		-- LIFETIME:
		-- lifetime        = nil                   -- number of frames bullet lives; nil = infinite
		--
		-- Notes:
		-- Bullets are dumb, engine handles movement and deletion.
		-- If parent exists, x/y are ignored.
		-- Child bullets follow parent transform if attached.
		-- When parent dies, child becomes independent root bullet and continues flying.

		-- Fire a simple bullet immediately
		local a = danmaku.bullet.spawn({
			sprite = ctx.bullets.bullet_red,
			x = 300,
			y = 100,
			speed = 1.5,
			angle = 0,
			accel = 0.05,
		})

		-- Fire another bullet
		local b = danmaku.bullet.spawn({
			sprite = ctx.bullets.bullet_green,
			x = 400,
			y = 100,
		})

		-- Modify bullet motion after spawn
		--
		-- Linear motion
		b:set_speed(3) -- current speed (px/frame)
		b:set_accel(-0.02) -- acceleration along current angle
		b:set_max_speed(5) -- cap speed
		b:set_min_speed(1) -- floor speed
		b:set_speed_limits(1, 5) -- shorthand for min/max speed

		-- Angular motion
		b:set_angular_vel(0.05) -- radians/frame
		b:set_angular_accel(-0.001) -- radians/frame^2
		b:set_max_angular_vel(0.1) -- cap rotation speed
		b:set_min_angular_vel(-0.05) -- minimum angular velocity
		b:set_angular_vel_limits(-0.05, 0.1) -- shorthand for min/max angular velocity

		-- Angle
		b:set_angle(math.pi, danmaku.angle.absolute) -- absolute: set to pi
		b:set_angle(math.pi / 4, danmaku.angle.relative) -- relative: rotate current angle by +pi/4
		b:set_angle(math.pi / 8, danmaku.angle.player) -- player-relative: aim toward player
		b:aim() -- shortcut for player-relative aiming

		-- Parent/child management
		b:set_parent_offset({ x = 10, y = 0 }) -- offset relative to parent (if attached)
		b:attach_to(a) -- attach to parent
		b:detach() -- detach from parent
		b:attach_to(a, { x = 5, y = 5 }) -- attach to another parent with offset

		-- Lifetime
		b:set_lifetime(90) -- auto-delete after 90 frames
        b:kill() -- alias for b:set_lifetime(0)

		-- Timings and yielding
		local frames = danmaku.time.current() -- stage time in frames

		danmaku.time.yield() -- yield to engine, resume next frame
		danmaku.time.wait(5) -- yield for 5 frames
		-- danmaku.time.wait(0) == danmaku.time.yield()

		-- EXAMPLE 1: Simple radial burst
		local center_x, center_y = 300, 200
		local bullet_count = 16
		local speed = 2.5

		for i = 1, bullet_count do
			local angle = (2 * math.pi / bullet_count) * i
			danmaku.bullet.spawn({
				sprite = ctx.bullets.bullet_blue,
				x = center_x,
				y = center_y,
				speed = speed,
				angle = angle,
			})
		end

		danmaku.time.wait(60)

		-- EXAMPLE 2: Accelerating spiral
		center_x, center_y = 400, 200
		local bullets_to_spawn = 36

		for i = 1, bullets_to_spawn do
			local angle = (2 * math.pi / 12) * i
			danmaku.bullet.spawn({
				sprite = ctx.bullets.bullet_red,
				x = center_x,
				y = center_y,
				speed = 0.5,
				accel = 0.03,
				max_speed = 4,
				angle = angle,
			})
			danmaku.time.wait(2)
		end

		danmaku.time.wait(100)

		-- EXAMPLE 3: Aimed bullets at player with spread
		center_x, center_y = 350, 100
		local burst_count = 5
		local spread = 0.3 -- radians

		for i = 1, burst_count do
			local offset = (i - (burst_count + 1) / 2) * spread
			b = danmaku.bullet.spawn({
				sprite = ctx.bullets.bullet_purple,
				x = center_x,
				y = center_y,
				speed = 3,
			})
			-- Aim at player with offset
			b:set_angle(offset, danmaku.angle.player)
		end

		danmaku.time.wait(80)

		-- EXAMPLE 4: Parent-child orbital formation
		local parent = danmaku.bullet.spawn({
			sprite = ctx.bullets.bullet_green,
			x = 300,
			y = 150,
			speed = 1.5,
			angle = math.pi / 4,
		})
		parent:set_angular_vel(0.01)

		-- Attach child bullets in orbit
		local orbit_count = 8
		local orbit_radius = 40

		for i = 1, orbit_count do
			local angle = (2 * math.pi / orbit_count) * i
			local offset_x = math.cos(angle) * orbit_radius
			local offset_y = math.sin(angle) * orbit_radius

			danmaku.bullet.spawn({
				sprite = ctx.bullets.bullet_red,
				parent = parent,
				parent_offset = { x = offset_x, y = offset_y },
			})
			danmaku.time.yield() -- Spawn one-by-one
		end

		danmaku.time.wait(120)
	end,
}
