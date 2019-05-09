fn post_image() {
	let result = reqwest::Client::new().get(url).send().unwrap();
	println!("{:?}", result);
}