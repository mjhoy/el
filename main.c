/*
   
 *  el  *

*/

#include <ncurses.h>
#include <locale.h>
#include <string.h>
#include <stdio.h>
#include <pwd.h>
#include <unistd.h>

#include "map.h"
#include "dbg.h"
#include "log.h"
#include "ui.h"
#include "level1.h"
#include "name.h"

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

      char name[30];
      char default_name[30];

      uid_t euid;
      struct passwd *pwd;

      /* set the default name as currect effective user name */
      euid = geteuid();         /* effective user id */
      pwd  = getpwuid(euid);    /* passwd struct */
      strncpy(default_name, pwd->pw_name, 29);
      default_name[29] = '\0';
      
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

      clear();
      mvprintw(
            row-1,
            0,
            "what's your name? [%s] ", default_name);

      getnstr(name, 30);
      if(strlen(name) == 0) {
          setName(default_name);
      } else {
          setName(name);
      }
      clear();
      mvprintw(
          row-1,
          0,
          "hello, %s! (hit a key to continue)",
          getName());
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
