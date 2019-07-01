#ifndef READALL_H

#include <stdio.h>

/* Size of each input chunk to be read and allocate for. */
#define READALL_CHUNK 262144

#define READALL_OK 0       /* Success */
#define READALL_INVALID -1 /* Invalid parameters */
#define READALL_ERROR -2   /* Stream error */
#define READALL_TOOMUCH -3 /* Too much input */
#define READALL_NOMEM -4   /* Out of memory */

int readall(FILE *in, char **dataptr, size_t *sizeptr);

#endif