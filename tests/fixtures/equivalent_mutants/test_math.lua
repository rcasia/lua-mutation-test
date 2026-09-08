local math_lib = dofile("src/math.lua")

assert(math_lib.add_identity(5) == 5)
assert(math_lib.sub_identity(5) == 5)
assert(math_lib.mul_identity(5) == 5)
assert(math_lib.div_identity(5) == 5)

print("OK")
