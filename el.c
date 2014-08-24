/*
   
 *  el  *

*/

#include <ncurses.h>
#include <locale.h>
#include <string.h>

#include "map.h"
#include "dbg.h"

const char *main_map =
      "                "
      "   #########    "
      "   #       #    "
      "   #            "
      "   #       #    "
      "   # L     #    "
      "   #########    "
      "                ";

void
render_map(Map *map) {
      int i, j;
      char cur;
      clear();
      for (i = 0; i < map->h; i++) {
            for(j = 0; j < map->w; j++) {
                  if (((map->w*i)+j) == map->pos) {
                        printw("L");
                  } else {
                        cur = map->data[(map->w*i)+j];
                        switch (cur) {
                        case '#':
                              printw("%c", cur);
                              break;
                        case 'L':
                              printw(" ");
                              break;
                        default:
                              printw("%c", cur);
                              break;
                        }
                  }
            }
            printw("\n");
      }
      mvprintw(
            map->h + 3,
            0,
            "h l ");
      refresh();
}

void
intro() {
      char *title = "🐋   L";

      int row, col;
      
      getmaxyx(stdscr, row, col);

      attron(A_BOLD);
      mvprintw(
            (row / 2) - 1,
            (col - strlen(title)) / 2,
            "%s", title);
      attroff(A_BOLD);

      mvprintw(
            row-1,
            0,
            "hit a key to continue ");

      refresh();
      getch();
}

int
main() {
      int ch, keep_playing = 1;
      
      setlocale(LC_ALL, "en_US.UTF-8");
      initscr();

      intro();

      Map *map = map_create(main_map, 16, 8);
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
      map_destroy(map);
      endwin();

      return 0;
}
