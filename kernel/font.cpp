#include "font.hpp"

void WriteAscii(PixelWriter& writer, const int x, const int y, const char c, const PixelColor& color) {
  if (c != 'A') {
    return;
  }
  for (int dy = 0; dy < 16; dy++) {
    for (int dx = 0; dx < 8; dx++) {
      if (kFontA[dy] & ((0x1 << (7 - dx)))) {
        writer.Write(x + dx, y + dy, color);
      }
    }
  }
}