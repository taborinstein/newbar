#include <stdio.h>
#include <sys/statvfs.h>

struct Df {
    int total;
    int used;
};

struct Df df_root() {
    struct statvfs buffer;
    statvfs("/", &buffer);
    int total = (buffer.f_blocks * buffer.f_frsize) / 1024;
    int avail = (buffer.f_bfree * buffer.f_frsize) / 1024;
    struct Df df = { total, total - avail};
    return df;
}