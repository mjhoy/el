/*
   
 *  el  *

*/

#include <ncurses.h>
#include <locale.h>
#include <string.h>
#include <stdio.h>

#include "map.h"
#include "dbg.h"
#include "log.h"
#include "ui.h"
#include "level1.h"

const char *main_map =
      "                "
      "   #########    "
      "   #       #    "
      "   #      0|    "
      "   #       #    "
      "   # L     #    "
      "   #########    "
      "                ";

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
      init_log();
      setlocale(LC_ALL, "en_US.UTF-8");
      initscr();

      intro();

      run_level_1();

      endwin();
      end_log();
      return 0;
}
