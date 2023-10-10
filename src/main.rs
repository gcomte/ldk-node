use bitcoin::bech32;
use bitcoin::bech32::FromBase32;
use colored::Colorize;
use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::bitcoin::Network;
use ldk_node::io::SqliteStore;
use ldk_node::{Builder, NetAddress, Node};
use lightning_invoice::Bolt11Invoice;
use lnurl::LnUrlResponse;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::str::FromStr;

const LIPA_NODE_ID: &str = "03f984c30b10c63f18732756d42c6e0d73507763feb4180b5bd785d2dc4a35db75";
const LIPA_NODE_ADDRESS: &str = "34.65.188.150:9735";
const ANDREIS_LNURLP: &str = "LNURL1DP68GURN8GHJ7AR9WD6XUET59ECXCETZDEJHGTNYV4MZ7MRWW4EXCUP0GACNVDT62UMGXDKN";

fn main() {
	let mut builder = Builder::new();
	builder.set_network(Network::Testnet);
	builder.set_esplora_server("https://blockstream.info/testnet/api".to_string());

	// Dont use RGS!
	/* builder.set_gossip_source_rgs(
		"https://rapidsync.lightningdevkit.org/testnet/snapshot".to_string(),
	); */
	builder.set_gossip_source_p2p();

	// Builds a node using default configuration:
	// https://github.com/lightningdevkit/ldk-node/blob/0c137264975e02757cf2b4a17de116d12a8c8296/src/lib.rs#L261
	// Stores data in /tmp/ldk_node/
	let node = builder.build().unwrap();

	node.start().unwrap();
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
				"lnurlp" => {
					lnurlp(&node, &mut words);
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

fn lnurlp(node: &Node<SqliteStore>, words: &mut dyn Iterator<Item = &str>) {
	let amount_sat = words.next().unwrap_or("1").parse::<u64>().unwrap();
	let amount_msat = amount_sat * 1000;

	let client = lnurl::Builder { proxy: None, timeout: Some(10_000) }.build_blocking().unwrap();

	let url = decode_bech32(ANDREIS_LNURLP);
	println!("LNURL decoded URL: {:?}", url);

	if let LnUrlResponse::LnUrlPayResponse(response) = client.make_request(&url).unwrap() {
		println!("LNURL response: {:?}", response);

		let pay_result = client.get_invoice(&response, amount_msat, None).unwrap();
		println!("pay_result: {:?}", pay_result);

		let invoice = Bolt11Invoice::from_str(&pay_result.invoice()).unwrap();
		println!("invoice: {invoice}");

		node.send_payment(&invoice).unwrap();
	}
}

fn decode_bech32(payload: &str) -> String {
	let raw = bech32::decode(payload)
		.expect(&*format!("Could not decode bech32: {payload} - Invalid bech32"));

	let bytes = Vec::<u8>::from_base32(&raw.1)
		.expect(&*format!("Could not decode Bech32: {payload} - Invalid base32",));

	String::from_utf8(bytes)
		.expect(&*format!("Could not decode Bech32: {payload} - Could not parse to String",))
}
