#include <ncurses.h>
#include <unistd.h>

#include "level1.h"
#include "map.h"
#include "ui.h"

const char *level_1_map =
    "                  "
    "   #########      "
    "   #       ###### "
    "   #      0|    # "
    "   #       ###  # "
    "   # L     # #  # "
    "   ######### #--# "
    "             #### ";

static Map *map;

static void finish_level_1();

int event0(void *_map, int event_pos) {
    Map *map = (Map *)_map;
    clear();
    printw("Let's open the door!");
    refresh();
    sleep(1);
    printw("\nPress ANY key to continue.");
    getch();

    map->pos = event_pos;
    map->data[map->pos + 1] = ' ';
    map->data[map->pos] = ' ';
    return 0;
}

void run_level_1() {
    int ch, keep_playing = 1;
      
    map = map_create(level_1_map, "level 1 map", 18, 8);

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
