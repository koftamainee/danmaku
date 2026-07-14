local _print = print

local log_context = {
    prefix = "unknown",
    do_log = true,
}

return {

    setctx = function(context)
        log_context = context
    end,

    log_scenario = function(message) 
        if log_context.do_log == true then
            _print("[scenario][" .. log_context.prefix .. "] " .. message)
        end
    end,

    log_mod = function(message) 
        if log_context.do_log == true then
            _print("[mod][" .. log_context.prefix .. "] " .. message)
        end
    end
    
}