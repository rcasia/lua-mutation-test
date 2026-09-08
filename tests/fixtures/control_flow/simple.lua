local function classify(x)
    if x > 0 then
        return "positive"
    elseif x < 0 then
        return "negative"
    else
        return "zero"
    end
end

local function sum(n)
    local total = 0
    for i = 1, n do
        total = total + i
    end
    return total
end

local function countdown(n)
    while n > 0 do
        n = n - 1
    end
    return n
end

local function retry()
    local attempts = 0
    repeat
        attempts = attempts + 1
    until attempts >= 3
    return attempts
end

return {
    classify = classify,
    sum = sum,
    countdown = countdown,
    retry = retry,
}
