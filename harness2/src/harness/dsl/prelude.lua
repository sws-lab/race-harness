function M(mnemonic)
    return __task_model:new_message(mnemonic)
end

function A(mnemonic)
    return __task_model:new_action(mnemonic)
end

function S(mnemonic)
    return __task_model:new_state(mnemonic)
end

function P(mnemonic, entry_state)
    return __task_model:new_process(mnemonic, entry_state)
end

function E(source, target, trigger, action)
    return __task_model:new_edge(source, target, trigger, action)
end

function executable(flag)
    return __task_model:executable(flag)
end

function swap_task_model(ctx)
    local old_ctx = __task_model
    __task_model = ctx
    return old_ctx
end

setmetatable(_G, {
    __index = function (t, k)
        return __task_model[k]
    end
})
