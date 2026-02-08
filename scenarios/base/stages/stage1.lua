local BASE_LIFETIME = 1000

local function BloomingFlower(engine, x, y)
	local base_angle = 0
	for wave = 1, 8 do
		for petal = 0, 11 do
			local sprite = (wave % 3 == 0) and "bullet_red" or (wave % 3 == 1) and "bullet_purple" or "bullet_pink"

			engine.spawn_bullet({
				sprite = sprite,
				x = x,
				y = y,
				angle = base_angle + petal * math.pi / 6,
				speed = 2.0 + wave * 0.3,
				angular_vel = 0.002 * wave,
				lifetime = BASE_LIFETIME,
			})
		end
		base_angle = base_angle + 0.2
		engine.wait(3)
	end
end

local function SpinningCross(engine, x, y)
	for _ = 1, 90 do
		for _ = -5, 5 do
			engine.spawn_bullet({
				sprite = "bullet_blue",
				x = x,
				y = y,
				angle = 0,
				speed = 2,
				angular_vel = 0.04,
				lifetime = BASE_LIFETIME,
			})
			engine.spawn_bullet({
				sprite = "bullet_cyan",
				x = x,
				y = y,
				angle = math.pi,
				speed = 2,
				angular_vel = 0.04,
				lifetime = BASE_LIFETIME,
			})
			engine.spawn_bullet({
				sprite = "bullet_green",
				x = x,
				y = y,
				angle = math.pi / 2,
				speed = 2,
				angular_vel = 0.04,
				lifetime = BASE_LIFETIME,
			})
			engine.spawn_bullet({
				sprite = "bullet_light_green",
				x = x,
				y = y,
				angle = math.pi * 1.5,
				speed = 2,
				angular_vel = 0.04,
				lifetime = BASE_LIFETIME,
			})
		end
		engine.wait(1)
	end
end

local function SakuraScatter(engine, x, y)
	for burst = 1, 12 do
		local base = (burst - 1) * math.pi / 6
		for ring = 1, 3 do
			local count = 8 + ring * 4
			local sprite = (ring == 1 and "bullet_pink") or (ring == 2 and "bullet_red") or "bullet_dark_red"
			for i = 0, count - 1 do
				engine.spawn_bullet({
					sprite = sprite,
					x = x,
					y = y,
					angle = base + i * 2 * math.pi / count,
					speed = 1.0 + ring * 0.8,
					angular_vel = (ring - 2) * 0.01,
					lifetime = BASE_LIFETIME,
				})
			end
		end
		engine.wait(4)
	end
end

local function DanmakuWheel(engine, x, y)
	local angle = 0
	for _ = 1, 120 do
		for arm = 0, 5 do
			local a = angle + arm * math.pi / 3
			engine.spawn_bullet({
				sprite = "bullet_yellow",
				x = x,
				y = y,
				angle = a,
				speed = 2.0,
				angular_vel = 0.03,
				lifetime = BASE_LIFETIME,
			})
			engine.spawn_bullet({
				sprite = "bullet_orange",
				x = x,
				y = y,
				angle = a,
				speed = 2.5,
				angular_vel = 0.03,
				lifetime = BASE_LIFETIME,
			})
			engine.spawn_bullet({
				sprite = "bullet_red",
				x = x,
				y = y,
				angle = a,
				speed = 3.0,
				angular_vel = 0.03,
				angular_accel = -0.0003,
				lifetime = BASE_LIFETIME,
			})
		end
		angle = angle + 0.12
		engine.yield()
	end
end

local function ExpandingRings(engine, x, y)
	for wave = 1, 6 do
		local count = 16 + wave * 4
		for i = 0, count - 1 do
			engine.spawn_bullet({
				sprite = (wave % 2 == 0 and "bullet_blue") or "bullet_cyan",
				x = x,
				y = y,
				angle = i * 2 * math.pi / count,
				speed = 1.5,
				angular_vel = 0.015,
				lifetime = BASE_LIFETIME,
			})
		end
		engine.wait(8)
	end
end

local function SpiralGalaxy(engine, x, y)
	local a = 0
	for _ = 1, 100 do
		for i = 1, 3 do
			engine.spawn_bullet({
				sprite = "bullet_purple",
				x = x,
				y = y,
				angle = a + i * 0.5,
				speed = 2,
				angular_vel = 0.05,
				lifetime = BASE_LIFETIME,
			})
			engine.spawn_bullet({
				sprite = "bullet_blue",
				x = x,
				y = y,
				angle = -a + i * 0.5,
				speed = 2,
				angular_vel = -0.05,
				lifetime = BASE_LIFETIME,
			})
		end
		a = a + 0.18
		engine.yield()
	end
end

local function ButterflyWings(engine, x, y)
	for phase = 1, 60 do
		local s = math.sin(phase * 0.1) * math.pi / 3
		local w = math.sin(phase * 0.1) * 0.01
		for i = 0, 5 do
			engine.spawn_bullet({
				sprite = "bullet_green",
				x = x,
				y = y,
				angle = -math.pi / 2 - s + i * 0.2,
				speed = 2.2,
				angular_vel = w,
				lifetime = BASE_LIFETIME,
			})
			engine.spawn_bullet({
				sprite = "bullet_light_green",
				x = x,
				y = y,
				angle = -math.pi / 2 + s - i * 0.2,
				speed = 2.2,
				angular_vel = -w,
				lifetime = BASE_LIFETIME,
			})
			engine.spawn_bullet({
				sprite = "bullet_yellow",
				x = x,
				y = y,
				angle = math.pi / 2 + s - i * 0.2,
				speed = 2.2,
				angular_vel = w,
				lifetime = BASE_LIFETIME,
			})
			engine.spawn_bullet({
				sprite = "bullet_orange",
				x = x,
				y = y,
				angle = math.pi / 2 - s + i * 0.2,
				speed = 2.2,
				angular_vel = -w,
				lifetime = BASE_LIFETIME,
			})
		end
		engine.yield()
	end
end

local PATTERNS = {
	BloomingFlower,
	SpinningCross,
	SakuraScatter,
	DanmakuWheel,
	ExpandingRings,
	SpiralGalaxy,
	ButterflyWings,
}

return {
	name = "Bullet APIs showcase - Enhanced",
	description = "Beautiful parent-child bullet formations",
	IKnowWhatImDoing = true,
	spritesheets = { "./assets/EoSD_bullets.json" },

	run = function(engine)
		while true do
			for i = #PATTERNS, 2, -1 do
				local j = math.random(i)
				PATTERNS[i], PATTERNS[j] = PATTERNS[j], PATTERNS[i]
			end

			for _, p in ipairs(PATTERNS) do
				p(engine, 320, 100)
				engine.wait(20)
			end
			engine.wait(40)
		end
	end,
}
