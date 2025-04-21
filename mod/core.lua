RE = SMODS.current_mod

function RE.load_rm_file(file)
	local chunk, err = SMODS.load_file(file, "Remotro")
	if chunk then
		local ok, func = pcall(chunk)
		if ok then
			return func
		else
			sendWarnMessage("Failed to process file: " .. func, "REMOTRO")
		end
	else
		sendWarnMessage("Failed to find or compile file: " .. tostring(err), "REMOTRO")
	end
	return nil
end

RE.load_rm_file("hooks/in.lua")
RE.JSON = RE.load_rm_file("vendor/json/json.lua")
RE.load_rm_file("vendor/networking/action_handlers.lua")
local SOCKET = RE.load_rm_file("vendor/networking/socket.lua")
RE.NETWORKING_THREAD = love.thread.newThread(SOCKET)
RE.NETWORKING_THREAD:start(SMODS.Mods["Remotro"].config.server_url, SMODS.Mods["Remotro"].config.server_port)

Client.connect()