#include <cstdint>
#include <new>

#include "frame_buffer_config.hpp"

struct PixelColor {
  uint8_t r, g, b;
};

class PixelWriter {
 public:
  PixelWriter(const FrameBufferConfig& config) : config_{config} {}
  virtual ~PixelWriter() = default;
  virtual void Write(int x, int y, const PixelColor& c) = 0;

 protected:
  uint8_t* PixelAt(int x, int y) { return config_.frame_buffer + 4 * (config_.pixels_per_scan_line * y + x); }

 private:
  const FrameBufferConfig& config_;
};

class RGBResv8BitPerColorPixelWriter : public PixelWriter {
 public:
  using PixelWriter::PixelWriter;

  virtual void Write(int x, int y, const PixelColor& c) override {
    auto p = PixelAt(x, y);
    p[0] = c.r;
    p[1] = c.g;
    p[2] = c.b;
  }
};

class BGRResv8BitPerColorPixelWriter : public PixelWriter {
 public:
  using PixelWriter::PixelWriter;

  virtual void Write(int x, int y, const PixelColor& c) override {
    auto p = PixelAt(x, y);
    p[0] = c.b;
    p[1] = c.g;
    p[2] = c.r;
  }
};

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
  // delete pixel_writer;
  pixel_writer->~PixelWriter();

  while (1) {
    __asm__("hlt");
  }
}
