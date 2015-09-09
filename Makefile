CFLAGS=-g -Wall -Wextra
LDFLAGS=-lncurses
OBJECTS=main.o map.o log.o ui.o level1.o name.o

el: $(OBJECTS)
	$(CC) $(CFLAGS) $(OBJECTS) -o el $(LDFLAGS)

all: el

.PHONY: clean
clean:
	rm -f *.o
	rm -f el
