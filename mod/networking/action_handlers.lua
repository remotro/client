Client = {}

function Client.connect()
	love.thread.getChannel("uiToNetwork"):push("connect")
end

function Client.send(msg)
	if msg ~= "action:keepAliveAck" then
		sendInfoMessage(string.format("Client sent message: %s", msg), "REMOTRO")
	end
	love.thread.getChannel("uiToNetwork"):push(msg)
end

local function action_keep_alive()
	Client.send("action:keepAliveAck")
end

local function string_to_table(str)
	local tbl = {}
	for part in string.gmatch(str, "([^,]+)") do
		local key, value = string.match(part, "([^:]+):(.+)")
		if key and value then
			tbl[key] = value
		end
	end
	return tbl
end

local super_game_update = Game.update

function Game:update(dt)
	super_game_update(self, dt)

	repeat
		local msg = love.thread.getChannel("networkToUi"):pop()
		if msg then
			local parsedAction = string_to_table(msg)

			if not ((parsedAction.action == "action:keepAlive") or (parsedAction.action == "action:keepAliveAck")) then
				local log = string.format("Client got %s message: ", parsedAction.action)
				for k, v in pairs(parsedAction) do
					log = log .. string.format(" (%s: %s) ", k, v)
				end
				sendInfoMessage(log, "REMOTRO")
			end

			if parsedAction.action == "keepAlive" then
				action_keep_alive()
			end
		end
	until not msg
end
