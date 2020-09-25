-- wireshark protocol analyzer for lessen protocol.

--プロトコル定義
lessen = Proto("lessen", "Lessen Protocol")

-- プロトコルフィールド定義
lessen_command_id_one = ProtoField.new("Lessen Protocol Command 1", "lessen.command_id_one", ftypes.STRING)
lessen_command_id_two = ProtoField.new("Lessen Protocol Command 2", "lessen.command_id_two", ftypes.STRING)
lessen_data = ProtoField.new("Lessen Protocol Data", "lessen.data", ftypes.STRING)

-- プロトコルフィールド定義
lessen.fields = {lessen_command_id_one, lessen_command_id_two, lessen_data}

-- Dissector定義
function lessen.dissector(buffer, pinfo, tree)
    -- 表示名
    pinfo.cols.protocol:set("LESSEN")

    -- データ長を計算
    local buflen = buffer:reported_length_remaining()
    local datalen = buflen - 2

    -- バッファ長がヘッダー長に満たない場合はスルー
    if buflen < 2 then
        return
    end

    -- サブツリーとして追加
    local subtree = tree:add(lessen, buffer())

    -- サブツリーにlessen_command_id_oneを追加
    subtree:add(lessen_command_id_one, buffer(0,1))
    -- サブツリーにlessen_command_id_twoを追加
    subtree:add(lessen_command_id_two, buffer(1,1))

    -- サブツリーにlessen_dataを追加
    if datalen > 0 then
        subtree:add(lessen_data, buffer(2,datalen))
    end
end

-- Lessen Protocolをポート番号と対応させる
tcp_table = DissectorTable.get("tcp.port")
tcp_table:add(37564, lessen)
