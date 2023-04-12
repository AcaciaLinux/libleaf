all:
	cargo build --release

clean:
	cargo clean

install: all
	mkdir -pv $(DESTDIR)/usr/lib/
	mkdir -pv $(DESTDIR)/usr/include/
	cp -v target/release/libleaf.so $(DESTDIR)/usr/lib/
	cp -v leaf.h $(DESTDIR)/usr/include/
