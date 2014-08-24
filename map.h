#ifndef __map_h__
#define __map_h__

typedef struct Map {
      const char *data;
      int w;
      int h;
      int pos;
} Map;

Map* map_create(const char *data, int width, int height);
void map_destroy(Map *map);
int map_move(Map *map, int ch);

#endif
