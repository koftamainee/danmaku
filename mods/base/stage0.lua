return {
	name = "API showcase",
	description = "xD",

	IKnowWhatImDoing = true,

	spritesheets = { "./assets/EoSD_bullets.json" },

	run = function(engine)
		engine.time() -- stage time in frames

		engine.yield() -- yielding to the engine, taking back control next frame
		engine.wait(5) -- yielding to the engine, taking back control after 5 frames
		-- engine.wait(0) == engine.wait(-n) == engine.yield

		-- Fire a simple bullet immediately
		engine.spawn_bullet(
			"bullet_red", -- sprite to use, must be in spritesheet
			300, -- x
			100, -- y
			0, -- angle
			1.5, -- speed, px/frame (optional)
			0.05, -- acceleration, px/frame^2 (optional)
			300 -- lifetime, frames, -1 -- infinite (optional)
		)

		local b = engine.spawn_bullet("bullet_green", 400, 100)

		b:is_alive() -- true if lifetime != 0, false otherwise

		b:set_angle(math.pi) -- argument in radians
		b:set_speed(3)
		b:set_accel(-0.02)
		b:set_lifetime(90) -- b:set_lifetime(0) kills bullet

		-- Fire a small pattern every 30 frames
		while true do
			for i = 0, 4 do
				local angle = i * (math.pi / 8) - (math.pi / 16) -- spread pattern
				local bullet = engine.spawn_bullet("bullet_red", 320, 240, angle, 2, 0.02, 200)
				bullet:set_lifetime(200)
			end
			engine.wait(30) -- pause for 30 frames before next wave
		end

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
