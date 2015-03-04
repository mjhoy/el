#include <stdlib.h>
#include "dbg.h"
#include "map.h"
#include "log.h"

// private
int map_initial_pos(Map *map);

// do nuthin event
int noopEvent(void *_map) {
      FILE *logfile = ellogfile();
      fprintf(logfile, "NOOP called\n");
      return 1;
}

Map*
map_create(const char *data, int width, int height) {
      int i;

      FILE *logfile = ellogfile();
      
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

      for (i = 0; i < 5; i++) {
            fprintf(logfile, "Event number %d\n", i);
            fflush(logfile);
            map->events[i] = &noopEvent;
      }
      
      return map;
}

void
map_destroy(Map *map) {      
      if(map->data) free(map->data);
      free(map);
}

int
map_move(Map *map, int ch) {
      FILE *logfile = ellogfile();
      
      int new_pos = map->pos;
      int temp_pos;
      
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

      switch (temp_pos = map->data[new_pos]) {
      case '#':
      case '|':
            return 0;
      case '0':
      case '1':
      case '2':
      case '3':
      case '4': /* events */
            /* TODO: convert character into integer */
            fprintf(logfile, "%d\n",temp_pos - '0');
            fprintf(logfile, "%c\n",temp_pos);
            fflush(logfile);
            (*map->events[temp_pos - '0'])(map);
      default:
            map->pos = new_pos;
            return 1;
      }
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
