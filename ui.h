#ifndef __UI_H
#define __UI_H

#include <ncurses.h>
#include "map.h"

void print_command_line(int row);
void render_map(Map *map);

#endif
