RE.InHooks = {}

function RE.InHooks.setup_run(bundle)
    -- Balatro assumes that run start will occur in run setup,
    -- which will populate the viewed deck (back). We must "pretend"
    -- this is the case as well. 
    G.GAME.viewed_back = G.P_CENTERS[bundle["back"]]
    G.FUNCS.start_run(e, {stake = bundle["stake"], seed = nil, challenge = nil});
end