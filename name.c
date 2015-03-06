#include "name.h"
#include <limits.h>

static char name[NAME_MAX];

char *
getName()
{
    return name;
}

void
setName(char *newName)
{
    int i;
    for (i = 0; i < NAME_MAX && newName[i] != '\0'; i++)
        name[i] = newName[i];
    name[i+1] = '\0';
}
