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

local input = readAll("input.txt")

local xd = input:gsub("\n\n", ":")
local xd2 = xd:gsub("\n", " ")
local xd3 = split(xd2, ":")

local integertable = {}

for key, value in pairs(xd3) do
    Xddd = split(value, " ")
    table.insert(integertable, add(Xddd))
end
print(math.max(table.unpack(integertable)))
