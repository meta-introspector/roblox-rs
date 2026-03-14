-- Build Monster Gyroscope .rbxl using Lune's roblox API
-- Run: nix develop -c lune run scripts/build_gyroscope
local roblox = require("@lune/roblox")
local fs = require("@lune/fs")
local Instance = roblox.Instance

-- Inject roblox datatypes as globals (Lune doesn't have them by default)
local Vector3 = roblox.Vector3
local CFrame = roblox.CFrame
local Color3 = roblox.Color3
local Enum = roblox.Enum

local PI = math.pi
local cos = math.cos
local sin = math.sin

local SSP = {2,3,5,7,11,13,17,19,23,29,31,41,47,59,71}
local CLOCK_R, ROTOR_R, FRAME_R = 20, 14, 50

local game = Instance.new("DataModel")
local ws = game:GetService("Workspace")
local rs = game:GetService("ReplicatedStorage")
local sss = game:GetService("ServerScriptService")

local model = Instance.new("Model")
model.Name = "MonsterGyroscope"
model.Parent = ws

local function part(name, props)
	local p = Instance.new("Part")
	p.Name = name
	p.Anchored = true
	for k, v in props do p[k] = v end
	p.Parent = model
	return p
end

local CENTER = Vector3.new(0, 20, 0)

-- Hub
part("Hub", { Shape = Enum.PartType.Ball, Size = Vector3.new(3,3,3), Position = CENTER, Material = Enum.Material.Neon, Color = Color3.fromRGB(255,170,0) })

-- Clock at step 0
local ca = -PI/2
local ce = CENTER + Vector3.new(cos(ca)*CLOCK_R, 0, sin(ca)*CLOCK_R)
part("ClockTip", { Shape = Enum.PartType.Ball, Size = Vector3.new(3,3,3), Position = ce, Material = Enum.Material.Neon, Color = Color3.fromRGB(68,136,255) })
part("ClockHand", { Size = Vector3.new(1,1,CLOCK_R), CFrame = CFrame.lookAt(CENTER:Lerp(ce,0.5), ce), Material = Enum.Material.Neon, Color = Color3.fromRGB(68,136,255) })

-- Rotor
local re = CENTER + Vector3.new(cos(ca+PI)*ROTOR_R, 0, sin(ca+PI)*ROTOR_R)
part("RotorTip", { Shape = Enum.PartType.Ball, Size = Vector3.new(2.5,2.5,2.5), Position = re, Material = Enum.Material.Neon, Color = Color3.fromRGB(255,85,85) })
part("RotorHand", { Size = Vector3.new(0.8,0.8,ROTOR_R), CFrame = CFrame.lookAt(CENTER:Lerp(re,0.5), re), Material = Enum.Material.Neon, Color = Color3.fromRGB(255,85,85) })

-- Frame pillars
for i = 1, 6 do
	local a = (i-1)*PI/3
	part("Frame_e"..SSP[i], { Size = Vector3.new(2,8,2), Position = Vector3.new(cos(a)*FRAME_R, 20, sin(a)*FRAME_R), Material = Enum.Material.Neon, Color = Color3.fromRGB(68,204,68) })
end

-- Ticks
for i = 0, 5 do
	local a = i*PI/3
	part("Tick_"..i, { Size = Vector3.new(1,0.5,1), Position = Vector3.new(cos(a)*CLOCK_R, 20, sin(a)*CLOCK_R), Color = Color3.fromRGB(60,60,100), Transparency = 0.5 })
end

-- Invisible markers
for i = 13, 15 do
	part("Inv_e"..SSP[i], { Shape = Enum.PartType.Ball, Size = Vector3.new(1.5,1.5,1.5), Position = CENTER + Vector3.new((i-14)*4, 10, 0), Material = Enum.Material.Neon, Color = Color3.fromRGB(255,85,85), Transparency = 0.3 })
end

-- Embed config module
local cfg = Instance.new("ModuleScript")
cfg.Name = "GyroscopeConfig"
cfg.Source = fs.readFile("src/ReplicatedStorage/GyroscopeConfig.lua")
cfg.Parent = rs

-- Embed TweenServiceV2
local tsv2 = Instance.new("ModuleScript")
tsv2.Name = "TweenServiceV2"
tsv2.Source = fs.readFile("vendor/TweenServiceV2/ReplicatedTweening.luau")
tsv2.Parent = rs

-- Embed server script
local srv = Instance.new("Script")
srv.Name = "GyroscopeBuilder"
srv.Source = fs.readFile("src/ServerScriptService/GyroscopeBuilder.server.lua")
srv.Parent = model

-- Embed client HUD
local sg = game:GetService("StarterGui")
local sgui = Instance.new("ScreenGui")
sgui.Name = "GyroscopeHUD"
sgui.Parent = sg
local hud = Instance.new("LocalScript")
hud.Name = "HUD"
hud.Source = fs.readFile("src/StarterGui/GyroscopeUI/HUD.client.lua")
hud.Parent = sgui

-- Model attributes
model:SetAttribute("description", "Monster Gyroscope Cl(15,0,0)")
model:SetAttribute("trivector", "e47^e59^e71=196883")
model:SetAttribute("o_ratio", -0.5)
model:SetAttribute("hub_sum", 8)
model:SetAttribute("period", 6)

-- Write
if not fs.isDir("output") then fs.writeDir("output") end
if not fs.isDir("output/roblox") then fs.writeDir("output/roblox") end
local data = roblox.serializePlace(game)
fs.writeFile("output/roblox/MonsterGyroscope.rbxl", data)

print("Built output/roblox/MonsterGyroscope.rbxl")
print("  " .. #model:GetChildren() .. " parts in model")
print("  " .. #data .. " bytes")
print("  Config + TweenServiceV2 + Animator + HUD embedded")
