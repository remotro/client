RE.InHooks = {}

function RE.InHooks.start_run(bundle, cb)
    if G.STATE ~= G.STATES.MAIN_MENU and G.STATE ~= G.STATES.SPLASH then
        cb({
            Err = "cannot do this action, must be in main_menu but in " .. G.STATE
        })
        return
    end

    back_obj = G.P_CENTERS[bundle["back"]]
    if not back_obj then
        cb({
            Err = "could not find back " .. bundle["back"]
        })
        return
    end

    if not back_obj.unlocked then
        cb({
            Err = "back " .. bundle["back"] .. " is not unlocked"
        })
        return
    end

    -- Balatro assumes that run start will occur in run setup,
    -- which will populate the viewed deck (back). We must "pretend"
    -- this is the case as well. 
    G.GAME.viewed_back = back_obj
    G.FUNCS.start_run(e, {stake = bundle["stake"], seed = bundle["seed"], challenge = nil});

    cb({
        Ok = {}
    })
end

function get_e(id)
    return G.blind_select.UIRoot.children[1].children[1].config.object:get_UIE_by_ID(id)
end

function RE.InHooks.select_blind(bundle, cb)
    if G.STATE ~= G.STATES.BLIND_SELECT then
        cb({
            Err = "cannot do this action, must be in blind_select but in " .. G.STATE
        })
        return
    end

    G.FUNCS.select_blind(get_e('select_blind_button'))
    cb({
        Ok = {}
    })
end

function RE.InHooks.skip_blind(bundle, cb)
    if G.STATE ~= G.STATES.BLIND_SELECT then
        cb({
            Err = "cannot do this action, must be in blind_select but in " .. G.STATE
        })
        return
    end

    G.FUNCS.skip_blind(get_e('skip_blind_button'))
    cb({
        Ok = {}
    })
end