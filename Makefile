.PHONY: certs server client

# Generate SSL certificates
certs:
	@echo "Generating SSL certificates..."
	@mkdir -p ssl
	# 1. Generate root CA
	openssl genrsa -out ssl/ca.key 4096
	openssl req -x509 -new -nodes -key ssl/ca.key -sha256 -days 365 -out ssl/ca.crt -subj "/CN=RootCA"

	# 2. Generate server cert/key
	openssl genrsa -out ssl/server.key 2048
	openssl req -new -key ssl/server.key -out ssl/server.csr -subj "/CN=localhost"
	openssl x509 -req -in ssl/server.csr -CA ssl/ca.crt -CAkey ssl/ca.key -CAcreateserial -out ssl/server.crt -days 365 -sha256

	# 3. Generate client cert/key
	openssl genrsa -out ssl/client.key 2048
	openssl req -new -key ssl/client.key -out ssl/client.csr -subj "/CN=client"
	openssl x509 -req -in ssl/client.csr -CA ssl/ca.crt -CAkey ssl/ca.key -CAcreateserial -out ssl/client.crt -days 365 -sha256
	@echo "✅ SSL certificates generated in ssl/ directory"

# Run the server
server:
	@echo "Starting gRPC server..."
	cargo run --bin server

# Run the client
client:
	@echo "Starting gRPC client..."
	cargo run --bin client