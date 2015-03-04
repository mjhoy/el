#include "ui.h"

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
                        case '0':
                        case '1':
                        case '2':
                        case '3':
                        case '4': //events
                              printw(" ");
                              break;
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
