Client = {}

function Client.send(msg)
	if msg ~= "action:keepAliveAck" then
		sendTraceMessage(string.format("client sent message: %s", msg), "REMOTRO")
	end
	love.thread.getChannel("uiToNetwork"):push(msg)
end

local function action_keep_alive()
	Client.send("action:keepAliveAck")
end

local super_game_update = Game:update

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
				sendTraceMessage(log, "REMOTRO")
			end

			if parsedAction.action == "keepAlive" then
				action_keep_alive()
			end
		end
	until not msg
end
