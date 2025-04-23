RE = SMODS.current_mod

function RE.load_re_file(file)
	local chunk, err = SMODS.load_file(file, "Remotro")
	if chunk then
		local ok, func = pcall(chunk)
		if ok then
			return func
		else
			error("Failed to process file: " .. func)
		end
	else
		error("Failed to find or compile file: " .. tostring(err))
	end
	return nil
end

local SOCKET = RE.load_re_file("vendor/socket/socket.lua")
RE.NETWORKING_THREAD = love.thread.newThread(SOCKET)
RE.NETWORKING_THREAD:start(SMODS.Mods["Remotro"].config.server_url, SMODS.Mods["Remotro"].config.server_port)

RE.JSON = RE.load_re_file("vendor/json/json.lua")
RE.load_re_file("net/client.lua")

RE.load_re_file("hooks/in.lua")
RE.load_re_file("hooks/manager.lua")

RE.Client.connect()