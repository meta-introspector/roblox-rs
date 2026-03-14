-- Monster Gyroscope Configuration · Cl(15,0,0)
local Config = {}

Config.SSP = {2,3,5,7,11,13,17,19,23,29,31,41,47,59,71}
Config.C1 = {4371,782,133,50,16,11,6,5,3,2,2,1,0,0,0}
Config.GENUS = {0,0,0,0,1,0,1,1,2,2,2,3,4,5,6}
Config.ROLES = {"Earth","Earth","Earth","Earth","Earth","Earth",
    "Spoke","Hub","Hub","Heaven","Heaven","Heaven",
    "Invisible","Invisible","Invisible"}

Config.CLOCK_RADIUS = 20
Config.ROTOR_RADIUS = 14
Config.FRAME_RADIUS = 50
Config.CENTER = Vector3.new(0, 20, 0)
Config.STEP_TIME = 0.8

Config.COLORS = {
    Hub = Color3.fromRGB(255, 170, 0),
    Clock = Color3.fromRGB(68, 136, 255),
    Rotor = Color3.fromRGB(255, 85, 85),
    FramePos = Color3.fromRGB(68, 204, 68),
    FrameNeg = Color3.fromRGB(204, 68, 68),
    Invisible = Color3.fromRGB(255, 85, 85),
    Tick = Color3.fromRGB(60, 60, 100),
}

-- Math
Config.TRIVECTOR_PRODUCT = 47 * 59 * 71 -- 196883
Config.O_RATIO = -0.5
Config.HUB_SUM = 8
Config.C1_NORM_SQ = 19737810
Config.PERIOD = 6
Config.SKELETON = {3, 19}

function Config.clock(n) return math.cos(n * math.pi / 3) end
function Config.rotor(n) return -math.cos(n * math.pi / 3) end
function Config.frame(n) return (n % 2 == 0) and 1 or -1 end

return Config
