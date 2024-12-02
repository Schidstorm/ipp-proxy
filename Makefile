REMOTE_FILE=".remote"
REMOTE_PATH="/tmp/ipp-proxy"

install: build disable_cups_631
	cargo install --path . --color always && \
	sudo cp systemd/ipp-proxy.service /etc/systemd/system/ && \
	sudo systemctl daemon-reload && \
	sudo systemctl enable ipp-proxy && \
	sudo systemctl restart ipp-proxy

disable_cups_631:
	sudo sed -i 's/Listen localhost:631/Listen localhost:6310/' /etc/cups/cupsd.conf && \
	sudo sudo systemctl restart cups

remoteBuild: upload
	ssh $(shell "cat" "$(REMOTE_FILE)") "cd $(REMOTE_PATH) && make install"

upload:
	rsync -avr --delete --exclude target . $(shell "cat" "$(REMOTE_FILE)"):$(REMOTE_PATH)

build:
	cargo build --color always

run:
	sudo ./target/debug/ipp-proxy