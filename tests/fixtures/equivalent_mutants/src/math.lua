local function add_identity(x)
    return x + 0
end

local function sub_identity(x)
    return x - 0
end

local function mul_identity(x)
    return x * 1
end

local function div_identity(x)
    return x / 1
end

return {
    add_identity = add_identity,
    sub_identity = sub_identity,
    mul_identity = mul_identity,
    div_identity = div_identity,
}
