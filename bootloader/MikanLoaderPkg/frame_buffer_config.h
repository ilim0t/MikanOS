#pragma once

#include <stdint.h>

typedef enum {
  kPixelRGBResv8BitPerColor,
  kPixelBGRResv8BitPerColor,
} PixelFormat;

typedef struct {
  uint8_t* frame_buffer;
  uint32_t pixels_per_scan_line;
  uint32_t horizontal_resolution;
  uint32_t vertical_resolution;
  PixelFormat pixel_format;
} FrameBufferConfig;
