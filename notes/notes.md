STUN

Client 1 sends req to server to get their ipv4 addr
Client 2 send req to server to get their ipv4 addr

RUBY SERVER
Have database of peers, unique 'name' columns with passwords
User can check to see if a name is taken when registering from cli. If not, create one with password.

User table will have a column for last_known_ip, or something like that. This will be updated in stun server style when a command from the client side rust binary is run. (like `sign-in` or something.)

USERS
username: string
password: string ( devise )
email: string ( use devise#{confirmable, recoverable} for forgotten passwords )
last_known_ip: string
public: bool (make everybody public at first for simplicity. )

You can search for a user to connect to from rust side, and if they exist, server will respond with their IP:PORT combination. possibility of adding other columns in user field that allow them to be 'discoverable' or not.

APPLICATION FLOW

(Client 1)
$ register "the-game" -p "mypassword"
-> ERROR(s): Name is already taken, password must contain uppercase letter and number
$ register "the-game21" -p "Mypassword2"
-> SUCCESS! 🍰 You have been registered. You can try to connect with other users by entering `$ connect user_name`
$ connect idontexist
-> ERROR(s): 'idontexist' does not exist!
$ connect my_ip_is_old #the peer port may have been closed by router
-> Connecting...
(some Rust error here related to that IP:PORT being out of date and closed)

## Here, client 1 still has program running and his last_known_ip on the RUBY server (which is updated with every request) is accurate

(Client 2)
$ register "jesse" -p "coolpass"
-> SUCCESS! 🍰 You have been registered. You can try to connect with other users by entering `$ connect user_name`
$ connect the-game21

# (server side logic here) SELECT last_known_ip FROM USERS where user = the-game21

# this IP:PORT combo is send back to Client 2 with successful status code, showing user found

-> Connecting...

(Client 1)
"jesse" would like to chat!
Accept> [y/n]
$ y
-> Connected with jesse. Say something!

(Client 2)
the-game21 has connected. Say something!
$ yo

(Client 1)
jesse: yo
$

Rust code must be implemented to be able to read lines from STDIN in a non-blocking fashion.

I believe a thread must be used to send data to peer, as well as listen to data from peer for concurrency purposes.
