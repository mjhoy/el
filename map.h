#ifndef __map_h__
#define __map_h__

typedef int (*eventFn)(void *map, int event_pos);

typedef struct Map {
    char *data;
    char *visible;
    const char *id;
    int w;
    int h;
    int pos;
    eventFn events[5];
} Map;

Map* map_create(const char *data, const char *id, int width, int height);
void map_destroy(Map *map);
int map_move(Map *map, int ch);

#endif
