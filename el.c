/*
   
 *  el  *

*/

#include <ncurses.h>
#include <locale.h>
#include <string.h>

void intro() {
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


int main() {
      setlocale(LC_ALL, "en_US.UTF-8");
      initscr();

      intro();

      endwin();
      return 0;
}
