/*
 * Copyright 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <cassert>
#include <cstdint>
#include <memory>
#include <utility>
#include <vector>

namespace pdl::packet {

/// Representation of raw packet data.
/// The slice keeps an owning reference to the source data.
class slice {
 public:
  slice(std::shared_ptr<const std::vector<uint8_t>> packet)
      : packet_(std::move(packet)), offset_(0), size_(packet_->size()) {}
  slice(std::shared_ptr<const std::vector<uint8_t>> packet, size_t offset,
        size_t size)
      : packet_(std::move(packet)), offset_(offset), size_(size) {
    assert(offset_ < packet_->size());
    assert(size_ <= packet_->size());
    assert(offset_ + size_ <= packet_->size());
  }

 private:
  std::shared_ptr<const std::vector<uint8_t>> packet_;
  size_t offset_{0};
  size_t size_{0};

  friend class weak_slice;
};

/// Representation of raw packet data.
/// The weak slice only keeps a non owning reference to the source data
/// and cannot outlive its input data.
class weak_slice {
 public:
  weak_slice() = default;
  // The move operations are declared explicitly: the declaration of the
  // copy constructor would otherwise suppress the implicit move constructor,
  // and moving a slice would copy the shared pointer to the packet bytes.
  weak_slice(uint8_t const* data, size_t offset, size_t size)
      : data_(data), offset_(offset), size_(size) {}
  weak_slice(slice const& slice)
      : weak_slice(slice.packet_->data(), slice.offset_, slice.size_) {}
  weak_slice(weak_slice const&) = default;
  weak_slice& operator=(weak_slice const&) = default;

  using const_iterator = uint8_t const*;

  const_iterator cbegin() const noexcept { return data_ + offset_; }
  const_iterator cend() const noexcept { return data_ + offset_ + size_; }

  /// Return a new slice that contains the selected subrange within the
  /// current slice. The range ['offset', 'offset' + 'slice') must be
  /// contained within the bonuds of the current slice.
  weak_slice subrange(size_t offset, size_t size) const {
    assert((offset + size) <= size_);
    return weak_slice(data_, offset_ + offset, size);
  }

  /// Return a new slice that contains the first N bytes of the slice.
  weak_slice take(size_t size) {
    assert(size <= size_);
    weak_slice out(data_, offset_, size);
    offset_ += size;
    size_ -= size;
    return out;
  }

  /// Read a scalar value encoded in little-endian.
  /// The bytes that are read from calling this function are consumed.
  /// This function can be used to iterativaly extract values from a packet
  /// slice.
  template <typename T, size_t N = sizeof(T)>
  T read_le() {
    static_assert(N <= sizeof(T));
    assert(N <= size_);
    T value = 0;
    auto const* ptr = data_ + offset_;
    for (size_t n = 0; n < N; n++) {
      value |= (T)ptr[n] << (8 * n);
    }
    offset_ += N;
    size_ -= N;
    return value;
  }

  /// Read a scalar value encoded in big-endian.
  /// The bytes that are read from calling this function are consumed.
  /// This function can be used to iterativaly extract values from a packet
  /// slice.
  template <typename T, size_t N = sizeof(T)>
  T read_be() {
    static_assert(N <= sizeof(T));
    assert(N <= size_);
    T value = 0;
    auto const* ptr = data_ + offset_;
    for (size_t n = 0; n < N; n++) {
      value = (value << 8) | (T)ptr[n];
    }
    offset_ += N;
    size_ -= N;
    return value;
  }

  /// Skip `size` bytes at the front of the slice.
  /// `size` must be lower than or equal to the slice size.
  void skip(size_t size) {
    assert(size <= size_);
    offset_ += size;
    size_ -= size;
  }

  /// Empty the slice.
  void clear() { size_ = 0; }

  /// Return the size of the slice in bytes.
  size_t size() const { return size_; }

  /// Return the contents of the slice as a byte vector.
  std::vector<uint8_t> bytes() const {
    return std::vector<uint8_t>(cbegin(), cend());
  }

  bool operator==(weak_slice const& other) const {
    return size_ == other.size_ && std::equal(cbegin(), cend(), other.cbegin());
  }

 private:
  uint8_t const* data_{nullptr};
  size_t offset_{0};
  size_t size_{0};
};

/// Interface class for generated packet builders.
class Builder {
 public:
  virtual ~Builder() = default;

  /// Method implemented by generated packet builders.
  /// The packet fields are concatenated to the output vector.
  virtual bool Serialize(std::vector<uint8_t>&) const { return true; }

  /// Method implemented by generated packet builders.
  /// Returns the size of the serialized packet in bytes.
  virtual size_t GetSize() const { return 0; }

  /// Write a scalar value encoded in little-endian.
  template <typename T, size_t N = sizeof(T)>
  static void write_le(std::vector<uint8_t>& output, T value) {
    static_assert(N <= sizeof(T));
    for (size_t n = 0; n < N; n++) {
      output.push_back(value >> (8 * n));
    }
  }

  /// Write a scalar value encoded in big-endian.
  template <typename T, size_t N = sizeof(T)>
  static void write_be(std::vector<uint8_t>& output, T value) {
    static_assert(N <= sizeof(T));
    for (size_t n = 0; n < N; n++) {
      output.push_back(value >> (8 * (N - 1 - n)));
    }
  }

  /// Helper method to serialize the packet to a byte vector.
  virtual std::vector<uint8_t> SerializeToBytes() const {
    std::vector<uint8_t> output;
    Serialize(output);
    return output;
  }
};

}  // namespace pdl::packet
