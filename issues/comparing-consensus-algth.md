#  Comparative Analysis of Consensus Algorithms in Rust

## Use Case 

A developer is tasked with implementing a basic version of two consensus algorithms, Raft and Paxos, in Rust to compare their performance in a distributed system. How can they structure their code to ensure that both algorithms can be tested under similar conditions, and what metrics should they focus on for the comparative analysis?

# Raft Consensus Algorithm Documentation
Overview
Raft is a consensus algorithm designed to manage a distributed state machine across a cluster of nodes. It was developed by Diego Ongaro and John Ousterhout at Stanford University in 2013. Raft is designed to be more understandable and easier to implement than other consensus algorithms, such as Paxos.

Objectives
The primary objectives of Raft are:

Safety: Ensure that the distributed state machine remains consistent across all nodes in the cluster.
Availability: Ensure that the cluster remains available and can continue to process requests even in the presence of failures.
Performance: Minimize the overhead of consensus and allow the cluster to process requests efficiently.
How Raft Works
Raft works by maintaining a distributed log of commands, which represents the state of the system. Each node in the cluster maintains a copy of the log, and the nodes agree on the contents of the log through a consensus protocol.

Raft Components
A Raft cluster consists of several components:

Leader: The leader node is responsible for managing the distributed log and replicating it to other nodes in the cluster.
Followers: Follower nodes replicate the leader's log and can become the leader if the current leader fails.
Candidates: Candidate nodes are nodes that are attempting to become the leader.
Raft States
Each node in the cluster can be in one of three states:

Leader: The node is the current leader and is responsible for managing the distributed log.
Follower: The node is a follower and replicates the leader's log.
Candidate: The node is a candidate and is attempting to become the leader.
Raft Messages
Raft uses several types of messages to communicate between nodes:

RequestVote: A candidate node sends a RequestVote message to other nodes to request their vote.
Vote: A node sends a Vote message to a candidate node to grant its vote.
AppendEntries: The leader node sends an AppendEntries message to follower nodes to replicate the log.
Heartbeat: The leader node sends a periodic Heartbeat message to follower nodes to maintain its leadership.
Raft Protocol
The Raft protocol consists of several phases:

Leader Election: When a node starts up, it begins in the follower state. If it doesn't receive a heartbeat from the leader within a certain time period (called the election timeout), it becomes a candidate and sends a RequestVote message to other nodes. If a majority of nodes grant their vote, the candidate becomes the new leader.
Log Replication: The leader node maintains a log of commands and replicates it to follower nodes using AppendEntries messages. Followers acknowledge the receipt of each log entry, and the leader considers the entry committed once a majority of followers have acknowledged it.
Heartbeats: The leader node sends periodic Heartbeat messages to follower nodes to maintain its leadership. If a follower node doesn't receive a heartbeat within the election timeout, it becomes a candidate and attempts to become the leader.
Raft Log
The Raft log is a sequence of log entries, each representing a command. Each log entry has a unique index and a term number. The term number represents the leader's term, and it increments each time a new leader is elected.

Log Replication
The leader node replicates the log to follower nodes using AppendEntries messages. Each AppendEntries message contains a set of log entries, and the follower node acknowledges the receipt of each entry. The leader considers an entry committed once a majority of followers have acknowledged it.

Leader Election
Leader election is the process by which a new leader is chosen when the current leader fails or becomes unavailable. The election process involves the following steps:

Candidate Election: A follower node becomes a candidate and sends a RequestVote message to other nodes.
Voting: Each node grants its vote to the candidate node with the highest term and index.
Leader Election: If a candidate node receives a majority of votes, it becomes the new leader.
Safety Properties
Raft ensures the following safety properties:

Leader Completeness: If a log entry is committed, it will be present in the leader's log.
Log Matching: All nodes agree on the contents of the log up to the committed index.
Leader Append-Only: A leader node can only append new entries to the log.
Why Raft Was Developed
Raft was developed to address the complexity and difficulty of implementing consensus algorithms like Paxos. Raft is designed to be more understandable and easier to implement, making it a more accessible choice for building distributed systems.

What New Stuff Raft Brings to the Table
Raft introduces several innovations that make it more practical and efficient than other consensus algorithms:

Simplified State Machine: Raft uses a simplified state machine that is easier to understand and implement.
Leader Election: Raft's leader election protocol is designed to be more efficient and less prone to split brain scenarios.
Log Replication: Raft's log replication protocol is optimized for performance and minimizes the overhead of consensus.
Conclusion
Raft is a consensus algorithm designed to manage a distributed state machine across a cluster of nodes. It is designed to be more understandable and easier to implement than other consensus algorithms, making it a popular choice for building distributed systems. Raft ensures safety properties like leader completeness, log matching, and leader append-only, and it introduces innovations like simplified state machines, leader election, and log replication.

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone)]
// Define the possible states of a Raft node
enum RaftState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Clone)]
// Define the Raft node struct
struct RaftNode {
    node_id: u64,
    state: RaftState,
    current_term: u64,
    voted_for: Option<u64>,
    log: Vec<LogEntry>,
    commit_index: u64,
    last_applied: u64,
    election_timeout: Duration,
    heartbeat_timeout: Duration,
    last_heartbeat: Instant,
    peers: HashMap<u64, RaftNode>,
}

impl RaftNode {
    // Create a new Raft node
    fn new(node_id: u64) -> Self {
        RaftNode {
            node_id,
            state: RaftState::Follower,
            current_term: 0,
            voted_for: None,
            log: vec![],
            commit_index: 0,
            last_applied: 0,
            election_timeout: Duration::from_millis(150),
            heartbeat_timeout: Duration::from_millis(50),
            last_heartbeat: Instant::now(),
            peers: HashMap::new(),
        }
    }

    // Add a peer node to this node's peer list
    fn add_peer(&mut self, peer: RaftNode) {
        self.peers.insert(peer.node_id, peer);
    }
}

#[derive(Clone)]
// Define a log entry
struct LogEntry {
    term: u64,
    command: String,
}

// Define the RequestVote message
struct RequestVote {
    term: u64,
    candidate_id: u64,
    last_log_index: u64,
    last_log_term: u64,
}

impl RequestVote {
    // Create a new RequestVote message
    fn new(term: u64, candidate_id: u64, last_log_index: u64, last_log_term: u64) -> Self {
        RequestVote {
            term,
            candidate_id,
            last_log_index,
            last_log_term,
        }
    }
}

#[derive(Clone, Debug)]
// Define the Vote message
struct Vote {
    term: u64,
    voter_id: u64,
}

impl Vote {
    // Create a new Vote message
    fn new(term: u64, voter_id: u64) -> Self {
        Vote { term, voter_id }
    }
}

// Define the AppendEntries message
struct AppendEntries {
    term: u64,
    leader_id: u64,
    prev_log_index: u64,
    prev_log_term: u64,
    entries: Vec<LogEntry>,
    leader_commit: u64,
}

impl AppendEntries {
    // Create a new AppendEntries message
    fn new(
        term: u64,
        leader_id: u64,
        prev_log_index: u64,
        prev_log_term: u64,
        entries: Vec<LogEntry>,
        leader_commit: u64,
    ) -> Self {
        AppendEntries {
            term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit,
        }
    }
}

impl RaftNode {
    // Start a new election
    fn start_election(&mut self) {
        self.state = RaftState::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.node_id);
        self.request_votes();
    }

    // Request votes from other nodes
    fn request_votes(&mut self) {
        for peer in self.peers.values_mut() {
            let request_vote = RequestVote::new(
                self.current_term,
                self.node_id,
                self.log.len() as u64,
                self.log.last().map_or(0, |entry| entry.term),
            );
            // Send the RequestVote message to the peer node
            // For simplicity, we'll assume the message is sent successfully
            // In a real implementation, you'd want to handle errors and retries
            peer.handle_request_vote(request_vote);
        }
    }

    // Handle a RequestVote message
    fn handle_request_vote(&mut self, request_vote: RequestVote) {
        if request_vote.term < self.current_term {
            // Reject the request if the term is outdated
            return;
        }

        if request_vote.term == self.current_term && self.voted_for.is_some() {
            // Reject the request if we've already voted for someone else
            return;
        }

        self.current_term = request_vote.term;
        self.voted_for = Some(request_vote.candidate_id);
        let vote = Vote::new(self.current_term, self.node_id);
        // Send the Vote message to the candidate node
        // For simplicity, we'll assume the message is sent successfully
        // In a real implementation, you'd want to handle errors and retries
        // For this example we will just print the vote
        println!("Vote: {:?}", vote);
    }
}

impl RaftNode {
    // Replicate the log to follower nodes
    fn replicate_log(&mut self) {
        for peer in self.peers.values_mut() {
            let append_entries = AppendEntries::new(
                self.current_term,
                self.node_id,
                self.log.len() as u64,
                self.log.last().map_or(0, |entry| entry.term),
                vec![],
                self.commit_index,
            );
            // Send the AppendEntries message to the peer node
            // For simplicity, we'll assume the message is sent successfully
            // In a real implementation, you'd want to handle errors and retries
            peer.handle_append_entries(append_entries);
        }
    }

    // Handle an AppendEntries message
    fn handle_append_entries(&mut self, append_entries: AppendEntries) {
        if append_entries.term < self.current_term {
            // Reject the request if the term is outdated
            return;
        }

        self.current_term = append_entries.term;
        self.commit_index = append_entries.leader_commit;
        // Append the new entries to the log
        self.log.extend(append_entries.entries);
    }
}

impl RaftNode {
    // Send a heartbeat to follower nodes
    fn send_heartbeat(&mut self) {
        for peer in self.peers.values_mut() {
            let append_entries = AppendEntries::new(
                self.current_term,
                self.node_id,
                self.log.len() as u64,
                self.log.last().map_or(0, |entry| entry.term),
                vec![],
                self.commit_index,
            );
            // Send the AppendEntries message to the peer node
            // For simplicity, we'll assume the message is sent successfully
            // In a real implementation, you'd want to handle errors and retries
            peer.handle_append_entries(append_entries);
        }
    }
}

fn main() {
    let mut node1 = RaftNode::new(1);
    let mut node2 = RaftNode::new(2);
    let mut node3 = RaftNode::new(3);

    node1.add_peer(node2.clone());
    node1.add_peer(node3.clone());
    node2.add_peer(node1.clone());
    node2.add_peer(node3.clone());
    node3.add_peer(node1.clone());
    node3.add_peer(node2.clone());

    node1.start_election();

    // Simulate some log replication
    node1.replicate_log();

    // Simulate a failure
    node2.state = RaftState::Follower;
    node2.current_term = 0;

    // Simulate some more log replication
    node1.replicate_log();
}
```

```rust
use std::collections::HashMap;
use std::time::Instant;

// Raft node states
enum State {
    Follower,
    Candidate,
    Leader,
}

// Raft node
struct Node {
    id: u32,
    state: State,
    current_term: u32,
    voted_for: Option<u32>,
    log: Vec<LogEntry>,
    peers: HashMap<u32, Node>,
    election_timeout: Instant,
}

impl Node {
    fn new(id: u32) -> Self {
        Node {
            id,
            state: State::Follower,
            current_term: 0,
            voted_for: None,
            log: vec![],
            peers: HashMap::new(),
            election_timeout: Instant::now(),
        }
    }
}

// Raft log entry
struct LogEntry {
    index: u32,
    term: u32,
    command: String,
}

impl LogEntry {
    fn new(index: u32, term: u32, command: String) -> Self {
        LogEntry { index, term, command }
    }
}

// RequestVote message
struct RequestVote {
    term: u32,
    candidate_id: u32,
    last_log_index: u32,
    last_log_term: u32,
}

impl RequestVote {
    fn new(term: u32, candidate_id: u32, last_log_index: u32, last_log_term: u32) -> Self {
        RequestVote {
            term,
            candidate_id,
            last_log_index,
            last_log_term,
        }
    }
}

// Vote message
struct Vote {
    term: u32,
    vote_granted: bool,
}

impl Vote {
    fn new(term: u32, vote_granted: bool) -> Self {
        Vote { term, vote_granted }
    }
}

// AppendEntries message
struct AppendEntries {
    term: u32,
    leader_id: u32,
    prev_log_index: u32,
    prev_log_term: u32,
    entries: Vec<LogEntry>,
    leader_commit: u32,
}

impl AppendEntries {
    fn new(
        term: u32,
        leader_id: u32,
        prev_log_index: u32,
        prev_log_term: u32,
        entries: Vec<LogEntry>,
        leader_commit: u32,
    ) -> Self {
        AppendEntries {
            term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit,
        }
    }
}

// Heartbeat message
struct Heartbeat {
    term: u32,
    leader_id: u32,
}

impl Heartbeat {
    fn new(term: u32, leader_id: u32) -> Self {
        Heartbeat { term, leader_id }
    }
}

impl Node {
    // Start a new election
    fn start_election(&mut self) {
        self.state = State::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.id);
        self.election_timeout = Instant::now();

        // Send RequestVote messages to all peers
        for peer in self.peers.values() {
            let request_vote = RequestVote::new(
                self.current_term,
                self.id,
                self.log.len() as u32,
                self.log.last().map_or(0, |entry| entry.term),
            );
            // Send the RequestVote message to the peer
            println!("Sending RequestVote to peer {}: {:?}", peer.id, request_vote);
        }
    }

    // Handle a received RequestVote message
    fn handle_request_vote(&mut self, request_vote: RequestVote) {
        if request_vote.term < self.current_term {
            // Reply with a Vote message with vote_granted set to false
            let vote = Vote::new(self.current_term, false);
            println!("Sending Vote to peer {}: {:?}", request_vote.candidate_id, vote);
            return;
        }

        if request_vote.term > self.current_term {
            // Update the current term and transition to the follower state
            self.current_term = request_vote.term;
            self.state = State::Follower;
            self.voted_for = None;
        }

        // Check if the candidate's log is at least as up-to-date as the current node's log
        if request_vote.last_log_index >= self.log.len() as u32
            && request_vote.last_log_term >= self.log.last().map_or(0, |entry| entry.term)
        {
            // Reply with a Vote message with vote_granted set to true
            let vote = Vote::new(self.current_term, true);
            println!("Sending Vote to peer {}: {:?}", request_vote.candidate_id, vote);
            self.voted_for = Some(request_vote.candidate_id);
        } else {
            // Reply with a Vote message with vote_granted set to false
            let vote = Vote::new(self.current_term, false);
            println!("Sending Vote to peer {}: {:?}", request_vote.candidate_id, vote);
        }
    }

    // Handle a received Vote message
    fn handle_vote(&mut self, vote: Vote) {
        if vote.term < self.current_term {
            // Ignore the Vote message
            return;
        }

        if vote.term > self.current_term {
            // Update the current term and transition to the follower state
            self.current_term = vote.term;
            self.state = State::Follower;
            self.voted_for = None;
        }

        // Check if the vote is granted
        if vote.vote_granted {
            // Increment the vote count
            let mut vote_count = 1;
            for peer in self.peers.values() {
                if peer.voted_for == Some(self.id) {
                    vote_count += 1;
                }
            }

            // Check if the node has received a majority of votes
            if vote_count >= (self.peers.len() + 1) / 2 + 1 {
                // Transition to the leader state
                self.state = State::Leader;
                println!("Elected as leader");
            }
        }
    }

    // Handle a received AppendEntries message
    fn handle_append_entries(&mut self, append_entries: AppendEntries) {
        if append_entries.term < self.current_term {
            // Reply with a failure response
            println!("Sending failure response to peer {}: {:?}", append_entries.leader_id, "Failure");
            return;
        }

        if append_entries.term > self.current_term {
            // Update the current term and transition to the follower state
            self.current_term = append_entries.term;
            self.state = State::Follower;
            self.voted_for = None;
        }

        // Check if the leader's log is at least as up-to-date as the current node's log
        if append_entries.prev_log_index >= self.log.len() as u32
            && append_entries.prev_log_term >= self.log.last().map_or(0, |entry| entry.term)
        {
            // Append the new log entries to the current node's log
            self.log.extend(append_entries.entries);
            println!("Appended log entries from peer {}: {:?}", append_entries.leader_id, append_entries.entries);
        } else {
            // Reply with a failure response
            println!("Sending failure response to peer {}: {:?}", append_entries.leader_id, "Failure");
        }
    }

    // Handle a received Heartbeat message
    fn handle_heartbeat(&mut self, heartbeat: Heartbeat) {
        if heartbeat.term < self.current_term {
            // Ignore the Heartbeat message
            return;
        }

        if heartbeat.term > self.current_term {
            // Update the current term and transition to the follower state
            self.current_term = heartbeat.term;
            self.state = State::Follower;
            self.voted_for = None;
        }

        // Reset the election timeout
        self.election_timeout = Instant::now();
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);

    node1.peers.insert(2, node2.clone());
    node1.peers.insert(3, node3.clone());

    node2.peers.insert(1, node1.clone());
    node2.peers.insert(3, node3.clone());

    node3.peers.insert(1, node1.clone());
    node3.peers.insert(2, node2.clone());

    node1.start_election();

    // Simulate a RequestVote message from node1 to node2
    let request_vote = RequestVote::new(1, 1, 0, 0);
    node2.handle_request_vote(request_vote);

    // Simulate a Vote message from node2 to node1
    let vote = Vote::new(1, true);
    node1.handle_vote(vote);

    // Simulate an AppendEntries message from node1 to node2
    let append_entries = AppendEntries::new(1, 1, 0, 0, vec![LogEntry::new(1, 1, "Command1".to_string())], 0);
    node2.handle_append_entries(append_entries);

    // Simulate a Heartbeat message from node1 to node2
    let heartbeat = Heartbeat::new(1, 1);
    node2.handle_heartbeat(heartbeat);
}
```

# Paxos consensus algorithm

Paxos Consensus Algorithm Documentation
Overview
Paxos is a consensus algorithm designed to achieve agreement among a group of distributed processes on a single value. It was developed by Leslie Lamport in 1998 as a solution to the problem of achieving consensus in a fault-tolerant distributed system.

Objectives
The primary objectives of Paxos are:

Safety: Ensure that the agreed-upon value is consistent across all processes in the system.
Liveness: Ensure that the system can make progress and eventually reach agreement, even in the presence of failures.
Fault Tolerance: Ensure that the system can recover from failures and continue to operate correctly.
How Paxos Works
Paxos works by having a group of processes, called proposers, propose values to a group of acceptors. The acceptors vote on the proposed values, and a value is considered accepted if it receives a majority of votes.

Paxos Components
A Paxos system consists of several components:

Proposers: Proposers are processes that propose values to the acceptors.
Acceptors: Acceptors are processes that vote on the proposed values.
Learners: Learners are processes that learn the agreed-upon value.
Paxos Messages
Paxos uses several types of messages to communicate between processes:

Prepare: A prepare message is sent by a proposer to the acceptors to prepare them for a proposal.
Promise: A promise message is sent by an acceptor to a proposer to indicate that it is willing to vote for the proposed value.
Accept: An accept message is sent by a proposer to the acceptors to request that they vote for the proposed value.
Ack: An ack message is sent by an acceptor to a proposer to indicate that it has voted for the proposed value.
Learn: A learn message is sent by an acceptor to the learners to inform them of the agreed-upon value.
Paxos Protocol
The Paxos protocol consists of several phases:

Prepare: A proposer sends a prepare message to the acceptors to prepare them for a proposal. The prepare message includes a proposal number, which is used to identify the proposal.
Promise: An acceptor responds to the prepare message with a promise message, indicating that it is willing to vote for the proposed value. The promise message includes the proposal number and the highest proposal number that the acceptor has previously promised to vote for.
Accept: If a majority of acceptors respond with promise messages, the proposer sends an accept message to the acceptors, requesting that they vote for the proposed value.
Ack: An acceptor responds to the accept message with an ack message, indicating that it has voted for the proposed value.
Learn: If a majority of acceptors respond with ack messages, the proposer sends a learn message to the learners, informing them of the agreed-upon value.
Paxos Properties
Paxos has several properties that make it a robust and reliable consensus algorithm:

Validity: If a value is proposed, it will eventually be accepted or rejected.
Agreement: If a value is accepted, it will be accepted by all processes in the system.
Termination: The algorithm will eventually terminate, even in the presence of failures.
Why Paxos Was Developed
Paxos was developed to address the problem of achieving consensus in a fault-tolerant distributed system. Traditional consensus algorithms, such as two-phase commit, were not fault-tolerant and could not handle failures.

What New Stuff Paxos Brings to the Table
Paxos introduces several new concepts that make it a robust and reliable consensus algorithm:

Proposal numbers: Paxos uses proposal numbers to identify proposals and ensure that only one proposal can be accepted at a time.
Majority voting: Paxos uses majority voting to ensure that a value is accepted only if it receives a majority of votes.
Fault tolerance: Paxos is designed to be fault-tolerant and can handle failures of proposers, acceptors, and learners.
Paxos Variants
There are several variants of Paxos that have been developed to address specific use cases:

Multi-Paxos: Multi-Paxos is a variant of Paxos that allows multiple values to be proposed and accepted concurrently.
Byzantine Paxos: Byzantine Paxos is a variant of Paxos that is designed to handle Byzantine failures, where a process can behave arbitrarily.
Disk Paxos: Disk Paxos is a variant of Paxos that uses disk storage to persist the state of the system.
Conclusion
Paxos is a robust and reliable consensus algorithm that is widely used in distributed systems. Its properties, such as validity, agreement, and termination, make it a popular choice for achieving consensus in fault-tolerant systems. The algorithm's use of proposal numbers, majority voting, and fault tolerance make it a significant improvement over traditional consensus algorithms.

# Advantages and Disadvantages
##Advantages

- **Fault Tolerance:** Paxos is designed to handle failures, making it a reliable choice for distributed systems.
- **Asynchronous Communication:** Paxos can handle asynchronous communication, which makes it more suitable for distributed systems.
- **Simple and Intuitive:** Paxos is relatively simple and intuitive, making it easier to understand and implement.

## Disadvantages:

- **Complexity:**  While Paxos is generally simple and intuitive, it can be complex to implement in practice.
- **Performance:** Paxos can have high latency and low throughput, especially in large-scale systems.
- **Liveness:** Paxos does not guarantee liveness, which means that the algorithm may not terminate in certain scenarios.


```rust

// Define the Paxos message enum
enum PaxosMessage {
    Prepare(Prepare),
    Promise(Promise),
    Accept(Accept),
    Ack(Ack),
    Learn(Learn),
}

// Define the Prepare message
struct Prepare {
    proposal_number: u64,
}

#[derive(Debug)]
// Define the Promise message
struct Promise {
    proposal_number: u64,
    highest_proposal_number: u64,
}

#[derive(Debug)]
// Define the Accept message
struct Accept {
    proposal_number: u64,
    value: String,
}

#[derive(Debug)]
// Define the Ack message
struct Ack {
    proposal_number: u64,
}

#[derive(Debug)]
// Define the Learn message
struct Learn {
    value: String,
}

use std::collections::HashMap;

// Define the Paxos node struct
struct PaxosNode {
    node_id: u64,
    proposal_number: u64,
    highest_proposal_number: u64,
    accepted_value: Option<String>,
    log: Vec<PaxosMessage>,
    peers: HashMap<u64, PaxosNode>,
}

impl PaxosNode {
    // Create a new Paxos node
    fn new(node_id: u64) -> Self {
        PaxosNode {
            node_id,
            proposal_number: 0,
            highest_proposal_number: 0,
            accepted_value: None,
            log: vec![],
            peers: HashMap::new(),
        }
    }

    // Handle an incoming message
    fn handle_message(&mut self, message: PaxosMessage) {
        match message {
            PaxosMessage::Prepare(prepare) => self.handle_prepare(prepare),
            PaxosMessage::Promise(promise) => self.handle_promise(promise),
            PaxosMessage::Accept(accept) => self.handle_accept(accept),
            PaxosMessage::Ack(ack) => self.handle_ack(ack),
            PaxosMessage::Learn(learn) => self.handle_learn(learn),
        }
    }

    // Handle a Prepare message
    fn handle_prepare(&mut self, prepare: Prepare) {
        if prepare.proposal_number > self.highest_proposal_number {
            self.highest_proposal_number = prepare.proposal_number;
            let promise = Promise {
                proposal_number: prepare.proposal_number,
                highest_proposal_number: self.highest_proposal_number,
            };
            // Send the Promise message to the proposer
            // For simplicity, we'll assume the message is sent successfully
            // In a real implementation, you'd want to handle errors and retries
            println!("Promise: {:?}", promise);
        }
    }

    // Handle a Promise message
    fn handle_promise(&mut self, promise: Promise) {
        if promise.proposal_number == self.proposal_number {
            // A majority of nodes have promised to vote for our proposal
            // We can now send an Accept message
            let accept = Accept {
                proposal_number: self.proposal_number,
                value: "example_value".to_string(), // For simplicity, we'll use a hardcoded value
            };
            // Send the Accept message to the acceptors
            // For simplicity, we'll assume the message is sent successfully
            // In a real implementation, you'd want to handle errors and retries
            println!("Accept: {:?}", accept);
        }
    }

    // Handle an Accept message
    fn handle_accept(&mut self, accept: Accept) {
        if accept.proposal_number >= self.highest_proposal_number {
            self.highest_proposal_number = accept.proposal_number;
            self.accepted_value = Some(accept.value.clone());
            let ack = Ack {
                proposal_number: accept.proposal_number,
            };
            // Send the Ack message to the proposer
            // For simplicity, we'll assume the message is sent successfully
            // In a real implementation, you'd want to handle errors and retries
            println!("Ack: {:?}", ack);
        }
    }

    // Handle an Ack message
    fn handle_ack(&mut self, ack: Ack) {
        if ack.proposal_number == self.proposal_number {
            // A majority of nodes have acknowledged our proposal
            // We can now send a Learn message
            let learn = Learn {
                value: self.accepted_value.clone().unwrap(),
            };
            // Send the Learn message to the learners
            // For simplicity, we'll assume the message is sent successfully
            // In a real implementation, you'd want to handle errors and retries
            println!("Learn: {:?}", learn);
        }
    }

    // Handle a Learn message
    fn handle_learn(&mut self, learn: Learn) {
        // Update our state with the learned value
        self.accepted_value = Some(learn.value);
    }
}

fn main() {
    let mut node1 = PaxosNode::new(1);
    let mut node2 = PaxosNode::new(2);
    let mut node3 = PaxosNode::new(3);

    // Simulate the Paxos protocol
    let prepare = Prepare { proposal_number: 1 };
    node1.handle_message(PaxosMessage::Prepare(prepare));

    let promise = Promise {
        proposal_number: 1,
        highest_proposal_number: 0,
    };
    node1.handle_message(PaxosMessage::Promise(promise));

    let accept = Accept {
        proposal_number: 1,
        value: "example_value".to_string(),
    };
    node1.handle_message(PaxosMessage::Accept(accept));

    let ack = Ack { proposal_number: 1 };
    node1.handle_message(PaxosMessage::Ack(ack));

    let learn = Learn {
        value: "example_value".to_string(),
    };
    node1.handle_message(PaxosMessage::Learn(learn));
}

``` 
