Client = {}

function Client.connect()
	love.thread.getChannel("uiToNetwork"):push("connect?")
end

function Client.send(msg)
	if msg ~= "pong!" then
		sendInfoMessage(string.format("Client sent message: %s", msg), "REMOTRO")
	end
	love.thread.getChannel("uiToNetwork"):push(msg)
end

local super_game_update = Game.update

function Game:update(dt)
	super_game_update(self, dt)

	repeat
		local msg = love.thread.getChannel("networkToUi"):pop()
		if msg then
			local exclamationIndex = string.find(msg, "!")
			local action = msg
			local body = {}
			
			if exclamationIndex then
				actionString = string.sub(msg, 1, exclamationIndex)
				body = RE.JSON.decode(string.sub(msg, exclamationIndex + 1))
			end

			if not ((action.action == "ping!") or (action.action == "pong!")) then
				local log = string.format("Client got %s message: ", parsedAction.action)
				for k, v in pairs(parsedAction) do
					log = log .. string.format(" (%s: %s) ", k, v)
				end
				sendInfoMessage(log, "REMOTRO")
			end

			if action == "ping" then
				Client.send("pong!")
			elseif action == "setup_run" then
				RE.IN_HOOKS.setup_run(body)
			end
		end
	until not msg
end
