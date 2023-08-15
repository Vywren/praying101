#hope this works
#$ rustc client.rs; rustc server.rs

# Terminal 1
#$ ./server
#Server started, waiting for clients

# Terminal 2
#$ ./client hello

# Terminal 1
#Server started, waiting for clients
#Client said: hello