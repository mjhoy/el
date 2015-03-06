#include "name.h"
#include <limits.h>

/* LOGIN_NAME_MAX not defined on OSX... hmm... */
#ifndef LOGIN_NAME_MAX
#define LOGIN_NAME_MAX 255
#endif

static char name[LOGIN_NAME_MAX];

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
