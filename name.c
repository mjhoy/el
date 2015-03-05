#include "name.h"

#define MAX_NAME_LEN 15

static char name[MAX_NAME_LEN];

char *
getName()
{
    return name;
}

void
setName(char *newName)
{
    int i;
    for (i = 0; i < MAX_NAME_LEN && newName[i] != '\0'; i++)
        name[i] = newName[i];
    name[i+1] = '\0';
}
