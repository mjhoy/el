#include <stdlib.h>
#include "dbg.h"
#include "map.h"

// private
int map_initial_pos(Map *map);

Map*
map_create(const char *data, int width, int height) {
      Map *map = malloc(sizeof(Map));

      char *m_data = malloc((sizeof(char) * width * height) + 1);

      strlcpy(m_data, data, (width * height) + 1);

      map->data = m_data;
      map->w = width;
      map->h = height;      

      map->pos = map_initial_pos(map);

      if(map->pos == -1) {
            die("couldn't find position for map");
      }
      
      return map;
}

void
map_destroy(Map *map) {
      free(map);
}

int
map_move(Map *map, int ch) {
      int new_pos = map->pos;
      switch (ch) {
      case 'h':
            new_pos = map->pos - 1;
            break;
      case 'l':
            new_pos = map->pos + 1;
            break;
      case 'j':
            new_pos = map->pos + map->w;
            break;
      case 'k':
            new_pos = map->pos - map->w;
            break;
      }

      if (map->data[new_pos] == '#') {
            return 0;
      }
      map->pos = new_pos;
      return 1;      
}

/* Return the initial position of the character for `map`.
 *
 * Returns -1 if L is not found.
 */
int
map_initial_pos(Map *map) {
      int i, j;
      int pos = -1;
      char cur;
      for (i = 0; i < map->h; i++) {
            for (j = 0; j < map->w; j++) {
                  cur = map->data[(map->w*i)+j];
                  switch (cur) {
                  case 'L':
                        pos = (map->w*i)+j;
                        break;
                  }
            }
      }
      return pos;      
}
