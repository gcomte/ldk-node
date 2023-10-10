use colored::Colorize;
use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::bitcoin::Network;
use ldk_node::io::SqliteStore;
use ldk_node::{Builder, NetAddress, Node};
use std::str::FromStr;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() {
	const LIPA_NODE_ID: &str = "03f984c30b10c63f18732756d42c6e0d73507763feb4180b5bd785d2dc4a35db75";
	const LIPA_NODE_ADDRESS: &str = "34.65.188.150:9735";

	let mut builder = Builder::new();
	builder.set_network(Network::Testnet);
	builder.set_esplora_server("https://blockstream.info/testnet/api".to_string());
	builder.set_gossip_source_rgs(
		"https://rapidsync.lightningdevkit.org/testnet/snapshot".to_string(),
	);

	// Builds a node using default configuration:
	// https://github.com/lightningdevkit/ldk-node/blob/0c137264975e02757cf2b4a17de116d12a8c8296/src/lib.rs#L261
	// Stores data in /tmp/ldk_node/
	let node = builder.build().unwrap();

	node.start().unwrap();

	// print node id:
	println!("NODE ID: {}", node.node_id());

	// Make LN protocol handshake with LIPA node (only connects, doesn't open any channel)
	let node_id = PublicKey::from_str(LIPA_NODE_ID).unwrap();
	let address = NetAddress::from_str(LIPA_NODE_ADDRESS).unwrap();
	node.connect(node_id, address, false).unwrap();

	poll_for_user_input(&node);

	/*
		let event = node.wait_next_event();
		println!("EVENT: {:?}", event);
		node.event_handled();

		let invoice = Bolt11Invoice::from_str("INVOICE_STR").unwrap();
		node.send_payment(&invoice).unwrap();
	*/

	node.stop().unwrap();
}

pub(crate) fn poll_for_user_input(node: &Node<SqliteStore>) {
	println!("{}", "T:U:S:C:A:N:Y sample lightning node console".blue().bold());

	let mut rl = DefaultEditor::new().unwrap();
	let prompt = "ϟ ".bold().blue().to_string();

	loop {
		let readline = rl.readline(&prompt);
		let line = match readline {
			Ok(line) => line,
			Err(ReadlineError::Interrupted) => {
				println!("CTRL-C");
				break;
			}
			Err(ReadlineError::Eof) => {
				println!("CTRL-D");
				break;
			}
			Err(err) => {
				println!("Error: {:?}", err);
				break;
			}
		};

		let mut words = line.split_whitespace();
		if let Some(word) = words.next() {
			match word {
				"help" => help(),
				"nodeinfo" => node_info(node),
				"stop" => {
					break;
				}
				_ => println!("{}", "Unknown command. See \"help\" for available commands.".red()),
			}
		}
	}
}

fn node_info(node: &Node<SqliteStore>) {
	println!("NODE ID: {}", node.node_id());
}

fn help() {
	println!("  nodeinfo");
	println!();
	println!("  stop");
}
