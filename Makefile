pastebin:
	go build -o bin/pastebin cmd/pastebin/*.go

userKey:
	go build -o bin/pastebin-ukey cmd/getUserKey/*.go

clean:
	rm -rf bin/*
