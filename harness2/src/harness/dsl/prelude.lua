function M(mnemonic)
    return __task_context:new_message(mnemonic)
end

function A(mnemonic)
    return __task_context:new_action(mnemonic)
end

function S(mnemonic)
    return __task_context:new_state(mnemonic)
end

function P(mnemonic, entry_state)
    return __task_context:new_process(mnemonic, entry_state)
end

function E(source, target, trigger, action)
    return __task_context:new_edge(source, target, trigger, action)
end

function executable(flag)
    return __task_context:executable(flag)
end

setmetatable(_G, {
    __index = function (t, k)
        return __task_context[k]
    end
})
