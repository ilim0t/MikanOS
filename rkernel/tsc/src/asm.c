#include <stdint.h>

uint64_t rdtsc()
{
  uint32_t tickl, tickh;
  asm ("rdtsc" : "=a"(tickl), "=d"(tickh));
  return (((uint64_t) tickh) << 32) | (uint64_t)tickl;
}
