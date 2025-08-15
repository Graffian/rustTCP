client sends messages to server , server already
stores all the clients in a hashmap along w their
client id and writers in a mutex guard , now when
 a client sends messages to the server , the server's 
role is to broadcast the message , how does it do that?

it loops through all client ids and writer func to do
.write_all in each writer func to display the messages i.e
send it back to the client , this part is cool but we have
use mpsc too to make it scalable

mpsc is a channel soemthing like a loophole or a channel
connecting 2 places , msgs coming in from one end using
tx , one rx that is just reading those messages in the 
channel

so what we do is in the mpsc also we send using tx.send 
the msg and the client_id and we recieve it on the other
part w rx.recv() then we acquire the lock loop over it and
write in all the clients ik this is the workflow i
understand it but ii dont get what to run as a task separately
can u help me in thinking that
