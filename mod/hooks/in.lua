RE.InHooks = {}

function RE.InHooks.start_run(bundle, cb)
    if G.SCREEN ~= G.SCREENS.MAIN_MENU then
        cb({
            type = "err",
            error = "cannot do this action, must be in main_menu but in " .. G.SCREEN
        })
    end

    back_obj = G.P_CENTERS[bundle["back"]]
    if not back_obj then
        cb({
            type = "err",
            error = "could not find back " .. bundle["back"]
        })
    end

    if not back_obj.unlocked then
        cb({
            type = "err",
            error = "back " .. bundle["back"] .. " is not unlocked"
        })
    end

    stake = G.P_STAKES

    -- Balatro assumes that run start will occur in run setup,
    -- which will populate the viewed deck (back). We must "pretend"
    -- this is the case as well. 
    G.GAME.viewed_back = back_obj
    G.FUNCS.start_run(e, {stake = bundle["stake"], seed = bundle["seed"], challenge = nil});

    cb({
        type = "ok",
        value = {}
    })
end