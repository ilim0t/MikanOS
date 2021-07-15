// #include <cstdint>
#include <cstdio>
#include <new>

#include "console.hpp"
#include "font.hpp"
#include "frame_buffer_config.hpp"
#include "graphics.hpp"

void operator delete(void* obj) noexcept {}

alignas(Console) char console_buf[sizeof(Console)];  // charである必要はなく1byteの型ならなんでも良い
Console* console;

int printk(const char* format, ...) {
  va_list ap;
  int result;
  char s[1024];

  va_start(ap, format);
  result = vsprintf(s, format, ap);
  va_end(ap);

  console->PutString(s);
  return result;
}
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
      pixel_writer->Write(x, y, {0, 255, 0});
    }
  }

  char buf[128];

  console = new (console_buf) Console(*pixel_writer, {0, 0, 0}, {255, 255, 255});
  for (int i = 0; i < 30; i++) {
    sprintf(buf, "line: %d\n", i);
    console->PutString(buf);
  }

  while (1) {
    __asm__("hlt");
  }
}
