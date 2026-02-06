local BASE_LIFETIME = 1000

local function spawn(engine, t)
	t.lifetime = t.lifetime or BASE_LIFETIME
	return engine.spawn_bullet(t)
end

local function BloomingFlower(engine, x, y)
	local base_angle = 0
	for wave = 1, 8 do
		for petal = 0, 11 do
			local sprite = (wave % 3 == 0) and "bullet_red" or (wave % 3 == 1) and "bullet_purple" or "bullet_pink"

			spawn(engine, {
				sprite = sprite,
				x = x,
				y = y,
				angle = base_angle + petal * math.pi / 6,
				speed = 2.0 + wave * 0.3,
				angular_vel = 0.002 * wave,
			})
		end
		base_angle = base_angle + 0.2
		engine.wait(3)
	end
end

local function SpinningCross(engine, x, y)
	for _ = 1, 90 do
		for _ = -5, 5 do
			spawn(engine, { sprite = "bullet_blue", x = x, y = y, angle = 0, speed = 2, angular_vel = 0.04 })
			spawn(engine, { sprite = "bullet_cyan", x = x, y = y, angle = math.pi, speed = 2, angular_vel = 0.04 })
			spawn(engine, { sprite = "bullet_green", x = x, y = y, angle = math.pi / 2, speed = 2, angular_vel = 0.04 })
			spawn(
				engine,
				{ sprite = "bullet_light_green", x = x, y = y, angle = math.pi * 1.5, speed = 2, angular_vel = 0.04 }
			)
		end
		engine.yield()
	end
end

local function SakuraScatter(engine, x, y)
	for burst = 1, 12 do
		local base = (burst - 1) * math.pi / 6
		for ring = 1, 3 do
			local count = 8 + ring * 4
			local sprite = (ring == 1 and "bullet_pink") or (ring == 2 and "bullet_red") or "bullet_dark_red"
			for i = 0, count - 1 do
				spawn(engine, {
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
			spawn(engine, { sprite = "bullet_yellow", x = x, y = y, angle = a, speed = 2.0, angular_vel = 0.03 })
			spawn(engine, { sprite = "bullet_orange", x = x, y = y, angle = a, speed = 2.5, angular_vel = 0.03 })
			spawn(engine, {
				sprite = "bullet_red",
				x = x,
				y = y,
				angle = a,
				speed = 3.0,
				angular_vel = 0.03,
				angular_accel = -0.0003,
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
			spawn(engine, {
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
			spawn(engine, {
				sprite = "bullet_purple",
				x = x,
				y = y,
				angle = a + i * 0.5,
				speed = 2,
				angular_vel = 0.05,
				lifetime = BASE_LIFETIME,
			})
			spawn(engine, {
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
			spawn(engine, {
				sprite = "bullet_green",
				x = x,
				y = y,
				angle = -math.pi / 2 - s + i * 0.2,
				speed = 2.2,
				angular_vel = w,
			})
			spawn(engine, {
				sprite = "bullet_light_green",
				x = x,
				y = y,
				angle = -math.pi / 2 + s - i * 0.2,
				speed = 2.2,
				angular_vel = -w,
			})
			spawn(engine, {
				sprite = "bullet_yellow",
				x = x,
				y = y,
				angle = math.pi / 2 + s - i * 0.2,
				speed = 2.2,
				angular_vel = w,
			})
			spawn(engine, {
				sprite = "bullet_orange",
				x = x,
				y = y,
				angle = math.pi / 2 - s + i * 0.2,
				speed = 2.2,
				angular_vel = -w,
			})
		end
		engine.yield()
	end
end

local function CascadingWaves(engine, x, y)
	local off = 0
	for wave = 1, 15 do
		for i = 0, 23 do
			local sprite = (i % 3 == 0 and "bullet_blue") or (i % 3 == 1 and "bullet_purple") or "bullet_cyan"
			spawn(engine, {
				sprite = sprite,
				x = x,
				y = y,
				angle = i * 2 * math.pi / 24 + off,
				speed = 1.8 + math.sin(wave * 0.5) * 0.5,
				angular_vel = 0.015,
				lifetime = BASE_LIFETIME,
			})
		end
		off = off + 0.26
		engine.wait(3)
	end
end

local function DragonBreath(engine, x, y)
	local dir = -math.pi / 2
	for _ = 1, 80 do
		for i = -8, 8 do
			spawn(engine, {
				sprite = (i < 0 and "bullet_orange") or (i > 0 and "bullet_yellow") or "bullet_red",
				x = x,
				y = y,
				angle = dir + i * (math.pi / 4) / 16,
				speed = 2.5 + math.abs(i) * 0.1,
				angular_vel = i * 0.004,
				lifetime = BASE_LIFETIME,
			})
		end
		dir = dir + 0.05
		engine.yield()
	end
end

-- =========================
-- PARENT-CHILD PATTERNS
-- =========================

local function OrbitingConstellation(engine, x, y)
	for cluster = 1, 6 do
		local parent = spawn(engine, {
			sprite = "bullet_yellow",
			x = x,
			y = y,
			angle = (cluster - 1) * math.pi / 3,
			speed = 1.8,
			angular_vel = 0.02,
		})

		for i = 0, 7 do
			local orbit_angle = i * 2 * math.pi / 8
			local radius = 30
			spawn(engine, {
				sprite = (i % 2 == 0) and "bullet_orange" or "bullet_red",
				parent = parent,
				parent_offset = { x = math.cos(orbit_angle) * radius, y = math.sin(orbit_angle) * radius },
			})
		end
		engine.wait(8)
	end
	engine.wait(60)
end

local function RotatingBarrier(engine, x, y)
	for wave = 1, 4 do
		for arm = 0, 5 do
			local parent = spawn(engine, {
				sprite = "bullet_cyan",
				x = x,
				y = y,
				angle = arm * math.pi / 3 + wave * 0.3,
				speed = 0.5,
				angular_vel = 0.04,
			})

			for segment = 1, 8 do
				spawn(engine, {
					sprite = (segment % 3 == 0) and "bullet_blue" or "bullet_purple",
					parent = parent,
					parent_offset = { x = segment * 15, y = 0 },
				})
			end
		end
		engine.wait(20)
	end
end

local function BlossomingTree(engine, x, y)
	local trunk = spawn(engine, {
		sprite = "bullet_green",
		x = x,
		y = y,
		angle = -math.pi / 2,
		speed = 2.0,
	})

	engine.wait(15)

	for branch = 0, 5 do
		local branch_bullet = spawn(engine, {
			sprite = "bullet_light_green",
			parent = trunk,
			parent_offset = { x = 0, y = 0 },
			angle = -math.pi / 2 + (branch - 2.5) * 0.4,
			speed = 1.5,
		})
		branch_bullet:detach()

		engine.wait(3)

		for petal = 0, 11 do
			local petal_angle = petal * 2 * math.pi / 12
			local radius = 25
			spawn(engine, {
				sprite = "bullet_pink",
				parent = branch_bullet,
				parent_offset = { x = math.cos(petal_angle) * radius, y = math.sin(petal_angle) * radius },
			})
		end
	end
	engine.wait(60)
end

local function ChainReaction(engine, x, y)
	for chain = 1, 8 do
		local prev = nil
		local chain_angle = (chain - 1) * math.pi / 4

		for link = 1, 6 do
			local bullet = spawn(engine, {
				sprite = (link % 2 == 0) and "bullet_yellow" or "bullet_orange",
				x = x,
				y = y,
				angle = chain_angle,
				speed = 2.0 + link * 0.2,
			})

			if prev then
				spawn(engine, {
					sprite = "bullet_red",
					parent = prev,
					parent_offset = { x = 20, y = 0 },
				})
			end
			prev = bullet
		end
		engine.wait(5)
	end
end

local function SpinningLotus(engine, x, y)
	for layer = 1, 5 do
		local petal_count = 6 + layer * 2

		for i = 0, petal_count - 1 do
			local parent = spawn(engine, {
				sprite = "bullet_pink",
				x = x,
				y = y,
				angle = i * 2 * math.pi / petal_count + layer * 0.2,
				speed = 1.2 + layer * 0.3,
				angular_vel = 0.03 * (layer % 2 == 0 and 1 or -1),
			})

			for tip = 1, 3 do
				spawn(engine, {
					sprite = (tip == 1) and "bullet_red" or (tip == 2) and "bullet_purple" or "bullet_blue",
					parent = parent,
					parent_offset = { x = tip * 12, y = 0 },
				})
			end
		end
		engine.wait(10)
	end
end

local function SwarmingFireflies(engine, x, y)
	for swarm = 1, 12 do
		local center = spawn(engine, {
			sprite = "bullet_yellow",
			x = x,
			y = y,
			angle = math.random() * 2 * math.pi,
			speed = 1.0 + math.random() * 1.0,
			angular_vel = (math.random() - 0.5) * 0.06,
		})

		local count = 8 + math.random(8)
		for i = 1, count do
			local orbit_angle = math.random() * 2 * math.pi
			local orbit_radius = 15 + math.random() * 25
			spawn(engine, {
				sprite = (i % 3 == 0) and "bullet_green" or "bullet_light_green",
				parent = center,
				parent_offset = {
					x = math.cos(orbit_angle) * orbit_radius,
					y = math.sin(orbit_angle) * orbit_radius,
				},
			})
		end
		engine.wait(6)
	end
end

local function CelestialRings(engine, x, y)
	for ring = 1, 4 do
		local core = spawn(engine, {
			sprite = "bullet_purple",
			x = x,
			y = y,
			speed = 0.8,
			angle = ring * math.pi / 2,
			angular_vel = 0.025,
		})

		local satellites = 12 + ring * 4
		for i = 0, satellites - 1 do
			local angle = i * 2 * math.pi / satellites
			local radius = 35 + ring * 10

			local satellite = spawn(engine, {
				sprite = "bullet_blue",
				parent = core,
				parent_offset = { x = math.cos(angle) * radius, y = math.sin(angle) * radius },
			})

			-- Add moons to the outermost ring (grandchildren: core -> satellite -> moon)
			if ring == 4 and i % 3 == 0 then
				for moon = 0, 3 do
					local moon_angle = moon * math.pi / 2
					spawn(engine, {
						sprite = "bullet_cyan",
						parent = satellite,
						parent_offset = { x = math.cos(moon_angle) * 12, y = math.sin(moon_angle) * 12 },
					})
				end
			end
		end
		engine.wait(15)
	end
end

local function PhoenixFeathers(engine, x, y)
	for wing = 0, 1 do
		local base_angle = -math.pi / 2 + (wing == 0 and -1 or 1) * math.pi / 3

		for feather = 1, 8 do
			local parent = spawn(engine, {
				sprite = "bullet_red",
				x = x,
				y = y,
				angle = base_angle + (feather - 4.5) * 0.15,
				speed = 2.0 + feather * 0.1,
				angular_vel = (wing == 0 and 1 or -1) * 0.02,
			})

			for barb = 1, 5 do
				spawn(engine, {
					sprite = (barb % 2 == 0) and "bullet_orange" or "bullet_yellow",
					parent = parent,
					parent_offset = { x = barb * 10, y = (barb % 2 == 0 and 1 or -1) * 8 },
				})
			end
			engine.wait(2)
		end
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
	CascadingWaves,
	DragonBreath,

	SpinningLotus,
	PhoenixFeathers,
}

return {
	name = "Bullet APIs showcase - Enhanced",
	description = "Beautiful parent-child bullet formations",
	IKnowWhatImDoing = true,
	spritesheets = { "./assets/EoSD_bullets.json" },

	run = function(engine)
		while true do
			-- Shuffle patterns
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
