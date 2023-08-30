# Cryptocurrency
Basic implementation of cryptocurrency and blockchain.

## Usage

Algorithm:

1. start a node
2. connect to other nodes
3. synchroize chain of the local node with the network
4. add transaction
5. mine block
6. synchroize all nodes with the network


To start a node:
```
cargo run 127.0.0.1 5000 "User Name"
```

### Http methods
Available Http methods are described in `src.main.rs`.


#### Get current state of chain of the node
```
GET http://127.0.0.1:5000/chain
```


#### Connect current node to nodes
```
POST http://127.0.0.1:5000/connect_node
```
payload:
```
{"addresses": ["http://127.0.0.1:5001", "http://127.0.0.1:5002"]}
```


#### Mine block

```
GET 127.0.0.1:5000/mine
```


#### Replace chain (consensus)
Synchronize connected nodes

```
GET 127.0.0.1:5000/replace_chain
```


#### Add transaction

```
POST 127.0.0.1:5000/add_transaction
```
payload:
```
{ "sender": "http://127.0.0.1:5000", "receiver": "http://127.0.0.1:5002", "amount": 10.0 }
```