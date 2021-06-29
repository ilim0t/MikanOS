// #include <cstdint>
#include <new>

#include "font.hpp"
#include "frame_buffer_config.hpp"
#include "graphics.hpp"

void operator delete(void* obj) noexcept {}

extern "C" void KernelMain(const FrameBufferConfig& frame_buffer_config) {
  alignas(RGBResv8BitPerColorPixelWriter) char
      pixel_writer_buf[sizeof(RGBResv8BitPerColorPixelWriter)];  // charである必要はなく1byteの型ならなんでも良い
  PixelWriter* pixel_writer;

  if (frame_buffer_config.pixel_format == PixelFormat::kPixelRGBResv8BitPerColor) {
    pixel_writer = new (pixel_writer_buf) RGBResv8BitPerColorPixelWriter{frame_buffer_config};
  } else if (frame_buffer_config.pixel_format == PixelFormat::kPixelBGRResv8BitPerColor) {
    pixel_writer = new (pixel_writer_buf) BGRResv8BitPerColorPixelWriter(frame_buffer_config);
  } else {
  }

  for (uint64_t x = 0; x < frame_buffer_config.horizontal_resolution; ++x) {
    for (uint64_t y = 0; y < frame_buffer_config.vertical_resolution; ++y) {
      pixel_writer->Write(x, y, {255, 255, 255});
    }
  }

  for (int x = 0; x < 200; ++x) {
    for (int y = 0; y < 100; ++y) {
      pixel_writer->Write(100 + x, 100 + y, {0, 255, 0});
    }
  }

  int i = 0;
  for (char c = '!'; c <= '~'; ++c, ++i) {
    WriteAscii(*pixel_writer, 8 * i, 50, c, {0, 0, 0});
  }

  while (1) {
    __asm__("hlt");
  }
}
