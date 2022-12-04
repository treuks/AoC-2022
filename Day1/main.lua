local function readAll(file)
    local f = assert(io.open(file, "rb"))
    local content = f:read("*all")
    f:close()
    return content
end

local function split (inputstr, sep)
    if sep == nil then
            sep = "%s"
    end
    local t = {}
    for str in string.gmatch(inputstr, "([^"..sep.."]+)") do
            table.insert(t, str)
    end
    return t
end

local function add(array)
    local ret = 0
    for key, value in pairs(array) do
        ret = ret + value
    end
    return ret
end

function spairs(t, order)
    -- collect the keys
    local keys = {}
    for k in pairs(t) do keys[#keys+1] = k end

    -- if order function given, sort by it by passing the table and keys a, b,
    -- otherwise just sort the keys 
    if order then
        table.sort(keys, function(a,b) return order(t, a, b) end)
    else
        table.sort(keys)
    end

    -- return the iterator function
    local i = 0
    return function()
        i = i + 1
        if keys[i] then
            return keys[i], t[keys[i]]
        end
    end
end

local input = readAll("input.txt")

local xd = input:gsub("\n\n", ":")
local xd2 = xd:gsub("\n", " ")
local xd3 = split(xd2, ":")

local integertable = {}

for key, value in pairs(xd3) do
    Xddd = split(value, " ")
    table.insert(integertable, add(Xddd))
end

for k,v in spairs(integertable, function(t,a,b) return t[b] < t[a] end) do
    print(k,v)
end
