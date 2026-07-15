return {
    clamp = function(num, min, max)
        return math.max(min, math.min(max, num))
    end,

    lerp = function(a, b, t)
        return a + (b - a) * t
    end,

    distance = function(x1, y1, x2, y2)
        return math.sqrt((x2 - x1)^2 + (y2 - y1)^2)
    end,

    diff = function(a, b)
        local diff = (a - b) % (2 * math.pi)
        if diff > math.pi then diff = diff - 2 * math.pi end
        return diff
    end,

    normalize_angle = function(angle)
        angle = angle % (2 * math.pi)
        if angle > math.pi then angle = angle - 2 * math.pi end
        return angle
    end,

    deg = function(rad)
        return rad * (180 / math.pi)
    end,

    rad = function(deg)
        return deg * (math.pi / 180)
    end,

    angle = function(x1, y1, x2, y2)
        return math.atan(y2 - y1, x2 - x1)
    end,

    map_range = function(value, from_min, from_max, to_min, to_max)
        local t = (value - from_min) / (from_max - from_min)
        return to_min + (to_max - to_min) * t
    end,
}