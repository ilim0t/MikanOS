#include "graphics.hpp"

uint8_t* PixelWriter::PixelAt(const int x, const int y) {
  return config_.frame_buffer +
         4 * (config_.pixels_per_scan_line * y + x);  // 3byteに使わない予約領域を加えて切りの良い4byteにしている
}

void RGBResv8BitPerColorPixelWriter::Write(const int x, const int y, const PixelColor& c) {
  const auto p = PixelAt(x, y);
  p[0] = c.r;
  p[1] = c.g;
  p[2] = c.b;
}

void BGRResv8BitPerColorPixelWriter::Write(const int x, const int y, const PixelColor& c) {
  const auto p = PixelAt(x, y);
  p[0] = c.b;
  p[1] = c.g;
  p[2] = c.r;
}
