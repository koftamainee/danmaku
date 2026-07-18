-- recursive function should be outside of return
local function table_deep_copy(tbl)
    local copy = {}
    for key, value in pairs(tbl) do
        if type(value) == "table" then
            copy[key] = table_deep_copy(value)
        else
            copy[key] = value
        end
    end
    return copy
end

return {
    map = function(array, callback)
        for i, value in ipairs(array) do
            array[i] = callback(value)
        end
        return array
    end,

    table_copy = function(tbl)
        local copy = {}
        for key, value in pairs(tbl) do
            copy[key] = value
        end
        return copy
    end,

    table_deep_copy = table_deep_copy,

    table_keys = function(tbl)
        local keys = {}
        for key, _ in pairs(tbl) do
            table.insert(keys, key)
        end
        return keys
    end,

    table_values = function(tbl)
        local values = {}
        for _, value in pairs(tbl) do
            table.insert(values, value)
        end
        return values
    end,

    table_merge = function(table1, table2)
        local result = {}
        for key, value in pairs(table1) do
            result[key] = value
        end
        for key, value in pairs(table2) do
            result[key] = value
        end
        return result
    end,

    table_merge_into = function(table1, table2)
        for key, value in pairs(table2) do
            table1[key] = value
        end
        return table1
    end
}