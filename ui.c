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

/* TODO move everything that doesn't have to do with calling printw()
 * into map.c */
void
render_map(Map *map) {
    int maxx, maxy;
    int i, j;
    int cur_coord;
    char cur;
    clear();
    for (i = 0; i < map->h; i++) {
        for(j = 0; j < map->w; j++) {
            cur_coord = (map->w*i) + j;
            if (cur_coord == map->pos) {
                printw("L");
            } else {
                cur = map->data[cur_coord];
                switch (cur) {
                case '0':
                case '1':
                case '2':
                case '3':
                case '4': //events
                    printw(" ");
                    break;
                case 'L':
                    printw(" ");
                    break;
                default:
                    if (map->visible[cur_coord] == 'X')
                        printw("%c", cur);
                    else
                        printw(" ");
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
