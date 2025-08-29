local fs = require("@lune/fs")
local serde = require("@lune/serde")
local process = require("@lune/process")
local roblox = require("@lune/roblox")

local inputPath = process.args[1]
assert(fs.isFile(inputPath), "path is not a file")

local pathParts = inputPath:split(".")
local extension = pathParts[#pathParts]
local isPlace = extension == "rbxl" or extension == "rbxlx"
local isModel = extension == "rbxm" or extension == "rbxmx"
assert(isPlace or isModel, "file is not a place or model")

local outputPath = process.args[2]
assert(outputPath and string.sub(outputPath, -4) == "json", "output path must end with .json")

local output = {
	1,
	0,
	{
		progname = "Lune",
		progver = "1.0",
		timestamp = os.time(),
	},
}

function indexInstance(instance: roblox.Instance, forceNoClassNameSuffix: boolean?)
	local name = instance.Name
	local className = instance.ClassName
	local byteSize = roblox.getByteSize(instance)

	local infoBlock = {
		name = if name == className or forceNoClassNameSuffix then name else `{name} ({className})`,
		dsize = byteSize,
		asize = byteSize,
	}

	local children = instance:GetChildren()
	if #children > 0 then
		local info = { infoBlock }

		for _, child in children do
			table.insert(info, indexInstance(child))
		end

		return info
	else
		return infoBlock
	end
end

local fileContents = fs.readFile(inputPath)

if isPlace then
	local game = roblox.deserializePlace(fileContents)
	local targetServices = {
		"Workspace",
		"Lighting",
		"MaterialService",
		"ReplicatedFirst",
		"ReplicatedStorage",
		"ServerScriptService",
		"ServerStorage",
		"StarterGui",
		"StarterPlayer",
		"Teams",
		"SoundService",
		"TextChatService",
	}

	local model = roblox.Instance.new("Model")
	model.Name = "Game"

	output[4] = { indexInstance(model, true) }

	for _, serviceName in targetServices do
		local service = game:GetService(serviceName)
		table.insert(output[4], indexInstance(service))
	end
elseif isModel then
	local model = roblox.Instance.new("Model")
	model.Name = "Model"

	for _, child in roblox.deserializeModel(fileContents) do
		child.Parent = model
	end

	output[4] = indexInstance(model, true)
end

fs.writeFile(outputPath, serde.encode("json", output, false))
