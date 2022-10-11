all:
	cargo run -- upload -e testuser@email.com -p password -s http://10.1.15.216:2283/api  -d ./test-files