//! Used to send the score to a website hosting a scoreboard

extern crate hyper;
use self::hyper::client::Client;

pub fn send( name: &str, score: u32 ) -> bool {
	let client = Client::new();
	let header = hyper::header::ContentType::form_url_encoded();
	let checksum = (score/16) * (score/16);
	let mut packet_body = String::from("playername=");
	packet_body.push_str(name);
	packet_body.push_str("&score=");
	packet_body.push_str( &score.to_string() );
	packet_body.push_str("&checksum=");
	packet_body.push_str( &checksum.to_string() );
	
	let res = client.post("http://cheerthemup.co.nf/scoreboard/50-2/")
		.body( &packet_body )
		.header(header)
		.send();
	if res.is_ok() { 
		let res = res.unwrap();
		let status = res.status;
		if status.is_success() { return true; }
	}
	false
}
