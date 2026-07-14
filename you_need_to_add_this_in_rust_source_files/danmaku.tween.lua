return {
    linear = function(t)
        return t
    end,

    quad_in = function(t)
        return t * t
    end,

    quad_out = function(t)
        return t * (2 - t)
    end,

    quad_in_out = function(t)
        if t < 0.5 then
            return 2 * t * t
        else
            return -1 + (4 - 2 * t) * t
        end
    end,

    cubic_in = function(t)
        return t * t * t
    end,

    cubic_out = function(t)
        local t1 = t - 1
        return t1 * t1 * t1 + 1
    end,

    bounce = function(t)
        if t < 1 / 2.75 then
            return 7.5625 * t * t
        elseif t < 2 / 2.75 then
            t = t - 1.5 / 2.75
            return 7.5625 * t * t + 0.75
        elseif t < 2.5 / 2.75 then
            t = t - 2.25 / 2.75
            return 7.5625 * t * t + 0.9375
        else
            t = t - 2.625 / 2.75
            return 7.5625 * t * t + 0.984375
        end
    end,

    elastic = function(t)
        if t == 0 or t == 1 then
            return t
        end
        return -math.pow(2, 10 * (t - 1)) * math.sin((t - 1.075) * 2 * math.pi / 0.3)
    end,
}