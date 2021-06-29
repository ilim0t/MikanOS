#include "font.hpp"

extern const uint8_t _binary_hankaku_bin_start;
extern const uint8_t _binary_hankaku_bin_end;
extern const uint8_t _binary_hankaku_bin_size;

const uint8_t* GetFont(const char c) {
  const auto index = 16 * static_cast<unsigned int>(c);
  if (index >= reinterpret_cast<uintptr_t>(&_binary_hankaku_bin_size)) {
    return nullptr;
  }
  return &_binary_hankaku_bin_start + index;
}

void WriteAscii(PixelWriter& writer, const int x, const int y, const char c, const PixelColor& color) {
  const uint8_t* font = GetFont(c);
  if (font == nullptr) {
    return;
  }
  for (int dy = 0; dy < 16; dy++) {
    for (int dx = 0; dx < 8; dx++) {
      if (font[dy] & ((0x1 << (7 - dx)))) {
        writer.Write(x + dx, y + dy, color);
      }
    }
  }
}

void WriteString(PixelWriter& writer, const int x, const int y, const char* s, const PixelColor& color) {
  for (int i = 0; s[i] != '\0'; i++) {
    WriteAscii(writer, x + 8 * i, y, s[i], color);
  }
}
