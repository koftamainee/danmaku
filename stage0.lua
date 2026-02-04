BASE_LIFETIME = 320

function BloomingFlower(x, y)
	local base_angle = 0.0
	for wave = 1, 8 do
		for petal = 0, 11 do
			local angle = base_angle + (petal * math.pi / 6)
			local speed = 2.0 + (wave * 0.3)
			spawn_bullet(x, y, angle, speed, BASE_LIFETIME)
		end
		base_angle = base_angle + 0.2
		wait(3)
	end
end

function SpinningCross(x, y)
	local angle = 0.0
	for frame = 1, 90 do
		for i = -5, 5 do
			spawn_bullet(x, y, angle, 2.0, BASE_LIFETIME)
			spawn_bullet(x, y, angle + math.pi, 2.0, BASE_LIFETIME)
		end
		for i = -5, 5 do
			spawn_bullet(x, y, angle + math.pi / 2, 2.0, BASE_LIFETIME)
			spawn_bullet(x, y, angle + math.pi * 3 / 2, 2.0, BASE_LIFETIME)
		end
		angle = angle + 0.08
		wait(1)
	end
end

function SakuraScatter(x, y)
	for burst = 1, 12 do
		local burst_angle = (burst - 1) * math.pi / 6
		for ring = 1, 3 do
			local num_bullets = 8 + ring * 4
			for i = 0, num_bullets - 1 do
				local angle = burst_angle + (i * 2 * math.pi / num_bullets)
				local speed = 1.0 + ring * 0.8
				spawn_bullet(x, y, angle, speed, BASE_LIFETIME + 40)
			end
		end
		wait(4)
	end
end

function DanmakuWheel(x, y)
	local angle = 0.0
	for frame = 1, 120 do
		for arm = 0, 5 do
			local arm_angle = angle + (arm * math.pi / 3)
			for bullet = 1, 3 do
				local speed = 1.5 + bullet * 0.5
				spawn_bullet(x, y, arm_angle, speed, BASE_LIFETIME + 60)
			end
		end
		angle = angle + 0.12
		yield()
	end
end

function ExpandingRings(x, y)
	for wave = 1, 6 do
		local num_bullets = 16 + wave * 4
		for i = 0, num_bullets - 1 do
			local angle = (i * 2 * math.pi / num_bullets) + (wave * 0.3)
			spawn_bullet(x, y, angle, 1.5, BASE_LIFETIME + 80)
		end
		wait(8)
	end
end

function SpiralGalaxy(x, y)
	local angle = 0.0
	for frame = 1, 100 do
		for i = 1, 3 do
			spawn_bullet(x, y, angle + (i * 0.5), 2.0, BASE_LIFETIME + 60)
		end
		for i = 1, 3 do
			spawn_bullet(x, y, -angle + (i * 0.5), 2.0, BASE_LIFETIME + 60)
		end
		angle = angle + 0.18
		yield()
	end
end

function ButterflyWings(x, y)
	for phase = 1, 60 do
		local spread = math.sin(phase * 0.1) * math.pi / 3
		for i = 0, 5 do
			local angle = -math.pi / 2 - spread + (i * 0.2)
			spawn_bullet(x, y, angle, 2.2, BASE_LIFETIME)
		end
		for i = 0, 5 do
			local angle = -math.pi / 2 + spread - (i * 0.2)
			spawn_bullet(x, y, angle, 2.2, BASE_LIFETIME)
		end
		for i = 0, 5 do
			local angle = math.pi / 2 + spread - (i * 0.2)
			spawn_bullet(x, y, angle, 2.2, BASE_LIFETIME)
		end
		for i = 0, 5 do
			local angle = math.pi / 2 - spread + (i * 0.2)
			spawn_bullet(x, y, angle, 2.2, BASE_LIFETIME)
		end
		yield()
	end
end

function Starburst(x, y)
	for burst = 1, 8 do
		local points = 5
		for point = 0, points - 1 do
			local base_angle = (point * 2 * math.pi / points) + (burst * 0.4)
			for i = 1, 5 do
				spawn_bullet(x, y, base_angle, 1.0 + i * 0.4, BASE_LIFETIME + 80)
			end
			spawn_bullet(x, y, base_angle + 0.15, 2.0, BASE_LIFETIME)
			spawn_bullet(x, y, base_angle - 0.15, 2.0, BASE_LIFETIME)
		end
		wait(6)
	end
end

function CascadingWaves(x, y)
	local offset = 0.0
	for wave = 1, 15 do
		for i = 0, 23 do
			local angle = (i * 2 * math.pi / 24) + offset
			local speed = 1.8 + math.sin(wave * 0.5) * 0.5
			spawn_bullet(x, y, angle, speed, BASE_LIFETIME + 60)
		end
		offset = offset + 0.26
		wait(3)
	end
end

function DragonBreath(x, y)
	local direction = -math.pi / 2
	for _ = 1, 80 do
		local spread = math.pi / 4
		for i = -8, 8 do
			local angle = direction + (i * spread / 16)
			local speed = 2.5 + math.abs(i) * 0.1
			spawn_bullet(x, y, angle, speed, BASE_LIFETIME + 80)
		end
		direction = direction + 0.05
		yield()
	end
end

function Stage1()
	local patterns = {
		BloomingFlower,
		SpinningCross,
		SakuraScatter,
		DanmakuWheel,
		ExpandingRings,
		SpiralGalaxy,
		ButterflyWings,
		Starburst,
		CascadingWaves,
		DragonBreath,
	}

	local function shuffle(tbl)
		for i = #tbl, 2, -1 do
			local j = math.random(1, i)
			tbl[i], tbl[j] = tbl[j], tbl[i]
		end
	end

	while true do
		shuffle(patterns)
		for i = 1, #patterns do
			patterns[i](320, 100)
			wait(20)
		end
		wait(40)
	end
end
