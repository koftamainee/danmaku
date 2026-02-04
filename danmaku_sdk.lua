---@meta danmaku_api

---@class Bullet
---@field x number
---@field y number
---@field angle number
---@field speed number
---@field lifetime number

---Spawns a bullet
---@param x number
---@param y number
---@param angle number radians
---@param speed number
---@param lifetime number frames
function spawn_bullet(x, y, angle, speed, lifetime) end

---Waits N frames
---@param frames number
function wait(frames) end

---Yields execution for one frame
function yield() end

---@param id integer
---@param angle number radians
function bullet_set_angle(id, angle) end

---@param id integer
---@param speed number
function bullet_set_speed(id, speed) end

---@param id integer
---@param vx number
---@param vy number
function bullet_set_velocity(id, vx, vy) end

---@param id integer
---@param ds number
function bullet_add_speed(id, ds) end

---@param id integer
---@param d_angle number radians
function bullet_rotate(id, d_angle) end
