local calc = dofile("src/calc.lua")

assert(calc.add(2, 3) == 5)
assert(calc.subtract(5, 3) == 2)

print("OK")
