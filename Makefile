CERTS_DIR=tls
CA_KEY=$(CERTS_DIR)/ca.key
CA_CERT=$(CERTS_DIR)/ca.crt
SERVER_KEY=$(CERTS_DIR)/server.key
SERVER_CSR=$(CERTS_DIR)/server.csr
SERVER_CERT=$(CERTS_DIR)/server.crt

.PHONY: certs clean server client

certs:
	@echo "🔐 Generating TLS certificates..."
	@mkdir -p $(CERTS_DIR)

	# 1. Generate Root CA Key and Cert
	openssl genrsa -out $(CA_KEY) 4096
	openssl req -x509 -new -nodes -key $(CA_KEY) -sha256 -days 3650 -out $(CA_CERT) -subj "/CN=MyCA"

	# 2. Generate Server Key and CSR with SAN
	openssl genrsa -out $(SERVER_KEY) 4096
	openssl req -new -key $(SERVER_KEY) -out $(SERVER_CSR) -config $(CERTS_DIR)/openssl.cnf

	# 3. Sign server certificate with proper extensions
	openssl x509 -req -in $(SERVER_CSR) -CA $(CA_CERT) -CAkey $(CA_KEY) -CAcreateserial \
	-out $(SERVER_CERT) -days 365 -sha256 \
	-extfile $(CERTS_DIR)/openssl.cnf -extensions v3_ext

	@echo "✅ Certificates created in $(CERTS_DIR)"

clean:
	rm -f $(CERTS_DIR)/*.key
	rm -f $(CERTS_DIR)/*.crt
	rm -f $(CERTS_DIR)/*.csr
	rm -f $(CERTS_DIR)/*.srl
	cargo clean

server:
	RUST_LOG=info cargo run --bin server

client:
	RUST_LOG=info cargo run --bin client
