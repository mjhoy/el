CFLAGS=-g -Wall
LDFLAGS=-lncurses
OBJECTS=el.o map.o log.o ui.o level1.o

el: $(OBJECTS)
	$(CC) $(CFLAGS) $(OBJECTS) -o el $(LDFLAGS)

all: el

.PHONY: clean
clean:
	rm -f *.o
	rm -f el
