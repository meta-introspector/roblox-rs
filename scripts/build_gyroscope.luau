-- Monster Gyroscope · Cl(15,0,0) — Lune Roblox scene builder
-- Builds a .rbxl place file with the full gyroscope model
-- Run: lune run scripts/build_gyroscope

local roblox = require("@lune/roblox")
local fs = require("@lune/fs")

local PI = math.pi
local cos = math.cos
local sin = math.sin

local SSP = {2,3,5,7,11,13,17,19,23,29,31,41,47,59,71}
local CLOCK_R = 20
local ROTOR_R = 14
local FRAME_R = 50
local CENTER = Vector3.new(0, 20, 0)

-- Create place
local game = roblox.deserializePlace(roblox.serializePlace(roblox.Instance.new("DataModel")))
local workspace = game:GetService("Workspace")

local model = roblox.Instance.new("Model")
model.Name = "MonsterGyroscope"
model.Parent = workspace

-- Helper: create a part
local function part(name, props)
	local p = roblox.Instance.new("Part")
	p.Name = name
	p.Anchored = true
	p.TopSurface = Enum.SurfaceType.Smooth
	p.BottomSurface = Enum.SurfaceType.Smooth
	for k, v in props do p[k] = v end
	p.Parent = model
	return p
end

-- Hub (golden sphere)
part("Hub", {
	Shape = Enum.PartType.Ball,
	Size = Vector3.new(3, 3, 3),
	Position = CENTER,
	Material = Enum.Material.Neon,
	Color = Color3.fromRGB(255, 170, 0),
})

-- Clock hand positions for all 6 steps (frozen at step 0)
local ca = -PI / 2
local clockEnd = CENTER + Vector3.new(cos(ca) * CLOCK_R, 0, sin(ca) * CLOCK_R)
local clockMid = CENTER + (clockEnd - CENTER) / 2

part("ClockTip", {
	Shape = Enum.PartType.Ball,
	Size = Vector3.new(3, 3, 3),
	Position = clockEnd,
	Material = Enum.Material.Neon,
	Color = Color3.fromRGB(68, 136, 255),
})

part("ClockHand", {
	Size = Vector3.new(1, 1, CLOCK_R),
	CFrame = CFrame.new(clockMid, clockEnd),
	Material = Enum.Material.Neon,
	Color = Color3.fromRGB(68, 136, 255),
})

-- Rotor hand (anti-phase)
local ra = ca + PI
local rotorEnd = CENTER + Vector3.new(cos(ra) * ROTOR_R, 0, sin(ra) * ROTOR_R)
local rotorMid = CENTER + (rotorEnd - CENTER) / 2

part("RotorTip", {
	Shape = Enum.PartType.Ball,
	Size = Vector3.new(2.5, 2.5, 2.5),
	Position = rotorEnd,
	Material = Enum.Material.Neon,
	Color = Color3.fromRGB(255, 85, 85),
})

part("RotorHand", {
	Size = Vector3.new(0.8, 0.8, ROTOR_R),
	CFrame = CFrame.new(rotorMid, rotorEnd),
	Material = Enum.Material.Neon,
	Color = Color3.fromRGB(255, 85, 85),
})

-- 6 Earth prime frame pillars
for i = 1, 6 do
	local angle = (i - 1) * PI / 3
	local pos = Vector3.new(cos(angle) * FRAME_R, 20, sin(angle) * FRAME_R)
	local p = part("Frame_e" .. SSP[i], {
		Size = Vector3.new(2, 8, 2),
		Position = pos,
		Material = Enum.Material.Neon,
		Color = Color3.fromRGB(68, 204, 68),
	})
	p:SetAttribute("prime", SSP[i])
	p:SetAttribute("role", "Earth")
	p:SetAttribute("eigenvalue", -1)
end

-- 6 tick marks on orbit
for i = 0, 5 do
	local a = i * PI / 3
	part("Tick_" .. i, {
		Size = Vector3.new(1, 0.5, 1),
		Position = Vector3.new(cos(a) * CLOCK_R, 20, sin(a) * CLOCK_R),
		Material = Enum.Material.Neon,
		Color = Color3.fromRGB(60, 60, 100),
		Transparency = 0.5,
	})
end

-- Invisible prime markers (47, 59, 71) — small red spheres above hub
for i = 13, 15 do
	local offset = (i - 13) * 4 - 4
	part("Invisible_e" .. SSP[i], {
		Shape = Enum.PartType.Ball,
		Size = Vector3.new(1.5, 1.5, 1.5),
		Position = CENTER + Vector3.new(offset, 10, 0),
		Material = Enum.Material.Neon,
		Color = Color3.fromRGB(255, 85, 85),
		Transparency = 0.3,
	}):SetAttribute("prime", SSP[i])
end

-- Hub prime markers (19, 23) — orange spheres
for i, p in {19, 23} do
	local offset = (i - 1) * 3 - 1.5
	part("Hub_e" .. p, {
		Shape = Enum.PartType.Ball,
		Size = Vector3.new(1.5, 1.5, 1.5),
		Position = CENTER + Vector3.new(offset, -3, 0),
		Material = Enum.Material.Neon,
		Color = Color3.fromRGB(255, 200, 50),
	}):SetAttribute("prime", p)
end

-- Set attributes on the model with all the math
model:SetAttribute("description", "Monster Gyroscope Cl(15,0,0)")
model:SetAttribute("trivector", "e47∧e59∧e71 = 196883")
model:SetAttribute("o_ratio", -0.5)
model:SetAttribute("hub_sum", 8)
model:SetAttribute("c1_norm_sq", 19737810)
model:SetAttribute("period", 6)
model:SetAttribute("clock_formula", "cos(n*pi/3)")
model:SetAttribute("rotor_formula", "-cos(n*pi/3)")
model:SetAttribute("skeleton_pair", "3,19")

-- Add the animation script as a child
local script = roblox.Instance.new("Script")
script.Name = "GyroscopeAnimator"
script.Source = [[
-- Monster Gyroscope Animator
-- Runs the O^n cycle with TweenService
local TweenService = game:GetService("TweenService")
local model = script.Parent
local hub = model:FindFirstChild("Hub")
local clockTip = model:FindFirstChild("ClockTip")
local clockHand = model:FindFirstChild("ClockHand")
local rotorTip = model:FindFirstChild("RotorTip")
local rotorHand = model:FindFirstChild("RotorHand")

local PI = math.pi
local cos = math.cos
local sin = math.sin
local CENTER = hub.Position
local CLOCK_R = 20
local ROTOR_R = 14
local STEP_TIME = 0.8
local tweenInfo = TweenInfo.new(STEP_TIME, Enum.EasingStyle.Quad, Enum.EasingDirection.InOut)

local frameParts = {}
for _, c in model:GetChildren() do
	if c.Name:match("^Frame_e") then table.insert(frameParts, c) end
end

local step = 0
while true do
	local angle = -PI/2 + step * PI/3
	local cx, cz = cos(angle)*CLOCK_R, sin(angle)*CLOCK_R
	local clockPos = CENTER + Vector3.new(cx, 0, cz)
	local rx, rz = cos(angle+PI)*ROTOR_R, sin(angle+PI)*ROTOR_R
	local rotorPos = CENTER + Vector3.new(rx, 0, rz)

	TweenService:Create(clockTip, tweenInfo, {Position = clockPos}):Play()
	TweenService:Create(clockHand, tweenInfo, {
		CFrame = CFrame.new(CENTER + Vector3.new(cx/2, 0, cz/2), clockPos),
	}):Play()
	TweenService:Create(rotorTip, tweenInfo, {Position = rotorPos}):Play()
	TweenService:Create(rotorHand, tweenInfo, {
		CFrame = CFrame.new(CENTER + Vector3.new(rx/2, 0, rz/2), rotorPos),
	}):Play()

	local frameColor = (step % 2 == 0) and Color3.fromRGB(68,204,68) or Color3.fromRGB(204,68,68)
	for _, p in frameParts do
		TweenService:Create(p, tweenInfo, {Color = frameColor}):Play()
	end

	task.wait(STEP_TIME + 0.2)
	step = (step + 1) % 6
end
]]
script.Parent = model

-- Serialize to .rbxl
local output = "output/roblox/MonsterGyroscope.rbxl"
local data = roblox.serializePlace(game)
fs.writeFile(output, data)

print("Built MonsterGyroscope.rbxl")
print("  Parts: " .. #model:GetChildren())
print("  Attributes: " .. #model:GetAttributes())
print("  Output: " .. output)
print("  Open in Roblox Studio to see the animated gyroscope")
