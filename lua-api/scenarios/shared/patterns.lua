return {
    type = "module"
    -- in module you can define shared code. can be loaded to other scenes
    -- using danmaku.import("module_path")

    blooming_flower = function(danmaku, x, y, bullet1, bullet2, bullet3, lifetime)
	local base_angle = 0
	for wave = 1, 8 do
		for petal = 0, 11 do
			local sprite = (wave % 3 == 0) and bullet1 or (wave % 3 == 1) and bullet2 or bullet3

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
end
}