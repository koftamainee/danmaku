return {
	name = "Bullet APIs showcase",
	description = "xD",

	IKnowWhatImDoing = true,

	spritesheets = { "./assets/EoSD_bullets.json" },

	run = function(engine)
		--- Full bullet structure (all optional except sprite, x/y if no parent):
		--
		-- REQUIRED FIELDS:
		-- sprite          = nil                   -- string, required: the sprite used for the bullet
		-- x               = nil                   -- number, required if no parent: initial X position in pixels
		-- y               = nil                   -- number, required if no parent: initial Y position in pixels
		--
		-- VELOCITY (polar coordinates):
		-- speed           = 0                     -- number, px/frame: initial speed along angle
		-- accel           = 0                     -- number, px/frame^2: acceleration along current angle
		-- min_speed       = nil                   -- number, px/frame: minimum speed
		-- max_speed       = nil                   -- number, px/frame: maximum speed
		--
		-- ANGLE & ROTATION:
		-- angle           = 0                     -- number, radians: initial movement angle
		-- angle_type      = engine.angle.absolute -- absolute | relative | player: how angle is interpreted
		-- angular_vel     = 0                     -- number, radians/frame: rotational velocity
		-- angular_accel   = 0                     -- number, radians/frame^2: rotational acceleration
		-- min_angular_vel = nil                   -- number, radians/frame: minimum angular velocity
		-- max_angular_vel = nil                   -- number, radians/frame: maximum angular velocity
		--
		-- HIERARCHY:
		-- parent          = nil                   -- optional bullet handle: attach bullet to another bullet
		-- parent_offset   = nil                   -- table {x=0, y=0}: offset relative to parent if attached
		--
		-- LIFETIME:
		-- lifetime        = -1                    -- number of frames bullet lives; -1 = infinite
		--
		-- Notes:
		-- Bullets are dumb, engine handles movement and deletion.
		-- If parent exists, x/y are ignored.
		-- Child bullets follow parent transform if attached.

		-- Fire a simple bullet immediately
		local a = engine.spawn_bullet({
			sprite = "bullet_red",
			x = 300,
			y = 100,
			speed = 1.5,
			angle = 0,
			accel = 0.05,
		})

		-- Fire another bullet
		local b = engine.spawn_bullet({
			sprite = "bullet_green",
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
		b:set_angle(math.pi, engine.angle.absolute) -- absolute: set to π
		b:set_angle(math.pi / 4, engine.angle.relative) -- relative: rotate current angle by +π/4
		b:set_angle(math.pi / 8, engine.angle.player) -- player-relative: aim toward player
		b:aim() -- shortcut for player-relative aiming

		-- Parent/child management
		b:set_parent_offset({ x = 10, y = 0 }) -- offset relative to parent (if attached)
		b:attach_to(a) -- attach to parent
		b:detach() -- detach from parent
		b:attach_to(a, { x = 5, y = 5 }) -- attach to another parent with offset

		-- Lifetime
		b:set_lifetime(90) -- auto-delete after 90 frames

		-- Timings and yielding
		engine.time() -- stage time in frames

		engine.yield() -- yield to engine, resume next frame
		engine.wait(5) -- yield for 5 frames
		-- engine.wait(0) == engine.yield()

		-- EXAMPLE 1: Simple radial burst
		print("Got here")
		local center_x, center_y = 300, 200
		local bullet_count = 16
		local speed = 2.5

		for i = 1, bullet_count do
			local angle = (2 * math.pi / bullet_count) * i
			engine.spawn_bullet({
				sprite = "bullet_red",
				x = center_x,
				y = center_y,
				speed = speed,
				angle = angle,
			})
		end

		engine.wait(60)

		-- EXAMPLE 2: Accelerating spiral
		center_x, center_y = 400, 200
		local bullets_to_spawn = 36

		for i = 1, bullets_to_spawn do
			local angle = (2 * math.pi / 12) * i
			engine.spawn_bullet({
				sprite = "bullet_blue",
				x = center_x,
				y = center_y,
				speed = 0.5,
				accel = 0.03,
				max_speed = 4,
				angle = angle,
			})
			engine.wait(2)
		end

		engine.wait(100)

		-- EXAMPLE 3: Aimed bullets at player with spread
		center_x, center_y = 350, 100
		local burst_count = 5
		local spread = 0.3 -- radians

		for i = 1, burst_count do
			local offset = (i - (burst_count + 1) / 2) * spread
			b = engine.spawn_bullet({
				sprite = "bullet_purple",
				x = center_x,
				y = center_y,
				speed = 3,
			})
			-- Aim at player with offset
			b:set_angle(offset, engine.angle.player)
		end

		engine.wait(80)

		-- EXAMPLE 4: Parent-child orbital formation
		local parent = engine.spawn_bullet({
			sprite = "bullet_yellow",
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

			engine.spawn_bullet({
				sprite = "bullet_red",
				parent = parent,
				parent_offset = { x = offset_x, y = offset_y },
			})
			engine.yield() -- Spawn one-by-one
		end

		engine.wait(120)

		-- NOTES:
		--
		-- math.random() is only availableway of getting
		-- random numbers in restricted mode (check below)
		-- engine seeds it before running the stage
		-- the same seed + same inputs = identical replay
		--
		--
		-- SANDBOX RESTRICTIONS
		-- In normal mode (IKnowWhatImDoing = false):
		--
		-- PROHIBETED:
		-- os.*, io.*, debug.* are NOT available
		-- math.randomseed() is NOT available
		-- system time, filesystem, env vars are blocked
		-- loading native C modules is blocked
		--
		-- ALLOWED:
		-- math.sin, cos, atan2, sqrt, etc. are allowed
		-- math.random() is allowed (engine-seeded)
		--
		-- This guarantees:
		-- - deterministic patterns
		-- - replay safety
		-- - no engine or system abuse
		--
		--
		-- IKnowWhatImDoing FLAG
		-- Since this stage sets:
		--     IKnowWhatImDoing = true
		--
		-- the sandbox is relaxed for this script.
		-- All of Lua standart library is available.
		-- The engine expose field with seed used for the
		-- current game sesssion
		--
		--   engine.rng_seed
		--
		-- You CAN define your RNGs,
		-- but you need to use provided seed.
		-- If not, replay system will break
		--
		-- Unrestricted mode is intended for:
		-- - experiments
		-- - tooling
		-- - non-replayable stages
	end,
}
