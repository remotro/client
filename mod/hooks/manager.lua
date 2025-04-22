local super_game_update = Game.update

function responder(kind)
	return function (body)
		RE.Client.respond({
			kind = kind,
			body = body
		})
	end
end

function Game:update(dt)
	super_game_update(self, dt)

	repeat
		local request = RE.Client.request()
		if request then
            sendDebugMessage("Recieved " .. request.kind)
            if request.kind == "main_menu/start_run" then
				RE.InHooks.start_run(request.body, responder("main_menu/start_run/result"))
            end
		end
	until not request
end
