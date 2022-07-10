
ORIGIN is a tuple (ObjectType,NAME,req_id)

RadioAgent
    tx_router_req_ch
    rx_radio_req_ch
    req_id
    tx_router_req_ch // clone sent to RequestRouter with each request
    rx_router_reply_ch

init(tx_router_req_ch) {
    //   cloned to RequestRouter/recv LocalRequest from RequestRouter
    create tx_radio_req_ch/rx_radio_req_ch

    //   cloned to RequestRouter/recv LocalReply from RequestRouter
    create tx_router_reply_ch/rx_router_reply_ch

    tx_router_req_ch.send(
        NEW_LOCAL_RADIO{
            NAME
            tx_radio_req_ch,
        }
    )
}
loop() {
    rx_radio_req_ch(req, tx_radio_reply_ch, ORIGIN) // from: tx_radio_req_ch
        query(Nethost, "uptime")
        tx_radio_reply_ch(RadioReply, ORIGIN)
}

next_req_id() -> int {
    self.req_id++
    return self.req_id
}

query() -> LocalReply {
    send_req_id=self.next_req_id()
    origin = ORIGIN{self.name, Radio, send.req_id}
    // send to rx_router_req_ch
    tx_router_req_ch(LocalRequest(Nethost), tx_router_reply_ch.clone(), origin)
    (LocalReply, recv_req_id) = rx_router_reply_ch.recv()
    if recv_req_id + 1 != send_req_id {
        Error (got reply for recv_req_id, expected reply for send_req_id)
    }
}
////////////////////////////////////////////////////////////////////////
RequestRouter
    tx_radio_reply_ch<RadioReply, ORIGIN>
    req_id
    active_reqs: BTreeMap<ReqId, tx_router_reply_ch)
    local_radios: HashMap<(NAME, tx_radio_rec_ch)), {tx_radio_req_ch, in_req_id}
    local_reply_ch: HashMap<(ObjectType, NAME), {tx_radio_req_ch, in_req_id}
    remote_nethosts: HashMap<NAME, {req_ch: tx_req_remote_ch,
                                    reply_ch: REMOC::tx_reply_remote_ch}>
    

init() {

}
loop() {
    select
    _____________________________________
    // recv from tx_router_req_ch
    rx_router_req_ch(target: TARGET, req: LocalRequest, origin: ORIGIN)

    match req
        self.req_id ++
        RadioReq(req):
            if target is local:
                tx_radio_req_ch = local_radios(target)
                // send to rx_radio_req_ch
                tx_radio_req_ch.send(req, tx_radio_reply_ch.clone(), origin)
            else:
                target_nethost = find_nethost(target)
                tx_remoc_req_ch = remote_nethosts(origin_nethost).req_ch
        NewLocalRadio(NAME, tx_radio_req_ch)
            self.local_radios[NAME] = tx_radio_req_ch
        NewRemoteNethost(NAME, tx_req_remote_ch)
            self.remote_nethosts[NAME] = tx_req_remote_ch
    _____________________________________
    tx_radio_reply_ch(reply: RadioReply, origin: ORIGIN)
        if origin is local:
            tx_router_reply_ch = local_reply_ch(origin)
            // send to rx_router_reply_ch
            tx_router_reply_ch.send(LocalReply::RadioReply(reply), origin.req_id)
        else:
            origin_nethost = find_nethost(origin)
            tx_remoc_reply_ch = remote_nethosts(origin_nethost).reply_ch
            tx_remoc_reply_ch.send(LocalReply::RadioReply(reply), origin)

}

////////////////////////////////////////////////////////////////////////
///
REMOC Server
init() {
    // called upon new nethost remoc client connection

}
loop(tx_router_req_ch) {
    new nethost remoc client
    create tx_remoc_req_ch/rx_remoc_req_ch REMOC<(TARGET, LocalRequest, ORIGIN)
    create tx_remoc_reply_ch/rx_remoc_reply_ch REMOC<(TARGET, LocalReply, ORIGIN)
    spawn RemocReqServer(tx_router_req_ch)
}

////////////////////////////////////////////////////////////////////////
///
RequestRouter REMOC Req Server thread
init() {
    // called upon new nethost remoc client connection

}
loop() {
    // recv from tx_remoc_req_ch
    // handle outbound requests
    rx_remoc_req_ch(target: TARGET, req: LocalRequest, origin: ORIGIN)
        tx_router_req_ch.send(target, req, origin)
}

////////////////////////////////////////////////////////////////////////
RequestRouter REMOC Reply Server thread
init() {
    // called upon new nethost remoc client connection

}
loop() {
    // recv from tx_remoc_reply_ch
    // handle outbound replies
    rx_remoc_reply_ch(target: TARGET, req: LocalRequest, origin: ORIGIN)
        tx_router_reply_ch.send(target, req, origin)
}

////////////////////////////////////////////////////////////////////////
///
RequestRouter REMOC Req Server thread
init() {
    // called upon new nethost remoc client connection

}
loop() {
    // recv from tx_remoc_req_ch
    // handle outbound requests
    rx_remoc_req_ch(target: TARGET, req: LocalRequest, origin: ORIGIN) tx_router_req_ch.send(target, req, origin)
}

////////////////////////////////////////////////////////////////////////
RequestRouter REMOC Reply Server thread
init() {
    // called upon new nethost remoc client connection

}
loop() {
    // recv from tx_remoc_reply_ch
    // handle outbound replies
    rx_remoc_reply_ch(target: TARGET, req: LocalRequest, origin: ORIGIN)
        tx_router_reply_ch.send(target, req, origin)
}