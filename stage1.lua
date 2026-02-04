local screen_width = 640
local screen_height = 480
local center_x = screen_width / 2
local center_y = screen_height / 2

local edge_margin = 20
local bullets = {}
local bullet_count_per_edge = 15
local spawn_delay = 2 -- frames between bullets
local frames_to_center = 20 -- frames to reach center
local collapse_speed = 5
local flower_speed = 3
local spiral_bullet_count = 100
local spiral_speed = 2
local spiral_angle_step = math.pi / 30

local function angle_to(x1, y1, x2, y2)
	return math.atan2(y2 - y1, x2 - x1)
end

-- Move bullets to center
local function move_to_center()
	for _, b in ipairs(bullets) do
		local vx = (center_x - b.x) / frames_to_center
		local vy = (center_y - b.y) / frames_to_center
		bullet_set_velocity(b.id, vx, vy)
	end
end

-- Collapse bullets inward
local function collapse_bullets()
	for _, b in ipairs(bullets) do
		local angle = angle_to(b.x, b.y, center_x, center_y)
		bullet_set_velocity(b.id, collapse_speed * math.cos(angle), collapse_speed * math.sin(angle))
	end
end

local lifetime = 400 -- new bullet lifetime

-- Spawn bullets sequentially along edges
local function spawn_edge_bullets_chain()
	bullets = {}

	local function add_bullet(x, y)
		local id = spawn_bullet(x, y, 0, 0, lifetime)
		bullets[#bullets + 1] = { id = id, x = x, y = y, lifetime = lifetime }
	end

	-- Top edge
	for i = 1, bullet_count_per_edge do
		local x = edge_margin + (screen_width - 2 * edge_margin) * (i - 1) / (bullet_count_per_edge - 1)
		local y = edge_margin
		add_bullet(x, y)
		wait(2)
	end

	-- Right edge
	for i = 1, bullet_count_per_edge do
		local x = screen_width - edge_margin
		local y = edge_margin + (screen_height - 2 * edge_margin) * (i - 1) / (bullet_count_per_edge - 1)
		add_bullet(x, y)
		wait(1)
	end

	-- Bottom edge
	for i = bullet_count_per_edge, 1, -1 do
		local x = edge_margin + (screen_width - 2 * edge_margin) * (i - 1) / (bullet_count_per_edge - 1)
		local y = screen_height - edge_margin
		add_bullet(x, y)
		wait(1)
	end

	-- Left edge
	for i = bullet_count_per_edge, 1, -1 do
		local x = edge_margin
		local y = edge_margin + (screen_height - 2 * edge_margin) * (i - 1) / (bullet_count_per_edge - 1)
		add_bullet(x, y)
		wait(1)
	end
end
function DanmakuWheel(x, y)
	local angle = 0.0
	for frame = 1, 120 do
		for arm = 0, 5 do
			local arm_angle = angle + (arm * math.pi / 3)
			for bullet = 1, 3 do
				local speed = 1.5 + bullet * 0.5
				spawn_bullet(x, y, arm_angle, speed, 320 + 60)
			end
		end
		angle = angle + 0.12
		yield()
	end
end

-- Stage 1 loop
function Stage1()
	while true do
		spawn_edge_bullets_chain()
		wait(30)
		move_to_center()
		wait(frames_to_center) -- reach center
		collapse_bullets()
		-- TODO: kill all bullets
		DanmakuWheel(center_x, center_y)
		wait(180) -- let them expand
	end
end
