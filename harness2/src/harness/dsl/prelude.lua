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

function add_abstract_model(name, model)
    __abstract_models[name] = model
end

function set_concretization(concretization)
    __concretization = concretization
end

function add_mapping(name, source, target, mapping)
    __mappings[name] = { source, target, mapping }
end

function add_query(query)
    table.insert(__queries, query)
end

setmetatable(_G, {
    __index = function (t, k)
        return __task_model[k]
    end
})
