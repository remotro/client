RE.Client = {}

local function send(msg)
    love.thread.getChannel("uiToNetwork"):push(msg)
end

local function recv(msg)
    return love.thread.getChannel("networkToUi"):pop()
end

function RE.Client.connect()
    send("connect!")
end

function RE.Client.request()
    request = nil
    repeat
        local msg = recv()
        if msg then
            local exclamationIndex = string.find(msg, "!")
            local kind = nil
            local body = nil
            
            
            if exclamationIndex then
                kind = string.sub(msg, 1, exclamationIndex - 1)
                body = string.sub(msg, exclamationIndex + 1)
                if kind ~= "ping" then
                    sendDebugMessage("Received " .. kind .. " " .. body)

                    if body ~= "" then
                        body = RE.JSON.decode(body)
                    else
                        body = {}
                    end

                    request = {
                        kind = kind,
                        body = body
                    }
                else
                    send("pong!")
                end    
            else
                sendWarnMessage("Malformed message packet")
            end
        end
    until request or not msg
    return request
end

function RE.Client.respond(response)
    local msg = response.kind
    if response.body ~= "" then
        msg = msg .. "!" .. RE.JSON.encode(response.body)
    else
        msg = msg .. "!"
    end
    
    sendDebugMessage("Sending " .. msg)
    send(msg)
end