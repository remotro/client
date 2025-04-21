RE.InHooks = {}

function RE.IN_HOOKS.setup_run(bundle)
    G.EVENT_MANAGER.push(Event{})
    G.FUNCS.setup_run(bundle["back"], bundle["stake"], bundle["seed"])
end