#include "log.h"

static FILE *logfile;

void
init_log() {
      logfile = fopen("log", "a");
}

void
end_log() {
      fclose(logfile);
}

FILE *
ellogfile() {
      return logfile;
}
