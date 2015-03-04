#include <ncurses.h>
#include <unistd.h>

#include "level1.h"
#include "map.h"
#include "ui.h"

const char *level_1_map =
      "                "
      "   #########    "
      "   #       #    "
      "   #      0|    "
      "   #       #    "
      "   # L     #    "
      "   #########    "
      "                ";

static Map *map;

static void finish_level_1();

int event0(void *map) {
      clear();
      printw("Oh no! The door is locked.");
      refresh();
      sleep(1);
      printw("\nPress ANY key to continue.");
      getch();
      return 0;
}

void run_level_1() {
      int ch, keep_playing = 1;
      
      map = map_create(level_1_map, 16, 8);

      // set events
      map->events[0] = &event0;

      render_map(map);

      while(keep_playing == 1) {
            ch = getch();

            switch (ch) {
            case 'q':
            case 'Q':
            case 127: // DEL
                  keep_playing = 0;
                  break;
            default:
                  map_move(map, ch);
                  render_map(map);
                  break;
            }
      }

      finish_level_1();
}

static void finish_level_1() {
      map_destroy(map);
}
