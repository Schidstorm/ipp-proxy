REMOTE_FILE=".remote"
REMOTE_PATH="/tmp/ipp-proxy"

install: build
	cargo install --path . --color always && \
	sudo cp systemd/ipp-proxy.service /etc/systemd/system/ && \
	sudo systemctl daemon-reload && \
	sudo systemctl enable ipp-proxy && \
	sudo systemctl restart ipp-proxy

remoteBuild: upload
	ssh $(shell "cat" "$(REMOTE_FILE)") "cd $(REMOTE_PATH) && make install"

upload:
	rsync -avr --delete --exclude target . $(shell "cat" "$(REMOTE_FILE)"):$(REMOTE_PATH)

build:
	cargo build --color always

run:
	sudo ./target/debug/ipp-proxy