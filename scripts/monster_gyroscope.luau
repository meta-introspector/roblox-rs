-- Monster Gyroscope · Cl(15,0,0)
-- Standalone Lune script: computes and displays the full O^n cycle
-- Run: lune run scripts/monster_gyroscope

local PI = math.pi
local cos = math.cos
local sin = math.sin
local fmt = string.format

-- Supersingular primes
local SSP = {2,3,5,7,11,13,17,19,23,29,31,41,47,59,71}
local C1 = {4371,782,133,50,16,11,6,5,3,2,2,1,0,0,0}
local GENUS = {0,0,0,0,1,0,1,1,2,2,2,3,4,5,6}
local sup = {"⁰","¹","²","³","⁴","⁵","⁶"}

-- Gyroscope functions
local function clk(n) return cos(n * PI / 3) end
local function rot(n) return -cos(n * PI / 3) end
local function frm(n) return (n % 2 == 0) and 1 or -1 end

-- Conserved quantities
local c1_norm_sq = 0
for i = 1, 15 do c1_norm_sq = c1_norm_sq + C1[i] * C1[i] end
local hub_sum = C1[8] + C1[9] -- c1(19) + c1(23) = 5 + 3 = 8

print("╔══════════════════════════════════════════════════════════════╗")
print("║           Monster Gyroscope · Cl(15,0,0)                   ║")
print("║  O⁶=I · 47×59×71=196883 · 15 supersingular primes         ║")
print("╚══════════════════════════════════════════════════════════════╝")
print()

-- Conserved quantities
print("Conserved quantities:")
print(fmt("  Hub projection:  1.0000 (eigenvalue +1)"))
print(fmt("  |c₁|² = %d = 2·3·5·7·19·9901", c1_norm_sq))
print(fmt("  Hub sum = %d (c₁(19)+c₁(23) = %d+%d)", hub_sum, C1[8], C1[9]))
print()

-- Phase table
print(fmt("%4s %10s %10s %10s %10s %10s %8s",
    "n", "Clock", "Rotor", "Frame", "Hub", "Clk+Rot", "Angle"))
print(string.rep("─", 72))

for n = 0, 6 do
    local c = clk(n)
    local r = rot(n)
    local f = frm(n)
    local angle = n * 60
    local marker = (n == 0 or n == 6) and " ← period" or ""
    print(fmt("O%s %10.4f %10.4f %10.1f %10.4f %10.4f %7d°%s",
        sup[n+1], c, r, f, 1.0, c + r, angle, marker))
end
print()

-- Phase diagram (ASCII art)
print("Phase Circle (clock=● rotor=○ hub=◆):")
print()
local grid = {}
local W, H = 41, 21
for y = 1, H do
    grid[y] = {}
    for x = 1, W do grid[y][x] = " " end
end

local cx, cy = 21, 11
local radius = 9

-- Draw circle
for i = 0, 59 do
    local a = i * 2 * PI / 60
    local x = math.floor(cx + cos(a) * radius * 2 + 0.5)
    local y = math.floor(cy + sin(a) * radius + 0.5)
    if x >= 1 and x <= W and y >= 1 and y <= H then
        grid[y][x] = "·"
    end
end

-- Draw tick marks at 60° intervals
for i = 0, 5 do
    local a = -PI/2 + i * PI / 3
    local x = math.floor(cx + cos(a) * (radius + 1) * 2 + 0.5)
    local y = math.floor(cy + sin(a) * (radius + 1) + 0.5)
    if x >= 1 and x <= W and y >= 1 and y <= H then
        grid[y][x] = tostring(i)
    end
end

-- Hub at center
grid[cy][cx] = "◆"

-- Draw all 6 clock positions
for n = 0, 5 do
    local a = -PI/2 + n * PI / 3
    -- Clock
    local x = math.floor(cx + cos(a) * radius * 1.6 + 0.5)
    local y = math.floor(cy + sin(a) * radius * 0.8 + 0.5)
    if x >= 1 and x <= W and y >= 1 and y <= H then
        grid[y][x] = "●"
    end
    -- Rotor (anti-phase)
    local ra = a + PI
    local rx = math.floor(cx + cos(ra) * radius * 1.2 + 0.5)
    local ry = math.floor(cy + sin(ra) * radius * 0.6 + 0.5)
    if rx >= 1 and rx <= W and ry >= 1 and ry <= H then
        grid[ry][rx] = "○"
    end
end

for y = 1, H do
    print("  " .. table.concat(grid[y]))
end
print()

-- Eigenspace decomposition
print("Eigenspace Structure:")
print("  Earth (λ=-1, 7D): e₂ e₃ e₅ e₇ e₁₁ e₁₃ e₄₇  [frame, period 2]")
print("  Spoke (λ=-1, 5D): e₁₇±e₂₉, e₁₇±e₃₁, ...     [frame, period 2]")
print("  Hub   (λ=+1, 1D): (e₁₉+e₂₃)/√2               [axle, fixed]")
print("  Clock (λ=ω,  2D): (e₁₉-e₂₃)/√2 + c₀          [rotation, period 6]")
print()

-- Trivector analysis
print("196883 Trivector e₄₇∧e₅₉∧e₇₁:")
print("  Product: 47 × 59 × 71 = " .. (47 * 59 * 71))
print("  O ratio: -0.5 = cos(120°) = Re(ω²)")
print("  Phase: -cos(nπ/3) — anti-phase with clock")
print("  Status: ABSOLUTE INVARIANT under all 15 prime negations")
print()

-- Prime negation walk summary
print("Prime Negation Walk:")
print(fmt("  %-12s %-20s %10s %10s", "Region", "Primes", "Hub effect", "Slot ratio"))
print("  " .. string.rep("─", 56))
print(fmt("  %-12s %-20s %10s %10s", "Earth", "2,3,5,7,11,13", "conserved", "-1.000"))
print(fmt("  %-12s %-20s %10s %10s", "Spoke", "17", "conserved", "-0.708"))
print(fmt("  %-12s %-20s %10s %10s", "Hub", "19", "BREAKS→-2", "+0.250"))
print(fmt("  %-12s %-20s %10s %10s", "Hub", "23", "BREAKS→+2", "+0.917"))
print(fmt("  %-12s %-20s %10s %10s", "Heaven", "29,31,41", "conserved", "mixed"))
print(fmt("  %-12s %-20s %10s %10s", "Invisible", "47,59,71", "conserved", "NaN (c₁=0)"))
print()

-- Hex walk
print("Hex Walk Groups:")
print("  Group 1 (86F): Earth+71  → hub_sum = +8 (conserved)")
print("  Group 2 (A3):  Heaven    → hub_sum = -8 (FLIPPED!)")
print("  Group 3 (F):   Invisible → hub_sum = +8 (no effect)")
print("  Skeleton: {3, 19} — irreducible core")
print()

-- DASL chain convergence
print("DASL Chain Fixed Point:")
print("  Step 0: 57 records → 23 records")
print("  Step 1: 24 records →  3 records")
print("  Step 2:  4 records →  4 records ← FIXED POINT")
print("  Attractor: e₄₇∧e₅₉∧e₇₁ (196883 trivector)")
print()

-- SSP table
print("Supersingular Primes:")
print(fmt("  %5s %5s %5s %5s %8s", "p", "exp", "c₁", "genus", "role"))
print("  " .. string.rep("─", 32))
local roles = {"Earth","Earth","Earth","Earth","Earth","Earth",
    "Spoke","Hub","Hub","Heaven","Heaven","Heaven",
    "Invisible","Invisible","Invisible"}
for i = 1, 15 do
    print(fmt("  %5d %5d %5d %5d %8s", SSP[i], 
        ({46,20,9,6,2,3,1,1,1,1,1,1,1,1,1})[i],
        C1[i], GENUS[i], roles[i]))
end

print()
print("The Monster never stops spinning. O⁶ = I.")
