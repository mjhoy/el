#include <stdlib.h>
#include "dbg.h"
#include "map.h"
#include "log.h"

static void map_update_visibility(Map *map);

// private
int map_initial_pos(Map *map);

// do nuthin event
int noopEvent(void *_map, int event_pos) {
    Map *map = (Map *)_map;
    FILE *logfile = ellogfile();
    fprintf(logfile, "NOOP called: map %s, pos: %i\n", map->id, event_pos);
    return 1;
}

Map*
map_create(const char *data, const char *id, int width, int height) {
    int i;

    FILE *logfile = ellogfile();
      
    Map *map = malloc(sizeof(Map));

    map->id = id;

    char *m_data = malloc((sizeof(char) * width * height) + 1);
    char *m_visible = malloc((sizeof(char) * width * height) + 1);

    strlcpy(m_data, data, (width * height) + 1);
    memset(m_visible, ' ', width * height);
    m_visible[width * height] = '\0';

    map->data = m_data;
    map->visible = m_visible;
    map->w = width;
    map->h = height;

    map->pos = map_initial_pos(map);

    map_update_visibility(map);

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

#define MAP_CAN_SEE_THROUGH(map, pos) (map->data[pos] != '#' && \
                                       map->data[pos] != '|')

#define CHECK_AND_MARK_NEIGHBOR(map, pos, flag)   \
    if (pos > 0 && pos < (map->w * map->h) &&     \
        (map->visible[pos] == ' ')) {             \
      map->visible[pos] = 'X';                    \
      flag = 1;                                   \
    }                                             \

void
map_update_visibility(Map *map) {
    map->visible[map->pos] = 'X';
    int set_new = 1;

    int i,j, w,h, cur_coord, neighbor;

    w = map->w;
    h = map->h;

    while (set_new) {
        set_new = 0;
        for (i = 0; i < h; i++) {
            for (j = 0; j < w; j++) {
                cur_coord = (w*i) + j;
                if (map->visible[cur_coord] == 'X' &&
                    MAP_CAN_SEE_THROUGH(map, cur_coord)) {

                    // North
                    neighbor = cur_coord - w;
                    CHECK_AND_MARK_NEIGHBOR(map, neighbor, set_new);

                    // south
                    neighbor = cur_coord + w;
                    CHECK_AND_MARK_NEIGHBOR(map, neighbor, set_new);

                    // east
                    if (j != w) {
                        neighbor = cur_coord + 1;
                        CHECK_AND_MARK_NEIGHBOR(map, neighbor, set_new);
                    }

                    // west
                    if (j != 0) {
                        neighbor = cur_coord - 1;
                        CHECK_AND_MARK_NEIGHBOR(map, neighbor, set_new);
                    }
                }
            }
        }
    }
}

void
map_destroy(Map *map) {      
    if(map->data) free(map->data);
    if(map->visible) free(map->visible);
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
        (*map->events[temp_pos - '0'])(map, new_pos);
        map_update_visibility(map);
        break;
    default:
        map->pos = new_pos;
        map_update_visibility(map);
        return 1;
    }
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
