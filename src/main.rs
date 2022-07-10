fn main() {
    println!("Hello, world!");
}

/*
@startuml
autonumber "<b>[0]"
skinparam responseMessageBelowArrow true
title Example Sequence Diagram
LocalRequestReceiver -> LocalRadioAgent: Blocking Request
LocalRequestReceiver <-- LocalRadioAgent: Response
LocalRequestReceiver ->> LocalRadioAgent: Non-Blocking Message
LocalRequestReceiver -> Thread1: Sometimes I talk to myself



loop select from crossbeam channels (rx_local_req, rx_)

note right
Each request we receive has a TARGET, req and ORIGIN.
The Target is the name of the radio, nethost, etc.
The req is an enum (e.g. RadioRequest) which will determine
what type of object this is addressed to.

We keep a hashmap of local_radios and remote_radios and
one for each other object type.

Assuming that the request is for a Radio:
we check to see if the target is in local_radios first.

local_radios hash has key, radio name, and value
tuple (crossbeam:tx_radio_req_ch, crossbeam:tx_request_reply_ch)

tx_radio_req_ch is crossbeam <(RadioRequest, ORIGIN, Option<tx_radio_reply_ch>)>
tx_radio_reply_ch is crossbeam <(RadioReply, ORIGIN)>

ORIGIN is a tuple (ObjectType,NAME,req_id)
where req_id is an incrementing sequence number for each
request.

NOTE: REST requests currently go direct to the RadioAgent and
create their own oneshot tx_radio_reply_ch channel to receive
the reply.
Instead, they could create the same oneshot channel and wrap
the RadioRequest into a LocalRequest and send the request
to the RequestRouter passing the optional tx_radio_reply_ch
so that the radio would reply directly to that channel.
A reply channel must be created for each REST request
since receiver(consumer) channels have one receiver
(multiple procucer single consumer).

Objects can make requests to other objects and then receive
replies to these requests. These are generic requests
and not specific RadioRequests. Thus, each object can make
a generic request and receive generic replies. Each
object is equiped with an outbound request handler.
The outbound request handler has send, recv, and
send_recv flavor methods. 
send(ObjectType, NAME, Request) is routed to the RequestRouter
and returns the integer req_id.

The outbound request handler allows multiple send()
requests to be dispatched before a recv().
The replies can come in any order. 

The outbound request handler has a BTreeMap
for replies with key the . If a recv_req_iq(req_id) is
called then we look in the BTreeMap for that req_id and
if is found we remove and return the reply.
If it is not found then we keep receiving on the
rx_request_reply_ch and returning it we get a match
of we place the reply into the BTreeMap.

Upon new creation of the Registry creating the local
Nethost, the RouterRouter for the local nethost is created.
The RequestRouter creates tx_radios_outbound_req_reply_ch(7)/
rx_radios_outbound_req_reply_ch(8) for outbound requests from
radios.


Upon new creation of a ObjectAgent (e.g. RadioAgent) we
are provided the tx_router_req_ch(1).
We create the tx_radio_req_ch(3)/rx_radio_req_ch(4) and
the tx_radio_reply_ch(5)/rx_radio_reply_ch(6).

1) tx_router_req_ch  (Requests sent to RequestRouter by OutboundRequestHandler/REST/Remoc)
crossbeam <(LocalRequest, ORIGIN, Option<tx_reply_ch>)>

RadioAgent constructor will send 
    NewLocalRadio(NAME,
                  tx_radio_req_ch(3),     // inbound requests
                  tx_radio_reply_ch(5))   // outbound request replies
via tx_router_req_ch(1).
RadioAgent expects tx_router_req_ch(1) as a reply to its
NewLocalRadio request to the
# RadioAgent expects a reply to its NewLocalRadio request to the
# RequestRouter. 
# RequestRouter create tx_radios_outbound_req_reply_ch(7)/
# rx_radios_outbound_req_reply_ch(8) give provide us a copy of
# tx_radios_outbound_req_reply_ch(7) as the default channel for sending
# our replies back to the RequestRouter.
RadioAgent sends outbound requests via tx_router_req_ch(1).

2) rx_router_req_ch  (Requests received by RequestRouter)
crossbeam <(LocalRequest, ORIGIN, Option<tx_reply_ch>)>
RequestRouter constructor will create the
tx_router_req_ch(1)/rx_router_req_ch(2).
The main loop of the RequestRouter waits recv() on this
channel for requests.

3) tx_radio_req_ch  (Requests sent to RadioAgent)
crossbeam <(RadioRequest, ORIGIN, tx_radio_reply_ch)>
During construction, RadioAgent will give a copy
of this to the RequestRouter as part of
NewLocalRadio(NAME, tx_radio_req_ch(4)) via
tx_router_req_ch(1). This will then be used by RequestRouter
to send in requests to this radio.
RadioAgent also returns this channel from the constructor so
that the registry has a copy which is given to the
EventRouter to pass messages to the radio.

4) rx_radio_req_ch  (Requests received by RadioAgent)
crossbeam <(RadioRequest, ORIGIN, Option<tx_radio_reply_ch>)>
The main loop of the RadioAgent receives on the rx_radio_req_ch(4).
The rx_radio_req_ch
The result of each request does out on given tx_radio_reply_ch(5)
or if None provided then replies should go to the RequestRouter,
via tx_radios_outbound_req_reply_ch(7).

5) tx_radio_reply_ch
crossbeam <(LocalRequest, ORIGIN)>
ORIGIN is a tuple (ObjectType,NAME,req_id)

We create this during the RadioAgent constructor and give it to
the RequestRouter as part of the NewLocalRadio message.
The RequestRouter uses this to forward and replies it
receives for this ORIGIN. rx_radio_reply_ch(6) is
used by RadioAgent outbound request handler to receive
these reply messages send on this channel from the
RequestRouter.


6) rx_radio_reply_ch
crossbeam <(LocalRequest, ORIGIN)>
RadioAgent creates this during the constructor and give
the RequestRouter tx_radio_reply_ch(5) as part of 
the NewLocalRadio message.
The RequestRouter forwards replies it
receives for this ORIGIN.
RadioAgent uses the rx_radio_reply_ch(6) in the
outbound request handler to listen for replies 
from the RequestRouter to any requests we make.
We send any requests we make via tx_router_req_ch(1).

7) tx_radios_outbound_req_reply_ch 
crossbeam <(LocalReply, ORIGIN)>
the default channel for sending

8) rx_radios_outbound_req_reply_ch
crossbeam <(LocalReply, ORIGIN)>
the default channel for sending


We tx_radio_req_ch.send(req, ORIGIN) and do not wait 
for the reply. 

When the radio replies at the end of processing the request:
1) REST ORIGIN:
    a oneshot tx_radio_reply_ch created by the REST route
    handler
2) Local Object Request
    an unbounded tx_radio_reply_ch created by the REST route
    handler

Then when we receive a request matching a RadioRequest
we 

end note

else rx_local_req(TARGET, req, ORIGIN)

LocalRequestReceiver -[#green]>> LocalRadioAgent: green arrow
else it rained
loop 1000 times
LocalRequestReceiver -[#blue]>> LocalRadioAgent: blue arrow
end
else it snowed
LocalRequestReceiver ->> LocalRadioAgent: //snowing italics//
end


@enduml
*/