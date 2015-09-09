/*
   
 *  el  *

 */

#include <ncurses.h>
#include <locale.h>
#include <string.h>
#include <stdio.h>
#include <pwd.h>
#include <unistd.h>
#include <time.h>

#include "map.h"
#include "dbg.h"
#include "log.h"
#include "ui.h"
#include "level1.h"
#include "name.h"

void
tellmeaboutthedate()
{
    int row, col;

    time_t t;
    struct tm loc;
    struct tm *locp;

    t = time(NULL);

    locp = localtime(&t);
    loc = *locp;

    getmaxyx(stdscr, row, col);
    clear();

    mvprintw(
        row-3,
        0,
        "just so you know it has been %d days since the year began.", loc.tm_yday);
    mvprintw(
        row-2,
        0,
        "(hit a key to continue)");
    refresh();
    getch();
}

void
setupname()
{
    char name[30];
    char default_name[30];

    uid_t euid;
    struct passwd *pwd;

    int row, col;

    getmaxyx(stdscr, row, col);

    /* set the default name as currect effective user name */
    euid = geteuid();         /* effective user id */
    pwd  = getpwuid(euid);    /* passwd struct */
    strncpy(default_name, pwd->pw_name, 29);
    default_name[29] = '\0';

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

void
intro() {
    char *title = "L";

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

    setupname();
    tellmeaboutthedate();
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
