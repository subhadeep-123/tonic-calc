CERTS_DIR=certs
KEY_FILE=$(CERTS_DIR)/server.key
CERT_FILE=$(CERTS_DIR)/server.crt
CA_KEY=$(CERTS_DIR)/ca.key
CA_CERT=$(CERTS_DIR)/ca.crt

.PHONY: all certs clean run-server run-client

all: certs

certs:
	@echo "Generating SSL certificates..."
	@mkdir -p $(CERTS_DIR)
	# 1. Generate root CA
	openssl genrsa -out $(CA_KEY) 4096
	openssl req -x509 -new -nodes -key $(CA_KEY) -sha256 -days 3650 -out $(CA_CERT) -subj "/CN=MyCA"

	# 2. Generate server cert/key
	# Generate server key and CSR
	openssl genrsa -out $(KEY_FILE) 4096
	openssl req -new -key $(KEY_FILE) -out $(CERTS_DIR)/server.csr -subj "/CN=localhost"
	
	# 3. Sign server cert with CA
	openssl x509 -req -in $(CERTS_DIR)/server.csr -CA $(CA_CERT) -CAkey $(CA_KEY) -CAcreateserial -out $(CERT_FILE) -days 365 -sha256
	@echo "✅ TLS certificates generated in $(CERTS_DIR)"@echo "✅ SSL certificates generated in ssl/ directory"
	

server:
	cargo run --bin server

client:
	cargo run --bin client

clean:
	rm -rf $(CERTS_DIR)
	cargo clean
