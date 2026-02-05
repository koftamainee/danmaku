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

	local function add_bullet(x, y, sprite)
		local id = spawn_bullet(x, y, 0, 0, lifetime, sprite)
		bullets[#bullets + 1] = { id = id, x = x, y = y, lifetime = lifetime }
	end

	-- Color sequence for edges
	local edge_colors = {
		"bullet_blue",
		"bullet_cyan",
		"bullet_green",
		"bullet_light_green",
		"bullet_yellow",
		"bullet_orange",
		"bullet_red",
		"bullet_dark_red",
		"bullet_purple",
		"bullet_pink",
		"bullet_white",
	}

	-- Top edge - blue to cyan gradient
	for i = 1, bullet_count_per_edge do
		local x = edge_margin + (screen_width - 2 * edge_margin) * (i - 1) / (bullet_count_per_edge - 1)
		local y = edge_margin
		local color_idx = math.floor((i - 1) * #edge_colors / bullet_count_per_edge) + 1
		add_bullet(x, y, edge_colors[color_idx])
		wait(2)
	end

	-- Right edge - green gradient
	for i = 1, bullet_count_per_edge do
		local x = screen_width - edge_margin
		local y = edge_margin + (screen_height - 2 * edge_margin) * (i - 1) / (bullet_count_per_edge - 1)
		local color_idx = math.floor((i - 1) * #edge_colors / bullet_count_per_edge) + 1
		add_bullet(x, y, edge_colors[color_idx % #edge_colors + 1])
		wait(1)
	end

	-- Bottom edge - yellow to orange gradient
	for i = bullet_count_per_edge, 1, -1 do
		local x = edge_margin + (screen_width - 2 * edge_margin) * (i - 1) / (bullet_count_per_edge - 1)
		local y = screen_height - edge_margin
		local color_idx = math.floor((bullet_count_per_edge - i) * #edge_colors / bullet_count_per_edge) + 1
		add_bullet(x, y, edge_colors[color_idx])
		wait(1)
	end

	-- Left edge - red to purple gradient
	for i = bullet_count_per_edge, 1, -1 do
		local x = edge_margin
		local y = edge_margin + (screen_height - 2 * edge_margin) * (i - 1) / (bullet_count_per_edge - 1)
		local color_idx = math.floor((bullet_count_per_edge - i) * #edge_colors / bullet_count_per_edge) + 1
		add_bullet(x, y, edge_colors[color_idx % #edge_colors + 1])
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
				local sprite = "bullet_yellow"
				if bullet == 2 then
					sprite = "bullet_orange"
				end
				if bullet == 3 then
					sprite = "bullet_red"
				end
				spawn_bullet(x, y, arm_angle, speed, 320 + 60, sprite)
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
