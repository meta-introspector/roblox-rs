-- Monster Gyroscope Builder · ServerScriptService
-- Builds the 3D gyroscope model and runs the O^n animation loop
local Config = require(game.ReplicatedStorage.GyroscopeConfig)
local TweenService = game:GetService("TweenService")

local PI = math.pi
local cos = math.cos
local sin = math.sin
local CENTER = Config.CENTER

-- Build model
local model = Instance.new("Model")
model.Name = "MonsterGyroscope"
model.Parent = workspace

local function part(name, props)
	local p = Instance.new("Part")
	p.Name = name
	p.Anchored = true
	p.TopSurface = Enum.SurfaceType.Smooth
	p.BottomSurface = Enum.SurfaceType.Smooth
	for k, v in pairs(props) do p[k] = v end
	p.Parent = model
	return p
end

local function label(parent, text, color)
	local gui = Instance.new("BillboardGui")
	gui.Size = UDim2.new(0, 160, 0, 30)
	gui.StudsOffset = Vector3.new(0, 3, 0)
	gui.Parent = parent
	local lbl = Instance.new("TextLabel")
	lbl.Size = UDim2.new(1, 0, 1, 0)
	lbl.BackgroundTransparency = 1
	lbl.Text = text
	lbl.TextColor3 = color
	lbl.TextScaled = true
	lbl.Font = Enum.Font.RobotoMono
	lbl.Parent = gui
	return lbl
end

-- Hub
local hub = part("Hub", {
	Shape = Enum.PartType.Ball,
	Size = Vector3.new(3, 3, 3),
	Position = CENTER,
	Material = Enum.Material.Neon,
	Color = Config.COLORS.Hub,
})
label(hub, "Hub (λ=+1)", Config.COLORS.Hub)

-- Clock
local clockTip = part("ClockTip", {
	Shape = Enum.PartType.Ball,
	Size = Vector3.new(3, 3, 3),
	Position = CENTER + Vector3.new(0, 0, -Config.CLOCK_RADIUS),
	Material = Enum.Material.Neon,
	Color = Config.COLORS.Clock,
})
label(clockTip, "Clock cos(nπ/3)", Config.COLORS.Clock)

local clockHand = part("ClockHand", {
	Size = Vector3.new(1, 1, Config.CLOCK_RADIUS),
	Material = Enum.Material.Neon,
	Color = Config.COLORS.Clock,
})

-- Rotor
local rotorTip = part("RotorTip", {
	Shape = Enum.PartType.Ball,
	Size = Vector3.new(2.5, 2.5, 2.5),
	Position = CENTER + Vector3.new(0, 0, Config.ROTOR_RADIUS),
	Material = Enum.Material.Neon,
	Color = Config.COLORS.Rotor,
})
label(rotorTip, "e₄₇∧e₅₉∧e₇₁ = 196883", Config.COLORS.Rotor)

local rotorHand = part("RotorHand", {
	Size = Vector3.new(0.8, 0.8, Config.ROTOR_RADIUS),
	Material = Enum.Material.Neon,
	Color = Config.COLORS.Rotor,
})

-- Frame pillars (6 Earth primes)
local frameParts = {}
for i = 1, 6 do
	local angle = (i - 1) * PI / 3
	local pos = Vector3.new(cos(angle) * Config.FRAME_RADIUS, 20, sin(angle) * Config.FRAME_RADIUS)
	local p = part("Frame_e" .. Config.SSP[i], {
		Size = Vector3.new(2, 8, 2),
		Position = pos,
		Material = Enum.Material.Neon,
		Color = Config.COLORS.FramePos,
	})
	label(p, "e" .. Config.SSP[i], Config.COLORS.FramePos)
	table.insert(frameParts, p)
end

-- Orbit ticks
for i = 0, 5 do
	local a = i * PI / 3
	part("Tick_" .. i, {
		Size = Vector3.new(1, 0.5, 1),
		Position = Vector3.new(cos(a) * Config.CLOCK_RADIUS, 20, sin(a) * Config.CLOCK_RADIUS),
		Color = Config.COLORS.Tick,
		Transparency = 0.5,
	})
end

-- Invisible prime markers
for i = 13, 15 do
	local p = part("Invisible_e" .. Config.SSP[i], {
		Shape = Enum.PartType.Ball,
		Size = Vector3.new(1.5, 1.5, 1.5),
		Position = CENTER + Vector3.new((i - 14) * 4, 10, 0),
		Material = Enum.Material.Neon,
		Color = Config.COLORS.Invisible,
		Transparency = 0.3,
	})
	label(p, "e" .. Config.SSP[i], Config.COLORS.Invisible)
end

-- Info display
local infoPart = part("InfoAnchor", {
	Size = Vector3.new(0.1, 0.1, 0.1),
	Position = CENTER + Vector3.new(0, 18, 0),
	Transparency = 1,
})
local infoLabel = label(infoPart, "", Color3.fromRGB(200, 200, 220))
infoLabel.Size = UDim2.new(1, 0, 1, 0)
infoLabel.Parent.Size = UDim2.new(0, 500, 0, 100)

-- Animation loop
local tweenInfo = TweenInfo.new(Config.STEP_TIME, Enum.EasingStyle.Quad, Enum.EasingDirection.InOut)
local sup = {"⁰","¹","²","³","⁴","⁵"}
local step = 0

while true do
	local angle = -PI/2 + step * PI/3
	local cx, cz = cos(angle) * Config.CLOCK_RADIUS, sin(angle) * Config.CLOCK_RADIUS
	local clockPos = CENTER + Vector3.new(cx, 0, cz)
	local rx, rz = cos(angle + PI) * Config.ROTOR_RADIUS, sin(angle + PI) * Config.ROTOR_RADIUS
	local rotorPos = CENTER + Vector3.new(rx, 0, rz)

	-- Tween clock
	TweenService:Create(clockTip, tweenInfo, {Position = clockPos}):Play()
	TweenService:Create(clockHand, tweenInfo, {
		CFrame = CFrame.new(CENTER:Lerp(clockPos, 0.5), clockPos),
		Size = Vector3.new(1, 1, (clockPos - CENTER).Magnitude),
	}):Play()

	-- Tween rotor
	TweenService:Create(rotorTip, tweenInfo, {Position = rotorPos}):Play()
	TweenService:Create(rotorHand, tweenInfo, {
		CFrame = CFrame.new(CENTER:Lerp(rotorPos, 0.5), rotorPos),
		Size = Vector3.new(0.8, 0.8, (rotorPos - CENTER).Magnitude),
	}):Play()

	-- Frame color
	local f = Config.frame(step)
	local fc = f > 0 and Config.COLORS.FramePos or Config.COLORS.FrameNeg
	for _, p in ipairs(frameParts) do
		TweenService:Create(p, tweenInfo, {Color = fc}):Play()
	end

	-- Info
	local cv = Config.clock(step)
	infoLabel.Text = string.format(
		"Monster Gyroscope · Cl(15,0,0)\nO%s  %d°  clock=%.3f  rotor=%.3f  frame=%+d\n47×59×71=196883  hub=1.0  |c₁|²=19,737,810  hub∑=8",
		sup[step + 1], step * 60, cv, -cv, f
	)

	task.wait(Config.STEP_TIME + 0.2)
	step = (step + 1) % 6
end
