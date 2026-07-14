local Xoshiro128 = {}
Xoshiro128.__index = Xoshiro128

function Xoshiro128:new(seed)
    local function splitmix(state)
        state = (state + 0x9e3779b9) & 0xffffffff
        local z = state
        z = ((z ~ (z >> 16)) * 0x85ebca6b) & 0xffffffff
        z = ((z ~ (z >> 13)) * 0xc2b2ae35) & 0xffffffff
        z = (z ~ (z >> 16)) & 0xffffffff
        return z, state
    end

    local s0, s = splitmix(seed)
    local s1, s = splitmix(s)
    local s2, s = splitmix(s)
    local s3, _ = splitmix(s)

    return setmetatable({ s0 = s0, s1 = s1, s2 = s2, s3 = s3 }, self)
end

function Xoshiro128:_rotl(x, k)
    return ((x << k) | (x >> (32 - k))) & 0xffffffff
end

function Xoshiro128:_next()
    local result = (self.s0 + self.s3) & 0xffffffff
    local t = (self.s1 << 9) & 0xffffffff

    self.s2 = self.s2 ~ self.s0
    self.s3 = self.s3 ~ self.s1
    self.s1 = self.s1 ~ self.s2
    self.s0 = self.s0 ~ self.s3

    self.s2 = self.s2 ~ t
    self.s3 = self:_rotl(self.s3, 11)

    return result
end

function Xoshiro128:rangef(min, max)
    return min + (self:_next() / 0xffffffff) * (max - min)
end

function Xoshiro128:rangei(min, max)
    return min + (self:_next() % (max - min + 1))
end

function Xoshiro128:sign()
    return self:rangei(0, 1) == 0 and -1 or 1
end

local gen

return {
    run = function(seed)
        gen = Xoshiro128:new(seed)
    end,

    rng = {
        float = function()
            return gen:_next() / 0xffffffff
        end,

        rangef = function(min, max)
            return gen:rangef(min, max)
        end,

        rangei = function(min, max)
            return gen:rangei(min, max)
        end,

        angle = function()
            return gen:rangef(0, 2 * math.pi)
        end,

        angle_range = function(min, max)
            return gen:rangef(min, max)
        end,

        bool = function()
            return gen:rangei(0, 1) == 0
        end,

        choice = function(list)
            return list[gen:rangei(1, #list)]
        end,

        shuffle = function(list)
            for i = #list, 2, -1 do
                local j = gen:rangei(1, i)
                list[i], list[j] = list[j], list[i]
            end
            return list
        end,

        sample = function(list, n)
            local copy = {}
            for i = 1, #list do copy[i] = list[i] end
            for i = #copy, 2, -1 do
                local j = gen:rangei(1, i)
                copy[i], copy[j] = copy[j], copy[i]
            end
            local result = {}
            local count = math.min(n, #copy)
            for i = 1, count do
                result[i] = copy[i]
            end
            return result
        end,

        direction = function()
            local a = gen:rangef(0, 2 * math.pi)
            return { x = math.cos(a), y = math.sin(a) }
        end,

        sign = function()
            return gen:sign()
        end
    }
}