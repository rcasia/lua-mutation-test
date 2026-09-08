local function nested(x, y)
    if x then
        if y then
            return x and y
        else
            return x
        end
    elseif y then
        while true do
            if y then
                break
            end
        end
    end

    for i = 1, 3 do
        for j = i, 3 do
            if i == j then
                return i + j
            end
        end
    end

    repeat
        return nil
    until x and y
end

return nested
