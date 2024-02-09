all: pastebin userKey

pastebin:
	go build -o bin/pastebin cmd/pastebin/*.go

pastebin-ukey:
	go build -o bin/pastebin-ukey cmd/pastebin-ukey/*.go

install:
	go install ./cmd/pastebin
	go install ./cmd/pastebin-ukey

clean:
	rm -rf bin/*
