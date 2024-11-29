// IMPLEMENTATION A
// Define the possible messages in the Paxos protocol
// #[derive(Debug)]
// enum PaxosMessage {
//     Prepare(u64, u64), // (proposal number, proposer id)
//     Promise(u64, u64, Option<u64>), // (proposal number, acceptor id, highest proposal number)
//     Accept(u64, u64, u64), // (proposal number, proposer id, value)
//     Ack(u64, u64), // (proposal number, acceptor id)
//     Learn(u64, u64), // (proposal number, value)
// }
// // Define a Paxos node
// struct PaxosNode {
//     node_id: u64,
//     proposal_number: u64,
//     highest_proposal_number: u64,
//     value: u64,
//     accepted: bool,
// }

// impl PaxosNode {
//     // Create a new Paxos node
//     fn new(node_id: u64) -> Self {
//         PaxosNode {
//             node_id,
//             proposal_number: 0,
//             highest_proposal_number: 0,
//             value: 0,
//             accepted: false,
//         }
//     }

//     // Handle a Prepare message
//     fn handle_prepare(&mut self, proposal_number: u64, proposer_id: u64) -> PaxosMessage {
//         if proposal_number > self.highest_proposal_number {
//             self.highest_proposal_number = proposal_number;
//             PaxosMessage::Promise(proposal_number, self.node_id, None)
//         } else {
//             PaxosMessage::Promise(proposal_number, self.node_id, Some(self.highest_proposal_number))
//         }
//     }

//     // Handle an Accept message
//     fn handle_accept(&mut self, proposal_number: u64, proposer_id: u64, value: u64) -> PaxosMessage {
//         if proposal_number >= self.highest_proposal_number {
//             self.value = value;
//             self.accepted = true;
//             PaxosMessage::Ack(proposal_number, self.node_id)
//         } else {
//             PaxosMessage::Ack(0, 0) // reject the proposal
//         }
//     }
// }

// // Define a Paxos proposer
// struct PaxosProposer {
//     node_id: u64,
//     proposal_number: u64,
//     value: u64,
//     accepted: bool,
// }

// impl PaxosProposer {
//     // Create a new Paxos proposer
//     fn new(node_id: u64, value: u64) -> Self {
//         PaxosProposer {
//             node_id,
//             proposal_number: 1,
//             value,
//             accepted: false,
//         }
//     }

//     // Send a Prepare message to the acceptors
//     fn send_prepare(&self) -> PaxosMessage {
//         PaxosMessage::Prepare(self.proposal_number, self.node_id)
//     }

//     // Handle a Promise message
//     fn handle_promise(&mut self, promise: PaxosMessage) {
//         match promise {
//             PaxosMessage::Promise(proposal_number, _, _) => {
//                 if proposal_number == self.proposal_number {
//                     // send an Accept message to the acceptors
//                     println!("Sending Accept message");
//                 } else {
//                     println!("Received promise for different proposal number");
//                 }
//             }
//             _ => println!("Received unexpected message"),
//         }
//     }

//     // Send an Accept message to the acceptors
//     fn send_accept(&self) -> PaxosMessage {
//         PaxosMessage::Accept(self.proposal_number, self.node_id, self.value)
//     }

//     // Handle an Ack message
//     fn handle_ack(&mut self, ack: PaxosMessage) {
//         match ack {
//             PaxosMessage::Ack(proposal_number, _) => {
//                 if proposal_number == self.proposal_number {
//                     self.accepted = true;
//                     println!("Proposal accepted");
//                 } else {
//                     println!("Received ack for different proposal number");
//                 }
//             }
//             _ => println!("Received unexpected message"),
//         }
//     }
// }
// fn main() {
//     let mut proposer = PaxosProposer::new(1, 10);
//     let mut acceptor = PaxosNode::new(2);

//     // Send a Prepare message to the acceptor
//     let prepare_message = proposer.send_prepare();
//     println!("Sending Prepare message: {:?}", prepare_message);

//     // Handle the Prepare message at the acceptor
//     match prepare_message {
//         PaxosMessage::Prepare(proposal_number, proposer_id) => {
//             let promise_message = acceptor.handle_prepare(proposal_number, proposer_id);
//             println!("Sending Promise message: {:?}", promise_message);
//             proposer.handle_promise(promise_message);
//         }
//         _ => println!("Received unexpected message"),
//     }

//     // Send an Accept message to the acceptor
//     let accept_message = proposer.send_accept();
//     println!("Sending Accept message: {:?}", accept_message);

//     // Handle the Accept message at the acceptor
//     match accept_message {
//         PaxosMessage::Accept(proposal_number, proposer_id, value) => {
//             let ack_message = acceptor.handle_accept(proposal_number, proposer_id, value);
//             println!("Sending Ack message: {:?}", ack_message);
//             proposer.handle_ack(ack_message);
//         }
//         _ => println!("Received unexpected message"),
//     }
// }


// IMPLEMENTATION B
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
