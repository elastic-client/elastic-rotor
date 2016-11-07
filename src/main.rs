//! # `elastic_rotor`
//! 
//! A WIP implementation of an asynchronous http client for Elasticsearch.
//! 
//! Only _sort of_ works... But will follow the following design:
//! - Provide a simple, fast constant connection pool
//! - Provide a more complex, but robust, sniffed connection pool
//! 
//! Communication to the loop is through a non-blocking `Queue`, wrapped in a `Client`.

extern crate time;

extern crate crossbeam;
extern crate futures;
extern crate rotor;
extern crate rotor_http;
extern crate rotor_tools;

#[macro_use]
extern crate lazy_static;

mod client;
pub use client::*;

//Test usage
use futures::Future;

lazy_static! {
	static ref QUEUE: Queue = Queue::new();
}

fn main() {
	// Build a client
	let builder = ClientBuilder::new(&QUEUE)
		.connect_localhost()
		.connect_localhost()
		.build();

	// Index some data
	let post = builder.and_then(|cli| {
			cli.req(Request::post("/testindex/testtype/1", b"{\"id\":1}"))
			   .and_then(|result| {
			   		match result {
			   			Ok(data) => print_res(data),
			   			Err(e) => println!("Error: {}", e)
			   		}

			   		futures::finished(cli)
			   })
			});

	// Search some data
	let search = post.and_then(|cli| {
		futures::collect((0..5).map(move |_| {
			cli.req(Request::get("/testindex/testtype/_search"))
			   .and_then(|result| {
					match result {
						Ok(data) => print_res(data),
						Err(e) => println!("Error: {}", e)
					}

					futures::finished(())
			   })
		}))
	});

	search.wait().unwrap();
}

fn print_res(res: Vec<u8>) {
	let res = ::std::str::from_utf8(&res).unwrap();
	println!("{}", res);
}