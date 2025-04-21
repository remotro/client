
local super_game_update = Game.update

function Game:update(dt)
	super_game_update(self, dt)

	repeat
		local request = RE.Client.request()
		if request then
            sendDebugMessage("Recieved " .. request.kind)
            if request.kind == "setup_run" then
				RE.InHooks.setup_run(request.body, RE.Client.respond)
            end
		end
	until not request
end
