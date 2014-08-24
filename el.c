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

void print_command_line(int row);

void
render_map(Map *map) {
      int maxx, maxy;
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
      getmaxyx(stdscr, maxy, maxx);
      print_command_line(maxy - 2);
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

void
print_command_line(int row) {
      mvprintw(row,0, "------------------------------");
      row = row + 1;

      mvprintw(row, 0, "movement ");
      attron(A_BOLD);
      mvprintw(row,9,"h j k l"); attroff(A_BOLD);

      mvprintw(row, 16, " help ");
      attron(A_BOLD);
      mvprintw(row,22,"?"); attroff(A_BOLD);

      mvprintw(row, 23, " quit ");
      attron(A_BOLD);
      mvprintw(row,29,"q"); attroff(A_BOLD);
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
