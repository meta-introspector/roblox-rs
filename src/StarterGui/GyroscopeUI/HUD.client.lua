-- Monster Gyroscope HUD · StarterGui
-- Shows phase table and conserved quantities overlay
local Config = require(game.ReplicatedStorage.GyroscopeConfig)

local gui = Instance.new("ScreenGui")
gui.Name = "GyroscopeHUD"
gui.Parent = script.Parent

local frame = Instance.new("Frame")
frame.Size = UDim2.new(0, 300, 0, 200)
frame.Position = UDim2.new(1, -310, 0, 10)
frame.BackgroundColor3 = Color3.fromRGB(10, 10, 18)
frame.BackgroundTransparency = 0.2
frame.BorderSizePixel = 0
frame.Parent = gui

local title = Instance.new("TextLabel")
title.Size = UDim2.new(1, 0, 0, 24)
title.BackgroundTransparency = 1
title.Text = "Monster Gyroscope · Cl(15,0,0)"
title.TextColor3 = Color3.fromRGB(136, 170, 255)
title.TextScaled = true
title.Font = Enum.Font.RobotoMono
title.Parent = frame

local info = Instance.new("TextLabel")
info.Size = UDim2.new(1, -10, 1, -30)
info.Position = UDim2.new(0, 5, 0, 28)
info.BackgroundTransparency = 1
info.TextColor3 = Color3.fromRGB(180, 180, 200)
info.TextXAlignment = Enum.TextXAlignment.Left
info.TextYAlignment = Enum.TextYAlignment.Top
info.TextScaled = true
info.Font = Enum.Font.RobotoMono
info.Text = string.format([[Conserved:
  Hub proj = 1.0000
  |c₁|² = %s
  Hub sum = %d

Eigenspaces:
  Earth (λ=-1, 7D)
  Spoke (λ=-1, 5D)
  Hub   (λ=+1, 1D)
  Clock (λ=ω,  2D)

Trivector: 47×59×71=%d
Skeleton: {%d, %d}]],
	"19,737,810", Config.HUB_SUM,
	Config.TRIVECTOR_PRODUCT,
	Config.SKELETON[1], Config.SKELETON[2])
info.Parent = frame
