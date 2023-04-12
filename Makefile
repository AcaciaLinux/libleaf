all:
	cargo build --release

clean:
	cargo clean

install: all
	mkdir -pv $(DESTDIR)/usr/lib/
	cp -v target/release/libleaf.so $(DESTDIR)/usr/lib/
