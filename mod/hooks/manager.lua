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
				RE.InHooks.start_run(request.body, responder("placeholder/result"))
			elseif request.kind == "blind_select/select" then
				RE.InHooks.select_blind(request.body, responder("placeholder/result"))
			elseif request.kind == "blind_select/skip" then
				RE.InHooks.skip_blind(request.body, responder("placeholder/result"))
            end
		end
	until not request
end
