/* confdefs.h */
#define PACKAGE_NAME "lame"
#define PACKAGE_TARNAME "lame"
#define PACKAGE_VERSION "3.100"
#define PACKAGE_STRING "lame 3.100"
#define PACKAGE_BUGREPORT "lame-dev@lists.sf.net"
#define PACKAGE_URL ""
#define PACKAGE "lame"
#define VERSION "3.100"
/* end confdefs.h.  */
#include <ctype.h>
#include <stdlib.h>
#if ((' ' & 0x0FF) == 0x020)
# define ISLOWER(c) ('a' <= (c) && (c) <= 'z')
# define TOUPPER(c) (ISLOWER(c) ? 'A' + ((c) - 'a') : (c))
#else
# define ISLOWER(c) 		   (('a' <= (c) && (c) <= 'i') 		     || ('j' <= (c) && (c) <= 'r') 		     || ('s' <= (c) && (c) <= 'z'))
# define TOUPPER(c) (ISLOWER(c) ? ((c) | 0x40) : (c))
#endif

#define XOR(e, f) (((e) && !(f)) || (!(e) && (f)))
int
main ()
{
  int i;
  for (i = 0; i < 256; i++)
    if (XOR (islower (i), ISLOWER (i))
	|| toupper (i) != TOUPPER (i))
      return 2;
  return 0;
}
