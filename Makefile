.PHONY: pcre2 clib clean

all: pcre2 clib

pcre2:
	cd pcre2/ && ./autogen.sh
	cd pcre2/ && ./configure --prefix $(CURDIR)/target/usr/local && make && make install

clib: target/libsearch.a 

target/search.o: src/search.c
	cc -Wall -c $^ -I$(CURDIR)/target/usr/local/include \
	-L$(CURDIR)/target/usr/local/lib \
	-lpcre2-8.a -o $@
target/libsearch.a: target/search.o
	ar rcs $@ $^

clean:
	@rm target/*search*
	cd pcre2 && make clean