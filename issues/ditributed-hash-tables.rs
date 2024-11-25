// Subtopic: Applying Rust in Distributed Hash Tables (DHTs) for Effective Peer-to-Peer Networking in Horizontal Scaling
// Use Case: An online gaming company uses Rust-based Distributed Hash Tables to effectively manage its peer-to-peer network, leading to an increase in the game's performance and ability to accommodate more players simultaneously. Furthermore, with the use of Rust, the system achieves superior reliability, concurrent execution, and safety in managing network nodes, enhancing the overall experience for players and contributing to the horizontal scaling of the system.

// promtp: I am part of a team developing an online multiplayer game in Rust. We use peer to peer networking and we want to start using distributed hash tables to manage the network in order to increase the horizontal scaling of the system. Can you help me do this?
// eval
// The prompt relates to a topic that is really complex, to add a Distributed Hash Table in order to improve the performance of an already working P2P network. Although the prompt is not ambiguous, it can be hard for the LLM to figure out what the user needs for the code to be executed properly.

// https://www.geeksforgeeks.org/distributed-hash-tables-with-kademlia/

// model 1
// * Instruction Following
// -- There is room for improvement, but in overall the instruction following has minor errors as it provides a good skeleton to work from but lacks in technical depth by omitting key components of DHT like routing tables and lookup mechanisms.

// *Accuracy:
// -- The model response is almost perfect on the expected accuracy, as it takes the basic structure if a P2P node and adds what a DHT node needs to have.
// -- The documentation provided is good as it correctly introduces the Kademlia algorithm. 
// -- It fails in providing technical depth in resolving the DHT algorithm, it is missing implementation of routing tables, lookup mechanisms, bucket management and node selection strategies. 
// -- It clearly shows Kademlia's advantages and disadvantages. 
// -- The code is free from syntax errors, but it doesn´t compile due to a lifetime error.

// * Efficiency:
// -- Although in general the code is correct, it must be more complex than the one provided. Key features of the Kademlia algorithm are missing, like routing tables and the lookup mechanisms. 

// * Up-to-date
// -- Both Rust crates recommended are too old, the `kad` crate was last updated 3 years ago and `kademlia-dht` was last updated 5 years ago. It's best to recommend `libp2p` which implements a Kademlia DHT that is used in P2P networks like IPFS.

// * Executable code
// -- The code is not immediately excecutable because of lifetime errors, this can be frustrating for users with less experience.
// -- It is expected that the model would give an answer similar to the one given, the user has to understand that the real value of the answer comes from having a skeleton of the code to then extend with the actual requirements. The given code has compilation errors, but it is not a big deal as the important part resides on how the DHT works.


// model 2

// * Instruction Following
// -- The code given must be suitable to be used as a skeleton to be combined with a functional P2P Node implementation, and the given node is unsuitable for this task. It gives an incomplete and centralized structure with the name of DHT which contradicts with what DHT actually means.

// *Accuracy:
// -- The model response fails to deliver proper documentation of what DHT is and its implications, and it is also not clear how to take this DHT Node implementation and apply it to an already functioning P2P Node. The documentation around the code is poor as it jumps way too early into code developement before explaining what even Kademlia is.  
// -- The `Node` implementation is not at all accurate, a lot of functions are missing, like `get` or `put`.  
// -- The `tokio` crate is imported but it is only used to create the example network, not to help create the DHT Node, this adds confusion and inefficency.
// -- The `distance`  function has nothing to do with Kademlia's XOR metric, it is a way too simplistic implementation.

// * Efficiency:
// -- It's not an efficient code, even when what is needed is just the skeleton. It presents a structure called `DHT` which is intended to use as a centralized structure, this goes against everything that DHT actually looks for, this is creating a decentralized way of storing key-value pairs. 
// -- It tries to implement a distance calculation between nodes which is simplistic and doesn't represent the way Kademlia does this. This might cause even more confusion as it is not well documented.
// -- Example code in `main.rs` is too verbose and redundant, it focuses heavily on redundant networking setup.

// * Presentation:
// -- More Markdown syntax is needed, there are none Markdown elements in the response, just plain text, this makes it hard for the user to understand the response. Propper formatting could have improved readability.
// -- It needs more balance between code and explanation.
// -- The  `main.rs` file where an example is given is completely redundant, it takes more space than the actual solution and it does not provide useful information.

// * Other issues
// -- The networking setup part is confusing as it takes too much space to just initialize the example network, it is redundant. It complicates the response unnecessarily.

// Next steps
// Fully implement the Kademlia algorithm by:

// Reading and gathering more information about how Kademlia works.
// Implement routing tables, lookup methods, XOR-based distance metrics.
// To store data: The node performs hashing of the key and then searches for the K nearest neighbors in the routing table, to then replicate the data in these nodes. This decentralizes the data.
// *** To get data:*** Performs an iterative lookup process. It asks to the K neighbors that are close to the hashed key, by doing this it retrieves the data or ensures that it is not available.


// Changes made:
// Added more documentation about how the Kademlia protocol works.
// Added Next Steps documentation in order to help the user know that there is still work to be done to complete the Kademlia algorithm. And how the user can continue from the given answer.
// Improved documentation on how Kademlia maintains a hash table distributed and how this helps decentralization.
