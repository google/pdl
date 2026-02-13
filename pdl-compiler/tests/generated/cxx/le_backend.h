// File generated from tests/canonical/le_test_file.pdl, with the command:
//  pdlc ...
// /!\ Do not edit by hand

#pragma once

#include <cstdint>
#include <string>
#include <optional>
#include <utility>
#include <vector>
#include <array>
#include <numeric>

#include <packet_runtime.h>

#ifndef _ASSERT_VALID
#ifdef ASSERT
#define _ASSERT_VALID ASSERT
#else
#include <cassert>
#define _ASSERT_VALID assert
#endif  // ASSERT
#endif  // !_ASSERT_VALID

namespace le_backend {
class ScalarParentView;
class EnumParentView;
class EmptyParentView;
class Packet_Scalar_FieldView;
class Packet_Enum_FieldView;
class Packet_Reserved_FieldView;
class Packet_Size_FieldView;
class Packet_Count_FieldView;
class Packet_FixedScalar_FieldView;
class Packet_FixedEnum_FieldView;
class Packet_Payload_Field_VariableSizeView;
class Packet_Payload_Field_SizeModifierView;
class Packet_Payload_Field_UnknownSizeView;
class Packet_Payload_Field_UnknownSize_TerminalView;
class Packet_Body_Field_VariableSizeView;
class Packet_Body_Field_UnknownSizeView;
class Packet_Body_Field_UnknownSize_TerminalView;
class Packet_ScalarGroup_FieldView;
class Packet_EnumGroup_FieldView;
class Packet_Struct_FieldView;
class Packet_Array_Field_ByteElement_ConstantSizeView;
class Packet_Array_Field_ByteElement_VariableSizeView;
class Packet_Array_Field_ByteElement_VariableCountView;
class Packet_Array_Field_ByteElement_UnknownSizeView;
class Packet_Array_Field_ScalarElement_ConstantSizeView;
class Packet_Array_Field_ScalarElement_VariableSizeView;
class Packet_Array_Field_ScalarElement_VariableCountView;
class Packet_Array_Field_ScalarElement_UnknownSizeView;
class Packet_Array_Field_EnumElement_ConstantSizeView;
class Packet_Array_Field_EnumElement_VariableSizeView;
class Packet_Array_Field_EnumElement_VariableCountView;
class Packet_Array_Field_EnumElement_UnknownSizeView;
class Packet_Array_Field_SizedElement_ConstantSizeView;
class Packet_Array_Field_SizedElement_VariableSizeView;
class Packet_Array_Field_SizedElement_VariableCountView;
class Packet_Array_Field_SizedElement_UnknownSizeView;
class Packet_Array_Field_UnsizedElement_ConstantSizeView;
class Packet_Array_Field_UnsizedElement_VariableSizeView;
class Packet_Array_Field_UnsizedElement_VariableCountView;
class Packet_Array_Field_UnsizedElement_UnknownSizeView;
class Packet_Array_Field_UnsizedElement_SizeModifierView;
class Packet_Array_Field_SizedElement_VariableSize_PaddedView;
class Packet_Array_Field_UnsizedElement_VariableCount_PaddedView;
class Packet_Optional_Scalar_FieldView;
class Packet_Optional_Enum_FieldView;
class Packet_Optional_Struct_FieldView;
class ScalarChild_AView;
class ScalarChild_BView;
class EnumChild_AView;
class EnumChild_BView;
class AliasedChild_AView;
class AliasedChild_BView;
class Struct_Enum_FieldView;
class Struct_Reserved_FieldView;
class Struct_Size_FieldView;
class Struct_Count_FieldView;
class Struct_FixedScalar_FieldView;
class Struct_FixedEnum_FieldView;
class Struct_ScalarGroup_FieldView;
class Struct_EnumGroup_FieldView;
class Struct_Struct_FieldView;
class Struct_Array_Field_ByteElement_ConstantSizeView;
class Struct_Array_Field_ByteElement_VariableSizeView;
class Struct_Array_Field_ByteElement_VariableCountView;
class Struct_Array_Field_ByteElement_UnknownSizeView;
class Struct_Array_Field_ScalarElement_ConstantSizeView;
class Struct_Array_Field_ScalarElement_VariableSizeView;
class Struct_Array_Field_ScalarElement_VariableCountView;
class Struct_Array_Field_ScalarElement_UnknownSizeView;
class Struct_Array_Field_EnumElement_ConstantSizeView;
class Struct_Array_Field_EnumElement_VariableSizeView;
class Struct_Array_Field_EnumElement_VariableCountView;
class Struct_Array_Field_EnumElement_UnknownSizeView;
class Struct_Array_Field_SizedElement_ConstantSizeView;
class Struct_Array_Field_SizedElement_VariableSizeView;
class Struct_Array_Field_SizedElement_VariableCountView;
class Struct_Array_Field_SizedElement_UnknownSizeView;
class Struct_Array_Field_UnsizedElement_ConstantSizeView;
class Struct_Array_Field_UnsizedElement_VariableSizeView;
class Struct_Array_Field_UnsizedElement_VariableCountView;
class Struct_Array_Field_UnsizedElement_UnknownSizeView;
class Struct_Array_Field_UnsizedElement_SizeModifierView;
class Struct_Array_Field_SizedElement_VariableSize_PaddedView;
class Struct_Array_Field_UnsizedElement_VariableCount_PaddedView;
class Struct_Optional_Scalar_FieldView;
class Struct_Optional_Enum_FieldView;
class Struct_Optional_Struct_FieldView;
class Enum_Incomplete_Truncated_ClosedView;
class Enum_Incomplete_Truncated_OpenView;
class Enum_Incomplete_Truncated_Closed_WithRangeView;
class Enum_Incomplete_Truncated_Open_WithRangeView;
class Enum_Complete_TruncatedView;
class Enum_Complete_Truncated_WithRangeView;
class Enum_Complete_WithRangeView;

enum class Enum7 : uint8_t {
    A = 0x1,
    B = 0x2,
};

inline std::string Enum7Text(Enum7 tag) {
    switch (tag) {
        case Enum7::A: return "A";
        case Enum7::B: return "B";
        default:
            return std::string("Unknown Enum7: " +
                   std::to_string(static_cast<uint64_t>(tag)));
    }
}

enum class Enum16 : uint16_t {
    A = 0xaabb,
    B = 0xccdd,
};

inline std::string Enum16Text(Enum16 tag) {
    switch (tag) {
        case Enum16::A: return "A";
        case Enum16::B: return "B";
        default:
            return std::string("Unknown Enum16: " +
                   std::to_string(static_cast<uint64_t>(tag)));
    }
}

class SizedStruct : public pdl::packet::Builder {
public:
    ~SizedStruct() override = default;
    SizedStruct() = default;
    explicit SizedStruct(uint8_t a) : a_(std::move(a)) {}
    SizedStruct(SizedStruct const&) = default;
    SizedStruct(SizedStruct&&) = default;
    SizedStruct& operator=(SizedStruct const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, SizedStruct* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        output->a_ = span.read_le<uint8_t, 1>();
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(a_ & 0xff)));
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    uint8_t a_{0};
};

class UnsizedStruct : public pdl::packet::Builder {
public:
    ~UnsizedStruct() override = default;
    UnsizedStruct() = default;
    explicit UnsizedStruct(std::vector<uint8_t> array) : array_(std::move(array)) {}
    UnsizedStruct(UnsizedStruct const&) = default;
    UnsizedStruct(UnsizedStruct&&) = default;
    UnsizedStruct& operator=(UnsizedStruct const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, UnsizedStruct* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_size_ = (chunk0 >> 0) & 0x3;
        size_t limit = (span.size() > output->array_size_) ? (span.size() - output->array_size_) : 0;
        while (span.size() > limit) {
            if (span.size() < 1) return false;
            output->array_.push_back(span.read_le<uint8_t, 1>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = (array_.size() * 1);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 1));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<uint8_t> array_;
};

class UnknownSizeStruct : public pdl::packet::Builder {
public:
    ~UnknownSizeStruct() override = default;
    UnknownSizeStruct() = default;
    explicit UnknownSizeStruct(std::vector<uint8_t> array) : array_(std::move(array)) {}
    UnknownSizeStruct(UnknownSizeStruct const&) = default;
    UnknownSizeStruct(UnknownSizeStruct&&) = default;
    UnknownSizeStruct& operator=(UnknownSizeStruct const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, UnknownSizeStruct* output) {
        pdl::packet::slice span = parent_span;
        while (span.size() > 0) {
            if (span.size() < 1) return false;
            output->array_.push_back(span.read_le<uint8_t, 1>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 1);
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> array_;
};

class ScalarParentView {
public:
    static ScalarParentView Create(pdl::packet::slice const& parent) {
        return ScalarParentView(parent);
    }

    uint8_t GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::vector<uint8_t> GetPayload() const {
        _ASSERT_VALID(valid_);
        return payload_.bytes();
    }
    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ScalarParentView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return false;
        }
        a_ = span.read_le<uint8_t, 1>();
        payload_size_ = span.read_le<uint8_t, 1>();
        if (span.size() < payload_size_) return false;
        payload_ = span.subrange(0, payload_size_);
        span.skip(payload_size_);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t a_;
    uint8_t payload_size_ {0};
    pdl::packet::slice payload_;

    friend class EmptyParentView;
    friend class ScalarChild_AView;
    friend class ScalarChild_BView;
};

class ScalarParentBuilder : public pdl::packet::Builder {
public:
    ~ScalarParentBuilder() override = default;
    ScalarParentBuilder() = default;
    explicit ScalarParentBuilder(uint8_t a, std::vector<uint8_t> payload) : a_(std::move(a)), payload_(std::move(payload)) {}
    ScalarParentBuilder(ScalarParentBuilder const&) = default;
    ScalarParentBuilder(ScalarParentBuilder&&) = default;
    ScalarParentBuilder& operator=(ScalarParentBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(a_ & 0xff)));
        size_t payload_size = payload_.size();
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
    }

    size_t GetSize() const override {
        return 2 + (payload_.size());
    }

    std::string ToString() const { return ""; }

    uint8_t a_{0};
    uint8_t payload_size_ {0};
    std::vector<uint8_t> payload_;
};

class EnumParentView {
public:
    static EnumParentView Create(pdl::packet::slice const& parent) {
        return EnumParentView(parent);
    }

    Enum16 GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::vector<uint8_t> GetPayload() const {
        _ASSERT_VALID(valid_);
        return payload_.bytes();
    }
    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit EnumParentView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 3) {
            return false;
        }
        a_ = Enum16(span.read_le<uint16_t, 2>());
        payload_size_ = span.read_le<uint8_t, 1>();
        if (span.size() < payload_size_) return false;
        payload_ = span.subrange(0, payload_size_);
        span.skip(payload_size_);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Enum16 a_{Enum16::A};
    uint8_t payload_size_ {0};
    pdl::packet::slice payload_;

    friend class EnumChild_AView;
    friend class EnumChild_BView;
};

class EnumParentBuilder : public pdl::packet::Builder {
public:
    ~EnumParentBuilder() override = default;
    EnumParentBuilder() = default;
    explicit EnumParentBuilder(Enum16 a, std::vector<uint8_t> payload) : a_(std::move(a)), payload_(std::move(payload)) {}
    EnumParentBuilder(EnumParentBuilder const&) = default;
    EnumParentBuilder(EnumParentBuilder&&) = default;
    EnumParentBuilder& operator=(EnumParentBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(static_cast<uint16_t>(a_))));
        size_t payload_size = payload_.size();
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
    }

    size_t GetSize() const override {
        return 3 + (payload_.size());
    }

    std::string ToString() const { return ""; }

    Enum16 a_{Enum16::A};
    uint8_t payload_size_ {0};
    std::vector<uint8_t> payload_;
};

class EmptyParentView {
public:
    static EmptyParentView Create(ScalarParentView const& parent) {
        return EmptyParentView(parent);
    }

    uint8_t GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::vector<uint8_t> GetPayload() const {
        _ASSERT_VALID(valid_);
        return payload_.bytes();
    }
    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit EmptyParentView(ScalarParentView const& parent)
          : bytes_(parent.bytes_) {
        valid_ = Parse(parent);
    }

    bool Parse(ScalarParentView const& parent) {
        // Check validity of parent packet.
        if (!parent.IsValid()) { return false; }
        // Copy parent field values.
        a_ = parent.a_;
        // Parse packet field values.
        pdl::packet::slice span = parent.payload_;
        payload_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t a_;
    uint8_t payload_size_ {0};
    pdl::packet::slice payload_;

    friend class AliasedChild_AView;
    friend class AliasedChild_BView;
};

class EmptyParentBuilder : public pdl::packet::Builder {
public:
    ~EmptyParentBuilder() override = default;
    EmptyParentBuilder() = default;
    explicit EmptyParentBuilder(uint8_t a, std::vector<uint8_t> payload) : a_(std::move(a)), payload_(std::move(payload)) {}
    EmptyParentBuilder(EmptyParentBuilder const&) = default;
    EmptyParentBuilder(EmptyParentBuilder&&) = default;
    EmptyParentBuilder& operator=(EmptyParentBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(a_ & 0xff)));
        size_t payload_size = payload_.size();
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
    }

    size_t GetSize() const override {
        return 2 + (payload_.size());
    }

    std::string ToString() const { return ""; }

    uint8_t a_{0};
    uint8_t payload_size_ {0};
    std::vector<uint8_t> payload_;
};

class Packet_Scalar_FieldView {
public:
    static Packet_Scalar_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_Scalar_FieldView(parent);
    }

    uint8_t GetA() const { _ASSERT_VALID(valid_); return a_; }

    uint64_t GetC() const { _ASSERT_VALID(valid_); return c_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Scalar_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        a_ = (chunk0 >> 0) & 0x7f;
        c_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t a_;
    uint64_t c_;


};

class Packet_Scalar_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_Scalar_FieldBuilder() override = default;
    Packet_Scalar_FieldBuilder() = default;
    explicit Packet_Scalar_FieldBuilder(uint8_t a, uint64_t c) : a_(std::move(a)), c_(std::move(c)) {}
    Packet_Scalar_FieldBuilder(Packet_Scalar_FieldBuilder const&) = default;
    Packet_Scalar_FieldBuilder(Packet_Scalar_FieldBuilder&&) = default;
    Packet_Scalar_FieldBuilder& operator=(Packet_Scalar_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(a_ & 0x7f)) | (static_cast<uint64_t>(c_ & 0x1ffffffffffffff) << 7));
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint8_t a_{0};
    uint64_t c_{0};
};

class Packet_Enum_FieldView {
public:
    static Packet_Enum_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_Enum_FieldView(parent);
    }

    Enum7 GetA() const { _ASSERT_VALID(valid_); return a_; }

    uint64_t GetC() const { _ASSERT_VALID(valid_); return c_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Enum_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        a_ = Enum7((chunk0 >> 0) & 0x7f);
        c_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Enum7 a_{Enum7::A};
    uint64_t c_;


};

class Packet_Enum_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_Enum_FieldBuilder() override = default;
    Packet_Enum_FieldBuilder() = default;
    explicit Packet_Enum_FieldBuilder(Enum7 a, uint64_t c) : a_(std::move(a)), c_(std::move(c)) {}
    Packet_Enum_FieldBuilder(Packet_Enum_FieldBuilder const&) = default;
    Packet_Enum_FieldBuilder(Packet_Enum_FieldBuilder&&) = default;
    Packet_Enum_FieldBuilder& operator=(Packet_Enum_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(static_cast<uint8_t>(a_))) | (static_cast<uint64_t>(c_ & 0x1ffffffffffffff) << 7));
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    Enum7 a_{Enum7::A};
    uint64_t c_{0};
};

class Packet_Reserved_FieldView {
public:
    static Packet_Reserved_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_Reserved_FieldView(parent);
    }

    uint8_t GetA() const { _ASSERT_VALID(valid_); return a_; }

    uint64_t GetC() const { _ASSERT_VALID(valid_); return c_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Reserved_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        a_ = (chunk0 >> 0) & 0x7f;
        c_ = (chunk0 >> 9) & 0x7fffffffffffff;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t a_;
    uint64_t c_;


};

class Packet_Reserved_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_Reserved_FieldBuilder() override = default;
    Packet_Reserved_FieldBuilder() = default;
    explicit Packet_Reserved_FieldBuilder(uint8_t a, uint64_t c) : a_(std::move(a)), c_(std::move(c)) {}
    Packet_Reserved_FieldBuilder(Packet_Reserved_FieldBuilder const&) = default;
    Packet_Reserved_FieldBuilder(Packet_Reserved_FieldBuilder&&) = default;
    Packet_Reserved_FieldBuilder& operator=(Packet_Reserved_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(a_ & 0x7f)) | (static_cast<uint64_t>(c_ & 0x7fffffffffffff) << 9));
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint8_t a_{0};
    uint64_t c_{0};
};

class Packet_Size_FieldView {
public:
    static Packet_Size_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_Size_FieldView(parent);
    }

    uint64_t GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::vector<uint8_t> GetB() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = b_;
        std::vector<uint8_t> elements;
        while (span.size() > 0 && span.size() >= 1) {
            elements.push_back(span.read_le<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Size_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        b_size_ = (chunk0 >> 0) & 0x7;
        a_ = (chunk0 >> 3) & 0x1fffffffffffffff;
        if (span.size() < b_size_) return false;
        b_ = span.subrange(0, b_size_);
        span.skip(b_size_);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t b_size_ {0};
    uint64_t a_;
    pdl::packet::slice b_;


};

class Packet_Size_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_Size_FieldBuilder() override = default;
    Packet_Size_FieldBuilder() = default;
    explicit Packet_Size_FieldBuilder(uint64_t a, std::vector<uint8_t> b) : a_(std::move(a)), b_(std::move(b)) {}
    Packet_Size_FieldBuilder(Packet_Size_FieldBuilder const&) = default;
    Packet_Size_FieldBuilder(Packet_Size_FieldBuilder&&) = default;
    Packet_Size_FieldBuilder& operator=(Packet_Size_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t b_size = (b_.size() * 1);
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(b_size)) | (static_cast<uint64_t>(a_ & 0x1fffffffffffffff) << 3));
        for (auto const& element : b_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return 8 + ((b_.size() * 1));
    }

    std::string ToString() const { return ""; }

    uint8_t b_size_ {0};
    uint64_t a_{0};
    std::vector<uint8_t> b_;
};

class Packet_Count_FieldView {
public:
    static Packet_Count_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_Count_FieldView(parent);
    }

    uint64_t GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::vector<uint8_t> GetB() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = b_;
        std::vector<uint8_t> elements;
        while (elements.size() < b_count_ && span.size() >= 1) {
            elements.push_back(span.read_le<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Count_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        b_count_ = (chunk0 >> 0) & 0x7;
        a_ = (chunk0 >> 3) & 0x1fffffffffffffff;
        if (span.size() < b_count_ * 1) return false;
        b_ = span.subrange(0, b_count_ * 1);
        span.skip(b_count_ * 1);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t b_count_ {0};
    uint64_t a_;
    pdl::packet::slice b_;


};

class Packet_Count_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_Count_FieldBuilder() override = default;
    Packet_Count_FieldBuilder() = default;
    explicit Packet_Count_FieldBuilder(uint64_t a, std::vector<uint8_t> b) : a_(std::move(a)), b_(std::move(b)) {}
    Packet_Count_FieldBuilder(Packet_Count_FieldBuilder const&) = default;
    Packet_Count_FieldBuilder(Packet_Count_FieldBuilder&&) = default;
    Packet_Count_FieldBuilder& operator=(Packet_Count_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(b_.size())) | (static_cast<uint64_t>(a_ & 0x1fffffffffffffff) << 3));
        for (auto const& element : b_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return 8 + ((b_.size() * 1));
    }

    std::string ToString() const { return ""; }

    uint8_t b_count_ {0};
    uint64_t a_{0};
    std::vector<uint8_t> b_;
};

class Packet_FixedScalar_FieldView {
public:
    static Packet_FixedScalar_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_FixedScalar_FieldView(parent);
    }

    uint64_t GetB() const { _ASSERT_VALID(valid_); return b_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_FixedScalar_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        if (static_cast<uint64_t>((chunk0 >> 0) & 0x7f) != 0x7) {
            return false;
        }
        b_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint64_t b_;


};

class Packet_FixedScalar_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_FixedScalar_FieldBuilder() override = default;
    Packet_FixedScalar_FieldBuilder() = default;
    explicit Packet_FixedScalar_FieldBuilder(uint64_t b) : b_(std::move(b)) {}
    Packet_FixedScalar_FieldBuilder(Packet_FixedScalar_FieldBuilder const&) = default;
    Packet_FixedScalar_FieldBuilder(Packet_FixedScalar_FieldBuilder&&) = default;
    Packet_FixedScalar_FieldBuilder& operator=(Packet_FixedScalar_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(0x7)) | (static_cast<uint64_t>(b_ & 0x1ffffffffffffff) << 7));
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint64_t b_{0};
};

class Packet_FixedEnum_FieldView {
public:
    static Packet_FixedEnum_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_FixedEnum_FieldView(parent);
    }

    uint64_t GetB() const { _ASSERT_VALID(valid_); return b_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_FixedEnum_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        if (Enum7((chunk0 >> 0) & 0x7f) != Enum7::A) {
            return false;
        }
        b_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint64_t b_;


};

class Packet_FixedEnum_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_FixedEnum_FieldBuilder() override = default;
    Packet_FixedEnum_FieldBuilder() = default;
    explicit Packet_FixedEnum_FieldBuilder(uint64_t b) : b_(std::move(b)) {}
    Packet_FixedEnum_FieldBuilder(Packet_FixedEnum_FieldBuilder const&) = default;
    Packet_FixedEnum_FieldBuilder(Packet_FixedEnum_FieldBuilder&&) = default;
    Packet_FixedEnum_FieldBuilder& operator=(Packet_FixedEnum_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(Enum7::A)) | (static_cast<uint64_t>(b_ & 0x1ffffffffffffff) << 7));
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint64_t b_{0};
};

class Packet_Payload_Field_VariableSizeView {
public:
    static Packet_Payload_Field_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Payload_Field_VariableSizeView(parent);
    }

    std::vector<uint8_t> GetPayload() const {
        _ASSERT_VALID(valid_);
        return payload_.bytes();
    }
    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Payload_Field_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        payload_size_ = (chunk0 >> 0) & 0x7;
        if (span.size() < payload_size_) return false;
        payload_ = span.subrange(0, payload_size_);
        span.skip(payload_size_);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t payload_size_ {0};
    pdl::packet::slice payload_;


};

class Packet_Payload_Field_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Payload_Field_VariableSizeBuilder() override = default;
    Packet_Payload_Field_VariableSizeBuilder() = default;
    explicit Packet_Payload_Field_VariableSizeBuilder(std::vector<uint8_t> payload) : payload_(std::move(payload)) {}
    Packet_Payload_Field_VariableSizeBuilder(Packet_Payload_Field_VariableSizeBuilder const&) = default;
    Packet_Payload_Field_VariableSizeBuilder(Packet_Payload_Field_VariableSizeBuilder&&) = default;
    Packet_Payload_Field_VariableSizeBuilder& operator=(Packet_Payload_Field_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t payload_size = payload_.size();
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
    }

    size_t GetSize() const override {
        return 1 + (payload_.size());
    }

    std::string ToString() const { return ""; }

    uint8_t payload_size_ {0};
    std::vector<uint8_t> payload_;
};

class Packet_Payload_Field_SizeModifierView {
public:
    static Packet_Payload_Field_SizeModifierView Create(pdl::packet::slice const& parent) {
        return Packet_Payload_Field_SizeModifierView(parent);
    }

    std::vector<uint8_t> GetPayload() const {
        _ASSERT_VALID(valid_);
        return payload_.bytes();
    }
    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Payload_Field_SizeModifierView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        payload_size_ = (chunk0 >> 0) & 0x7;
        if (span.size() < (payload_size_ - 2)) return false;
        payload_ = span.subrange(0, (payload_size_ - 2));
        span.skip((payload_size_ - 2));
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t payload_size_ {0};
    pdl::packet::slice payload_;


};

class Packet_Payload_Field_SizeModifierBuilder : public pdl::packet::Builder {
public:
    ~Packet_Payload_Field_SizeModifierBuilder() override = default;
    Packet_Payload_Field_SizeModifierBuilder() = default;
    explicit Packet_Payload_Field_SizeModifierBuilder(std::vector<uint8_t> payload) : payload_(std::move(payload)) {}
    Packet_Payload_Field_SizeModifierBuilder(Packet_Payload_Field_SizeModifierBuilder const&) = default;
    Packet_Payload_Field_SizeModifierBuilder(Packet_Payload_Field_SizeModifierBuilder&&) = default;
    Packet_Payload_Field_SizeModifierBuilder& operator=(Packet_Payload_Field_SizeModifierBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t payload_size = (payload_.size() +2);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
    }

    size_t GetSize() const override {
        return 1 + (payload_.size());
    }

    std::string ToString() const { return ""; }

    uint8_t payload_size_ {0};
    std::vector<uint8_t> payload_;
};

class Packet_Payload_Field_UnknownSizeView {
public:
    static Packet_Payload_Field_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Payload_Field_UnknownSizeView(parent);
    }

    std::vector<uint8_t> GetPayload() const {
        _ASSERT_VALID(valid_);
        return payload_.bytes();
    }
    uint16_t GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Payload_Field_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 2) return false;
        payload_ = span.subrange(0, span.size() - 2);
        span.skip(span.size() - 2);
        if (span.size() < 2) {
            return false;
        }
        a_ = span.read_le<uint16_t, 2>();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice payload_;
    uint16_t a_;


};

class Packet_Payload_Field_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Payload_Field_UnknownSizeBuilder() override = default;
    Packet_Payload_Field_UnknownSizeBuilder() = default;
    explicit Packet_Payload_Field_UnknownSizeBuilder(std::vector<uint8_t> payload, uint16_t a) : payload_(std::move(payload)), a_(std::move(a)) {}
    Packet_Payload_Field_UnknownSizeBuilder(Packet_Payload_Field_UnknownSizeBuilder const&) = default;
    Packet_Payload_Field_UnknownSizeBuilder(Packet_Payload_Field_UnknownSizeBuilder&&) = default;
    Packet_Payload_Field_UnknownSizeBuilder& operator=(Packet_Payload_Field_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        output.insert(output.end(), payload_.begin(), payload_.end());
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(a_ & 0xffff)));
    }

    size_t GetSize() const override {
        return 2 + (payload_.size());
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> payload_;
    uint16_t a_{0};
};

class Packet_Payload_Field_UnknownSize_TerminalView {
public:
    static Packet_Payload_Field_UnknownSize_TerminalView Create(pdl::packet::slice const& parent) {
        return Packet_Payload_Field_UnknownSize_TerminalView(parent);
    }

    uint16_t GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::vector<uint8_t> GetPayload() const {
        _ASSERT_VALID(valid_);
        return payload_.bytes();
    }
    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Payload_Field_UnknownSize_TerminalView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return false;
        }
        a_ = span.read_le<uint16_t, 2>();
        payload_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint16_t a_;
    pdl::packet::slice payload_;


};

class Packet_Payload_Field_UnknownSize_TerminalBuilder : public pdl::packet::Builder {
public:
    ~Packet_Payload_Field_UnknownSize_TerminalBuilder() override = default;
    Packet_Payload_Field_UnknownSize_TerminalBuilder() = default;
    explicit Packet_Payload_Field_UnknownSize_TerminalBuilder(uint16_t a, std::vector<uint8_t> payload) : a_(std::move(a)), payload_(std::move(payload)) {}
    Packet_Payload_Field_UnknownSize_TerminalBuilder(Packet_Payload_Field_UnknownSize_TerminalBuilder const&) = default;
    Packet_Payload_Field_UnknownSize_TerminalBuilder(Packet_Payload_Field_UnknownSize_TerminalBuilder&&) = default;
    Packet_Payload_Field_UnknownSize_TerminalBuilder& operator=(Packet_Payload_Field_UnknownSize_TerminalBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(a_ & 0xffff)));
        output.insert(output.end(), payload_.begin(), payload_.end());
    }

    size_t GetSize() const override {
        return 2 + (payload_.size());
    }

    std::string ToString() const { return ""; }

    uint16_t a_{0};
    std::vector<uint8_t> payload_;
};

class Packet_Body_Field_VariableSizeView {
public:
    static Packet_Body_Field_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Body_Field_VariableSizeView(parent);
    }

    std::vector<uint8_t> GetPayload() const {
        _ASSERT_VALID(valid_);
        return payload_.bytes();
    }
    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Body_Field_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        payload_size_ = (chunk0 >> 0) & 0x7;
        if (span.size() < payload_size_) return false;
        payload_ = span.subrange(0, payload_size_);
        span.skip(payload_size_);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t payload_size_ {0};
    pdl::packet::slice payload_;


};

class Packet_Body_Field_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Body_Field_VariableSizeBuilder() override = default;
    Packet_Body_Field_VariableSizeBuilder() = default;
    explicit Packet_Body_Field_VariableSizeBuilder(std::vector<uint8_t> payload) : payload_(std::move(payload)) {}
    Packet_Body_Field_VariableSizeBuilder(Packet_Body_Field_VariableSizeBuilder const&) = default;
    Packet_Body_Field_VariableSizeBuilder(Packet_Body_Field_VariableSizeBuilder&&) = default;
    Packet_Body_Field_VariableSizeBuilder& operator=(Packet_Body_Field_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t payload_size = payload_.size();
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
    }

    size_t GetSize() const override {
        return 1 + (payload_.size());
    }

    std::string ToString() const { return ""; }

    uint8_t payload_size_ {0};
    std::vector<uint8_t> payload_;
};

class Packet_Body_Field_UnknownSizeView {
public:
    static Packet_Body_Field_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Body_Field_UnknownSizeView(parent);
    }

    std::vector<uint8_t> GetPayload() const {
        _ASSERT_VALID(valid_);
        return payload_.bytes();
    }
    uint16_t GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Body_Field_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 2) return false;
        payload_ = span.subrange(0, span.size() - 2);
        span.skip(span.size() - 2);
        if (span.size() < 2) {
            return false;
        }
        a_ = span.read_le<uint16_t, 2>();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice payload_;
    uint16_t a_;


};

class Packet_Body_Field_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Body_Field_UnknownSizeBuilder() override = default;
    Packet_Body_Field_UnknownSizeBuilder() = default;
    explicit Packet_Body_Field_UnknownSizeBuilder(std::vector<uint8_t> payload, uint16_t a) : payload_(std::move(payload)), a_(std::move(a)) {}
    Packet_Body_Field_UnknownSizeBuilder(Packet_Body_Field_UnknownSizeBuilder const&) = default;
    Packet_Body_Field_UnknownSizeBuilder(Packet_Body_Field_UnknownSizeBuilder&&) = default;
    Packet_Body_Field_UnknownSizeBuilder& operator=(Packet_Body_Field_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        output.insert(output.end(), payload_.begin(), payload_.end());
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(a_ & 0xffff)));
    }

    size_t GetSize() const override {
        return 2 + (payload_.size());
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> payload_;
    uint16_t a_{0};
};

class Packet_Body_Field_UnknownSize_TerminalView {
public:
    static Packet_Body_Field_UnknownSize_TerminalView Create(pdl::packet::slice const& parent) {
        return Packet_Body_Field_UnknownSize_TerminalView(parent);
    }

    uint16_t GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::vector<uint8_t> GetPayload() const {
        _ASSERT_VALID(valid_);
        return payload_.bytes();
    }
    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Body_Field_UnknownSize_TerminalView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return false;
        }
        a_ = span.read_le<uint16_t, 2>();
        payload_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint16_t a_;
    pdl::packet::slice payload_;


};

class Packet_Body_Field_UnknownSize_TerminalBuilder : public pdl::packet::Builder {
public:
    ~Packet_Body_Field_UnknownSize_TerminalBuilder() override = default;
    Packet_Body_Field_UnknownSize_TerminalBuilder() = default;
    explicit Packet_Body_Field_UnknownSize_TerminalBuilder(uint16_t a, std::vector<uint8_t> payload) : a_(std::move(a)), payload_(std::move(payload)) {}
    Packet_Body_Field_UnknownSize_TerminalBuilder(Packet_Body_Field_UnknownSize_TerminalBuilder const&) = default;
    Packet_Body_Field_UnknownSize_TerminalBuilder(Packet_Body_Field_UnknownSize_TerminalBuilder&&) = default;
    Packet_Body_Field_UnknownSize_TerminalBuilder& operator=(Packet_Body_Field_UnknownSize_TerminalBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(a_ & 0xffff)));
        output.insert(output.end(), payload_.begin(), payload_.end());
    }

    size_t GetSize() const override {
        return 2 + (payload_.size());
    }

    std::string ToString() const { return ""; }

    uint16_t a_{0};
    std::vector<uint8_t> payload_;
};

class Packet_ScalarGroup_FieldView {
public:
    static Packet_ScalarGroup_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_ScalarGroup_FieldView(parent);
    }


    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_ScalarGroup_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return false;
        }
        if (static_cast<uint64_t>(span.read_le<uint16_t, 2>()) != 0x2a) {
            return false;
        }
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;



};

class Packet_ScalarGroup_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_ScalarGroup_FieldBuilder() override = default;
    Packet_ScalarGroup_FieldBuilder() = default;
    Packet_ScalarGroup_FieldBuilder(Packet_ScalarGroup_FieldBuilder const&) = default;
    Packet_ScalarGroup_FieldBuilder(Packet_ScalarGroup_FieldBuilder&&) = default;
    Packet_ScalarGroup_FieldBuilder& operator=(Packet_ScalarGroup_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(0x2a)));
    }

    size_t GetSize() const override {
        return 2;
    }

    std::string ToString() const { return ""; }


};

class Packet_EnumGroup_FieldView {
public:
    static Packet_EnumGroup_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_EnumGroup_FieldView(parent);
    }


    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_EnumGroup_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return false;
        }
        if (Enum16(span.read_le<uint16_t, 2>()) != Enum16::A) {
            return false;
        }
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;



};

class Packet_EnumGroup_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_EnumGroup_FieldBuilder() override = default;
    Packet_EnumGroup_FieldBuilder() = default;
    Packet_EnumGroup_FieldBuilder(Packet_EnumGroup_FieldBuilder const&) = default;
    Packet_EnumGroup_FieldBuilder(Packet_EnumGroup_FieldBuilder&&) = default;
    Packet_EnumGroup_FieldBuilder& operator=(Packet_EnumGroup_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(Enum16::A)));
    }

    size_t GetSize() const override {
        return 2;
    }

    std::string ToString() const { return ""; }


};

class Packet_Struct_FieldView {
public:
    static Packet_Struct_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_Struct_FieldView(parent);
    }

    SizedStruct const& GetA() const { _ASSERT_VALID(valid_); return a_; }

    UnsizedStruct const& GetB() const { _ASSERT_VALID(valid_); return b_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Struct_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!SizedStruct::Parse(span, &a_)) return false;
        if (!UnsizedStruct::Parse(span, &b_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    SizedStruct a_;
    UnsizedStruct b_;


};

class Packet_Struct_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_Struct_FieldBuilder() override = default;
    Packet_Struct_FieldBuilder() = default;
    explicit Packet_Struct_FieldBuilder(SizedStruct a, UnsizedStruct b) : a_(std::move(a)), b_(std::move(b)) {}
    Packet_Struct_FieldBuilder(Packet_Struct_FieldBuilder const&) = default;
    Packet_Struct_FieldBuilder(Packet_Struct_FieldBuilder&&) = default;
    Packet_Struct_FieldBuilder& operator=(Packet_Struct_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        a_.Serialize(output);
        b_.Serialize(output);
    }

    size_t GetSize() const override {
        return a_.GetSize() + b_.GetSize();
    }

    std::string ToString() const { return ""; }

    SizedStruct a_;
    UnsizedStruct b_;
};

class Packet_Array_Field_ByteElement_ConstantSizeView {
public:
    static Packet_Array_Field_ByteElement_ConstantSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_ByteElement_ConstantSizeView(parent);
    }

    std::array<uint8_t, 4> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::array<uint8_t, 4> elements;
        for (int n = 0; n < 4; n++) {
            elements[n] = span.read_le<uint8_t, 1>();
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ByteElement_ConstantSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 4) {
            return false;
        }
        array_ = span.subrange(0, 4);
        span.skip(4);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice array_;


};

class Packet_Array_Field_ByteElement_ConstantSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_ByteElement_ConstantSizeBuilder() override = default;
    Packet_Array_Field_ByteElement_ConstantSizeBuilder() = default;
    explicit Packet_Array_Field_ByteElement_ConstantSizeBuilder(std::array<uint8_t, 4> array) : array_(std::move(array)) {}
    Packet_Array_Field_ByteElement_ConstantSizeBuilder(Packet_Array_Field_ByteElement_ConstantSizeBuilder const&) = default;
    Packet_Array_Field_ByteElement_ConstantSizeBuilder(Packet_Array_Field_ByteElement_ConstantSizeBuilder&&) = default;
    Packet_Array_Field_ByteElement_ConstantSizeBuilder& operator=(Packet_Array_Field_ByteElement_ConstantSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 1);
    }

    std::string ToString() const { return ""; }

    std::array<uint8_t, 4> array_;
};

class Packet_Array_Field_ByteElement_VariableSizeView {
public:
    static Packet_Array_Field_ByteElement_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_ByteElement_VariableSizeView(parent);
    }

    std::vector<uint8_t> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<uint8_t> elements;
        while (span.size() > 0 && span.size() >= 1) {
            elements.push_back(span.read_le<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ByteElement_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) return false;
        array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_size_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_ByteElement_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_ByteElement_VariableSizeBuilder() override = default;
    Packet_Array_Field_ByteElement_VariableSizeBuilder() = default;
    explicit Packet_Array_Field_ByteElement_VariableSizeBuilder(std::vector<uint8_t> array) : array_(std::move(array)) {}
    Packet_Array_Field_ByteElement_VariableSizeBuilder(Packet_Array_Field_ByteElement_VariableSizeBuilder const&) = default;
    Packet_Array_Field_ByteElement_VariableSizeBuilder(Packet_Array_Field_ByteElement_VariableSizeBuilder&&) = default;
    Packet_Array_Field_ByteElement_VariableSizeBuilder& operator=(Packet_Array_Field_ByteElement_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = (array_.size() * 1);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 1));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<uint8_t> array_;
};

class Packet_Array_Field_ByteElement_VariableCountView {
public:
    static Packet_Array_Field_ByteElement_VariableCountView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_ByteElement_VariableCountView(parent);
    }

    std::vector<uint8_t> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<uint8_t> elements;
        while (elements.size() < array_count_ && span.size() >= 1) {
            elements.push_back(span.read_le<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ByteElement_VariableCountView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_count_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_count_ * 1) return false;
        array_ = span.subrange(0, array_count_ * 1);
        span.skip(array_count_ * 1);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_count_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_ByteElement_VariableCountBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_ByteElement_VariableCountBuilder() override = default;
    Packet_Array_Field_ByteElement_VariableCountBuilder() = default;
    explicit Packet_Array_Field_ByteElement_VariableCountBuilder(std::vector<uint8_t> array) : array_(std::move(array)) {}
    Packet_Array_Field_ByteElement_VariableCountBuilder(Packet_Array_Field_ByteElement_VariableCountBuilder const&) = default;
    Packet_Array_Field_ByteElement_VariableCountBuilder(Packet_Array_Field_ByteElement_VariableCountBuilder&&) = default;
    Packet_Array_Field_ByteElement_VariableCountBuilder& operator=(Packet_Array_Field_ByteElement_VariableCountBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 1));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<uint8_t> array_;
};

class Packet_Array_Field_ByteElement_UnknownSizeView {
public:
    static Packet_Array_Field_ByteElement_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_ByteElement_UnknownSizeView(parent);
    }

    std::vector<uint8_t> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<uint8_t> elements;
        while (span.size() > 0 && span.size() >= 1) {
            elements.push_back(span.read_le<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ByteElement_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        array_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice array_;


};

class Packet_Array_Field_ByteElement_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_ByteElement_UnknownSizeBuilder() override = default;
    Packet_Array_Field_ByteElement_UnknownSizeBuilder() = default;
    explicit Packet_Array_Field_ByteElement_UnknownSizeBuilder(std::vector<uint8_t> array) : array_(std::move(array)) {}
    Packet_Array_Field_ByteElement_UnknownSizeBuilder(Packet_Array_Field_ByteElement_UnknownSizeBuilder const&) = default;
    Packet_Array_Field_ByteElement_UnknownSizeBuilder(Packet_Array_Field_ByteElement_UnknownSizeBuilder&&) = default;
    Packet_Array_Field_ByteElement_UnknownSizeBuilder& operator=(Packet_Array_Field_ByteElement_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 1);
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> array_;
};

class Packet_Array_Field_ScalarElement_ConstantSizeView {
public:
    static Packet_Array_Field_ScalarElement_ConstantSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_ScalarElement_ConstantSizeView(parent);
    }

    std::array<uint16_t, 4> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::array<uint16_t, 4> elements;
        for (int n = 0; n < 4; n++) {
            elements[n] = span.read_le<uint16_t, 2>();
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ScalarElement_ConstantSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return false;
        }
        array_ = span.subrange(0, 8);
        span.skip(8);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice array_;


};

class Packet_Array_Field_ScalarElement_ConstantSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_ScalarElement_ConstantSizeBuilder() override = default;
    Packet_Array_Field_ScalarElement_ConstantSizeBuilder() = default;
    explicit Packet_Array_Field_ScalarElement_ConstantSizeBuilder(std::array<uint16_t, 4> array) : array_(std::move(array)) {}
    Packet_Array_Field_ScalarElement_ConstantSizeBuilder(Packet_Array_Field_ScalarElement_ConstantSizeBuilder const&) = default;
    Packet_Array_Field_ScalarElement_ConstantSizeBuilder(Packet_Array_Field_ScalarElement_ConstantSizeBuilder&&) = default;
    Packet_Array_Field_ScalarElement_ConstantSizeBuilder& operator=(Packet_Array_Field_ScalarElement_ConstantSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 2);
    }

    std::string ToString() const { return ""; }

    std::array<uint16_t, 4> array_;
};

class Packet_Array_Field_ScalarElement_VariableSizeView {
public:
    static Packet_Array_Field_ScalarElement_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_ScalarElement_VariableSizeView(parent);
    }

    std::vector<uint16_t> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<uint16_t> elements;
        while (span.size() > 0 && span.size() >= 2) {
            elements.push_back(span.read_le<uint16_t, 2>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ScalarElement_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) return false;
        array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_size_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_ScalarElement_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_ScalarElement_VariableSizeBuilder() override = default;
    Packet_Array_Field_ScalarElement_VariableSizeBuilder() = default;
    explicit Packet_Array_Field_ScalarElement_VariableSizeBuilder(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Packet_Array_Field_ScalarElement_VariableSizeBuilder(Packet_Array_Field_ScalarElement_VariableSizeBuilder const&) = default;
    Packet_Array_Field_ScalarElement_VariableSizeBuilder(Packet_Array_Field_ScalarElement_VariableSizeBuilder&&) = default;
    Packet_Array_Field_ScalarElement_VariableSizeBuilder& operator=(Packet_Array_Field_ScalarElement_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = (array_.size() * 2);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 2));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<uint16_t> array_;
};

class Packet_Array_Field_ScalarElement_VariableCountView {
public:
    static Packet_Array_Field_ScalarElement_VariableCountView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_ScalarElement_VariableCountView(parent);
    }

    std::vector<uint16_t> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<uint16_t> elements;
        while (elements.size() < array_count_ && span.size() >= 2) {
            elements.push_back(span.read_le<uint16_t, 2>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ScalarElement_VariableCountView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_count_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_count_ * 2) return false;
        array_ = span.subrange(0, array_count_ * 2);
        span.skip(array_count_ * 2);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_count_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_ScalarElement_VariableCountBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_ScalarElement_VariableCountBuilder() override = default;
    Packet_Array_Field_ScalarElement_VariableCountBuilder() = default;
    explicit Packet_Array_Field_ScalarElement_VariableCountBuilder(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Packet_Array_Field_ScalarElement_VariableCountBuilder(Packet_Array_Field_ScalarElement_VariableCountBuilder const&) = default;
    Packet_Array_Field_ScalarElement_VariableCountBuilder(Packet_Array_Field_ScalarElement_VariableCountBuilder&&) = default;
    Packet_Array_Field_ScalarElement_VariableCountBuilder& operator=(Packet_Array_Field_ScalarElement_VariableCountBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 2));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<uint16_t> array_;
};

class Packet_Array_Field_ScalarElement_UnknownSizeView {
public:
    static Packet_Array_Field_ScalarElement_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_ScalarElement_UnknownSizeView(parent);
    }

    std::vector<uint16_t> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<uint16_t> elements;
        while (span.size() > 0 && span.size() >= 2) {
            elements.push_back(span.read_le<uint16_t, 2>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ScalarElement_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        array_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice array_;


};

class Packet_Array_Field_ScalarElement_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_ScalarElement_UnknownSizeBuilder() override = default;
    Packet_Array_Field_ScalarElement_UnknownSizeBuilder() = default;
    explicit Packet_Array_Field_ScalarElement_UnknownSizeBuilder(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Packet_Array_Field_ScalarElement_UnknownSizeBuilder(Packet_Array_Field_ScalarElement_UnknownSizeBuilder const&) = default;
    Packet_Array_Field_ScalarElement_UnknownSizeBuilder(Packet_Array_Field_ScalarElement_UnknownSizeBuilder&&) = default;
    Packet_Array_Field_ScalarElement_UnknownSizeBuilder& operator=(Packet_Array_Field_ScalarElement_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 2);
    }

    std::string ToString() const { return ""; }

    std::vector<uint16_t> array_;
};

class Packet_Array_Field_EnumElement_ConstantSizeView {
public:
    static Packet_Array_Field_EnumElement_ConstantSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_EnumElement_ConstantSizeView(parent);
    }

    std::array<Enum16, 4> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::array<Enum16, 4> elements;
        for (int n = 0; n < 4; n++) {
            elements[n] = Enum16(span.read_le<uint16_t, 2>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_EnumElement_ConstantSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return false;
        }
        array_ = span.subrange(0, 8);
        span.skip(8);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice array_;


};

class Packet_Array_Field_EnumElement_ConstantSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_EnumElement_ConstantSizeBuilder() override = default;
    Packet_Array_Field_EnumElement_ConstantSizeBuilder() = default;
    explicit Packet_Array_Field_EnumElement_ConstantSizeBuilder(std::array<Enum16, 4> array) : array_(std::move(array)) {}
    Packet_Array_Field_EnumElement_ConstantSizeBuilder(Packet_Array_Field_EnumElement_ConstantSizeBuilder const&) = default;
    Packet_Array_Field_EnumElement_ConstantSizeBuilder(Packet_Array_Field_EnumElement_ConstantSizeBuilder&&) = default;
    Packet_Array_Field_EnumElement_ConstantSizeBuilder& operator=(Packet_Array_Field_EnumElement_ConstantSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 2);
    }

    std::string ToString() const { return ""; }

    std::array<Enum16, 4> array_;
};

class Packet_Array_Field_EnumElement_VariableSizeView {
public:
    static Packet_Array_Field_EnumElement_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_EnumElement_VariableSizeView(parent);
    }

    std::vector<Enum16> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<Enum16> elements;
        while (span.size() > 0 && span.size() >= 2) {
            elements.push_back(Enum16(span.read_le<uint16_t, 2>()));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_EnumElement_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) return false;
        array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_size_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_EnumElement_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_EnumElement_VariableSizeBuilder() override = default;
    Packet_Array_Field_EnumElement_VariableSizeBuilder() = default;
    explicit Packet_Array_Field_EnumElement_VariableSizeBuilder(std::vector<Enum16> array) : array_(std::move(array)) {}
    Packet_Array_Field_EnumElement_VariableSizeBuilder(Packet_Array_Field_EnumElement_VariableSizeBuilder const&) = default;
    Packet_Array_Field_EnumElement_VariableSizeBuilder(Packet_Array_Field_EnumElement_VariableSizeBuilder&&) = default;
    Packet_Array_Field_EnumElement_VariableSizeBuilder& operator=(Packet_Array_Field_EnumElement_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = (array_.size() * 2);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 2));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<Enum16> array_;
};

class Packet_Array_Field_EnumElement_VariableCountView {
public:
    static Packet_Array_Field_EnumElement_VariableCountView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_EnumElement_VariableCountView(parent);
    }

    std::vector<Enum16> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<Enum16> elements;
        while (elements.size() < array_count_ && span.size() >= 2) {
            elements.push_back(Enum16(span.read_le<uint16_t, 2>()));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_EnumElement_VariableCountView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_count_ = (chunk0 >> 0) & 0xf;
        array_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_count_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_EnumElement_VariableCountBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_EnumElement_VariableCountBuilder() override = default;
    Packet_Array_Field_EnumElement_VariableCountBuilder() = default;
    explicit Packet_Array_Field_EnumElement_VariableCountBuilder(std::vector<Enum16> array) : array_(std::move(array)) {}
    Packet_Array_Field_EnumElement_VariableCountBuilder(Packet_Array_Field_EnumElement_VariableCountBuilder const&) = default;
    Packet_Array_Field_EnumElement_VariableCountBuilder(Packet_Array_Field_EnumElement_VariableCountBuilder&&) = default;
    Packet_Array_Field_EnumElement_VariableCountBuilder& operator=(Packet_Array_Field_EnumElement_VariableCountBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 2));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<Enum16> array_;
};

class Packet_Array_Field_EnumElement_UnknownSizeView {
public:
    static Packet_Array_Field_EnumElement_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_EnumElement_UnknownSizeView(parent);
    }

    std::vector<Enum16> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<Enum16> elements;
        while (span.size() > 0 && span.size() >= 2) {
            elements.push_back(Enum16(span.read_le<uint16_t, 2>()));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_EnumElement_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        array_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice array_;


};

class Packet_Array_Field_EnumElement_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_EnumElement_UnknownSizeBuilder() override = default;
    Packet_Array_Field_EnumElement_UnknownSizeBuilder() = default;
    explicit Packet_Array_Field_EnumElement_UnknownSizeBuilder(std::vector<Enum16> array) : array_(std::move(array)) {}
    Packet_Array_Field_EnumElement_UnknownSizeBuilder(Packet_Array_Field_EnumElement_UnknownSizeBuilder const&) = default;
    Packet_Array_Field_EnumElement_UnknownSizeBuilder(Packet_Array_Field_EnumElement_UnknownSizeBuilder&&) = default;
    Packet_Array_Field_EnumElement_UnknownSizeBuilder& operator=(Packet_Array_Field_EnumElement_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 2);
    }

    std::string ToString() const { return ""; }

    std::vector<Enum16> array_;
};

class Packet_Array_Field_SizedElement_ConstantSizeView {
public:
    static Packet_Array_Field_SizedElement_ConstantSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_SizedElement_ConstantSizeView(parent);
    }

    std::array<SizedStruct, 4> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::array<SizedStruct, 4> elements;
        for (int n = 0; n < 4; n++) {
            SizedStruct::Parse(span, &elements[n]);
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_SizedElement_ConstantSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 4) {
            return false;
        }
        array_ = span.subrange(0, 4);
        span.skip(4);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice array_;


};

class Packet_Array_Field_SizedElement_ConstantSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_SizedElement_ConstantSizeBuilder() override = default;
    Packet_Array_Field_SizedElement_ConstantSizeBuilder() = default;
    explicit Packet_Array_Field_SizedElement_ConstantSizeBuilder(std::array<SizedStruct, 4> array) : array_(std::move(array)) {}
    Packet_Array_Field_SizedElement_ConstantSizeBuilder(Packet_Array_Field_SizedElement_ConstantSizeBuilder const&) = default;
    Packet_Array_Field_SizedElement_ConstantSizeBuilder(Packet_Array_Field_SizedElement_ConstantSizeBuilder&&) = default;
    Packet_Array_Field_SizedElement_ConstantSizeBuilder& operator=(Packet_Array_Field_SizedElement_ConstantSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::array<SizedStruct, 4> array_;
};

class Packet_Array_Field_SizedElement_VariableSizeView {
public:
    static Packet_Array_Field_SizedElement_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_SizedElement_VariableSizeView(parent);
    }

    std::vector<SizedStruct> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<SizedStruct> elements;
        while (span.size() > 0) {
            SizedStruct element;
            if (!SizedStruct::Parse(span, &element)) break;
            elements.emplace_back(std::move(element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_SizedElement_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) return false;
        array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_size_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_SizedElement_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_SizedElement_VariableSizeBuilder() override = default;
    Packet_Array_Field_SizedElement_VariableSizeBuilder() = default;
    explicit Packet_Array_Field_SizedElement_VariableSizeBuilder(std::vector<SizedStruct> array) : array_(std::move(array)) {}
    Packet_Array_Field_SizedElement_VariableSizeBuilder(Packet_Array_Field_SizedElement_VariableSizeBuilder const&) = default;
    Packet_Array_Field_SizedElement_VariableSizeBuilder(Packet_Array_Field_SizedElement_VariableSizeBuilder&&) = default;
    Packet_Array_Field_SizedElement_VariableSizeBuilder& operator=(Packet_Array_Field_SizedElement_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& element) { return s + element.GetSize(); });
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<SizedStruct> array_;
};

class Packet_Array_Field_SizedElement_VariableCountView {
public:
    static Packet_Array_Field_SizedElement_VariableCountView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_SizedElement_VariableCountView(parent);
    }

    std::vector<SizedStruct> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<SizedStruct> elements;
        while (elements.size() < array_count_) {
            SizedStruct element;
            if (!SizedStruct::Parse(span, &element)) break;
            elements.emplace_back(std::move(element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_SizedElement_VariableCountView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_count_ = (chunk0 >> 0) & 0xf;
        array_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_count_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_SizedElement_VariableCountBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_SizedElement_VariableCountBuilder() override = default;
    Packet_Array_Field_SizedElement_VariableCountBuilder() = default;
    explicit Packet_Array_Field_SizedElement_VariableCountBuilder(std::vector<SizedStruct> array) : array_(std::move(array)) {}
    Packet_Array_Field_SizedElement_VariableCountBuilder(Packet_Array_Field_SizedElement_VariableCountBuilder const&) = default;
    Packet_Array_Field_SizedElement_VariableCountBuilder(Packet_Array_Field_SizedElement_VariableCountBuilder&&) = default;
    Packet_Array_Field_SizedElement_VariableCountBuilder& operator=(Packet_Array_Field_SizedElement_VariableCountBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<SizedStruct> array_;
};

class Packet_Array_Field_SizedElement_UnknownSizeView {
public:
    static Packet_Array_Field_SizedElement_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_SizedElement_UnknownSizeView(parent);
    }

    std::vector<SizedStruct> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<SizedStruct> elements;
        while (span.size() > 0) {
            SizedStruct element;
            if (!SizedStruct::Parse(span, &element)) break;
            elements.emplace_back(std::move(element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_SizedElement_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        array_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice array_;


};

class Packet_Array_Field_SizedElement_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_SizedElement_UnknownSizeBuilder() override = default;
    Packet_Array_Field_SizedElement_UnknownSizeBuilder() = default;
    explicit Packet_Array_Field_SizedElement_UnknownSizeBuilder(std::vector<SizedStruct> array) : array_(std::move(array)) {}
    Packet_Array_Field_SizedElement_UnknownSizeBuilder(Packet_Array_Field_SizedElement_UnknownSizeBuilder const&) = default;
    Packet_Array_Field_SizedElement_UnknownSizeBuilder(Packet_Array_Field_SizedElement_UnknownSizeBuilder&&) = default;
    Packet_Array_Field_SizedElement_UnknownSizeBuilder& operator=(Packet_Array_Field_SizedElement_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<SizedStruct> array_;
};

class Packet_Array_Field_UnsizedElement_ConstantSizeView {
public:
    static Packet_Array_Field_UnsizedElement_ConstantSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_UnsizedElement_ConstantSizeView(parent);
    }

    std::array<UnsizedStruct, 4> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::array<UnsizedStruct, 4> elements;
        for (int n = 0; n < 4; n++) {
            UnsizedStruct::Parse(span, &elements[n]);
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_ConstantSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        array_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice array_;


};

class Packet_Array_Field_UnsizedElement_ConstantSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_UnsizedElement_ConstantSizeBuilder() override = default;
    Packet_Array_Field_UnsizedElement_ConstantSizeBuilder() = default;
    explicit Packet_Array_Field_UnsizedElement_ConstantSizeBuilder(std::array<UnsizedStruct, 4> array) : array_(std::move(array)) {}
    Packet_Array_Field_UnsizedElement_ConstantSizeBuilder(Packet_Array_Field_UnsizedElement_ConstantSizeBuilder const&) = default;
    Packet_Array_Field_UnsizedElement_ConstantSizeBuilder(Packet_Array_Field_UnsizedElement_ConstantSizeBuilder&&) = default;
    Packet_Array_Field_UnsizedElement_ConstantSizeBuilder& operator=(Packet_Array_Field_UnsizedElement_ConstantSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::array<UnsizedStruct, 4> array_;
};

class Packet_Array_Field_UnsizedElement_VariableSizeView {
public:
    static Packet_Array_Field_UnsizedElement_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_UnsizedElement_VariableSizeView(parent);
    }

    std::vector<UnsizedStruct> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<UnsizedStruct> elements;
        while (span.size() > 0) {
            UnsizedStruct element;
            if (!UnsizedStruct::Parse(span, &element)) break;
            elements.emplace_back(std::move(element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) return false;
        array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_size_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_UnsizedElement_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_UnsizedElement_VariableSizeBuilder() override = default;
    Packet_Array_Field_UnsizedElement_VariableSizeBuilder() = default;
    explicit Packet_Array_Field_UnsizedElement_VariableSizeBuilder(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Packet_Array_Field_UnsizedElement_VariableSizeBuilder(Packet_Array_Field_UnsizedElement_VariableSizeBuilder const&) = default;
    Packet_Array_Field_UnsizedElement_VariableSizeBuilder(Packet_Array_Field_UnsizedElement_VariableSizeBuilder&&) = default;
    Packet_Array_Field_UnsizedElement_VariableSizeBuilder& operator=(Packet_Array_Field_UnsizedElement_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& element) { return s + element.GetSize(); });
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<UnsizedStruct> array_;
};

class Packet_Array_Field_UnsizedElement_VariableCountView {
public:
    static Packet_Array_Field_UnsizedElement_VariableCountView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_UnsizedElement_VariableCountView(parent);
    }

    std::vector<UnsizedStruct> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<UnsizedStruct> elements;
        while (elements.size() < array_count_) {
            UnsizedStruct element;
            if (!UnsizedStruct::Parse(span, &element)) break;
            elements.emplace_back(std::move(element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_VariableCountView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_count_ = (chunk0 >> 0) & 0xf;
        array_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_count_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_UnsizedElement_VariableCountBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_UnsizedElement_VariableCountBuilder() override = default;
    Packet_Array_Field_UnsizedElement_VariableCountBuilder() = default;
    explicit Packet_Array_Field_UnsizedElement_VariableCountBuilder(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Packet_Array_Field_UnsizedElement_VariableCountBuilder(Packet_Array_Field_UnsizedElement_VariableCountBuilder const&) = default;
    Packet_Array_Field_UnsizedElement_VariableCountBuilder(Packet_Array_Field_UnsizedElement_VariableCountBuilder&&) = default;
    Packet_Array_Field_UnsizedElement_VariableCountBuilder& operator=(Packet_Array_Field_UnsizedElement_VariableCountBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<UnsizedStruct> array_;
};

class Packet_Array_Field_UnsizedElement_UnknownSizeView {
public:
    static Packet_Array_Field_UnsizedElement_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_UnsizedElement_UnknownSizeView(parent);
    }

    std::vector<UnsizedStruct> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<UnsizedStruct> elements;
        while (span.size() > 0) {
            UnsizedStruct element;
            if (!UnsizedStruct::Parse(span, &element)) break;
            elements.emplace_back(std::move(element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        array_ = span;
        span.clear();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    pdl::packet::slice array_;


};

class Packet_Array_Field_UnsizedElement_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_UnsizedElement_UnknownSizeBuilder() override = default;
    Packet_Array_Field_UnsizedElement_UnknownSizeBuilder() = default;
    explicit Packet_Array_Field_UnsizedElement_UnknownSizeBuilder(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Packet_Array_Field_UnsizedElement_UnknownSizeBuilder(Packet_Array_Field_UnsizedElement_UnknownSizeBuilder const&) = default;
    Packet_Array_Field_UnsizedElement_UnknownSizeBuilder(Packet_Array_Field_UnsizedElement_UnknownSizeBuilder&&) = default;
    Packet_Array_Field_UnsizedElement_UnknownSizeBuilder& operator=(Packet_Array_Field_UnsizedElement_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<UnsizedStruct> array_;
};

class Packet_Array_Field_UnsizedElement_SizeModifierView {
public:
    static Packet_Array_Field_UnsizedElement_SizeModifierView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_UnsizedElement_SizeModifierView(parent);
    }

    std::vector<UnsizedStruct> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<UnsizedStruct> elements;
        while (span.size() > 0) {
            UnsizedStruct element;
            if (!UnsizedStruct::Parse(span, &element)) break;
            elements.emplace_back(std::move(element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_SizeModifierView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < (array_size_ - 2)) return false;
        array_ = span.subrange(0, (array_size_ - 2));
        span.skip((array_size_ - 2));
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_size_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_UnsizedElement_SizeModifierBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_UnsizedElement_SizeModifierBuilder() override = default;
    Packet_Array_Field_UnsizedElement_SizeModifierBuilder() = default;
    explicit Packet_Array_Field_UnsizedElement_SizeModifierBuilder(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Packet_Array_Field_UnsizedElement_SizeModifierBuilder(Packet_Array_Field_UnsizedElement_SizeModifierBuilder const&) = default;
    Packet_Array_Field_UnsizedElement_SizeModifierBuilder(Packet_Array_Field_UnsizedElement_SizeModifierBuilder&&) = default;
    Packet_Array_Field_UnsizedElement_SizeModifierBuilder& operator=(Packet_Array_Field_UnsizedElement_SizeModifierBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& element) { return s + element.GetSize(); }) +2);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<UnsizedStruct> array_;
};

class Packet_Array_Field_SizedElement_VariableSize_PaddedView {
public:
    static Packet_Array_Field_SizedElement_VariableSize_PaddedView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_SizedElement_VariableSize_PaddedView(parent);
    }

    std::vector<uint16_t> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<uint16_t> elements;
        while (span.size() > 0 && span.size() >= 2) {
            elements.push_back(span.read_le<uint16_t, 2>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_SizedElement_VariableSize_PaddedView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        array_size_ = (chunk0 >> 0) & 0xf;
        size_t array_start_size = span.size();
        if (span.size() < array_size_) return false;
        array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        if (array_start_size - span.size() < 16) {
            if (span.size() < 16 - (array_start_size - span.size())) return false;
            span.skip(16 - (array_start_size - span.size()));
        }
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_size_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_SizedElement_VariableSize_PaddedBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_SizedElement_VariableSize_PaddedBuilder() override = default;
    Packet_Array_Field_SizedElement_VariableSize_PaddedBuilder() = default;
    explicit Packet_Array_Field_SizedElement_VariableSize_PaddedBuilder(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Packet_Array_Field_SizedElement_VariableSize_PaddedBuilder(Packet_Array_Field_SizedElement_VariableSize_PaddedBuilder const&) = default;
    Packet_Array_Field_SizedElement_VariableSize_PaddedBuilder(Packet_Array_Field_SizedElement_VariableSize_PaddedBuilder&&) = default;
    Packet_Array_Field_SizedElement_VariableSize_PaddedBuilder& operator=(Packet_Array_Field_SizedElement_VariableSize_PaddedBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = (array_.size() * 2);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        size_t array_start = output.size();
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
        if (output.size() - array_start < 16) {
            output.resize(array_start + 16, 0);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::max<size_t>((array_.size() * 2), 16));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<uint16_t> array_;
};

class Packet_Array_Field_UnsizedElement_VariableCount_PaddedView {
public:
    static Packet_Array_Field_UnsizedElement_VariableCount_PaddedView Create(pdl::packet::slice const& parent) {
        return Packet_Array_Field_UnsizedElement_VariableCount_PaddedView(parent);
    }

    std::vector<UnsizedStruct> GetArray() const {
        _ASSERT_VALID(valid_);
        pdl::packet::slice span = array_;
        std::vector<UnsizedStruct> elements;
        while (elements.size() < array_count_) {
            UnsizedStruct element;
            if (!UnsizedStruct::Parse(span, &element)) break;
            elements.emplace_back(std::move(element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_VariableCount_PaddedView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        array_count_ = span.read_le<uint8_t, 1>();
        size_t array_start_size = span.size();
        array_ = span;
        span.clear();
        if (array_start_size - span.size() < 16) {
            if (span.size() < 16 - (array_start_size - span.size())) return false;
            span.skip(16 - (array_start_size - span.size()));
        }
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t array_count_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_UnsizedElement_VariableCount_PaddedBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_UnsizedElement_VariableCount_PaddedBuilder() override = default;
    Packet_Array_Field_UnsizedElement_VariableCount_PaddedBuilder() = default;
    explicit Packet_Array_Field_UnsizedElement_VariableCount_PaddedBuilder(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Packet_Array_Field_UnsizedElement_VariableCount_PaddedBuilder(Packet_Array_Field_UnsizedElement_VariableCount_PaddedBuilder const&) = default;
    Packet_Array_Field_UnsizedElement_VariableCount_PaddedBuilder(Packet_Array_Field_UnsizedElement_VariableCount_PaddedBuilder&&) = default;
    Packet_Array_Field_UnsizedElement_VariableCount_PaddedBuilder& operator=(Packet_Array_Field_UnsizedElement_VariableCount_PaddedBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        size_t array_start = output.size();
        for (auto const& element : array_) {
            element.Serialize(output);
        }
        if (output.size() - array_start < 16) {
            output.resize(array_start + 16, 0);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::max<size_t>(std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }), 16));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<UnsizedStruct> array_;
};

class Packet_Optional_Scalar_FieldView {
public:
    static Packet_Optional_Scalar_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_Optional_Scalar_FieldView(parent);
    }

    std::optional<uint32_t> GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::optional<uint32_t> GetB() const { _ASSERT_VALID(valid_); return b_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Optional_Scalar_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            if (span.size() < 3) {
                return false;
            }
            a_ = span.read_le<uint32_t, 3>();
        }
        if (c1 == 1) {
            if (span.size() < 4) {
                return false;
            }
            b_ = span.read_le<uint32_t, 4>();
        }
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    std::optional<uint32_t> a_;
    std::optional<uint32_t> b_;


};

class Packet_Optional_Scalar_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_Optional_Scalar_FieldBuilder() override = default;
    Packet_Optional_Scalar_FieldBuilder() = default;
    explicit Packet_Optional_Scalar_FieldBuilder(std::optional<uint32_t> a, std::optional<uint32_t> b) : a_(std::move(a)), b_(std::move(b)) {}
    Packet_Optional_Scalar_FieldBuilder(Packet_Optional_Scalar_FieldBuilder const&) = default;
    Packet_Optional_Scalar_FieldBuilder(Packet_Optional_Scalar_FieldBuilder&&) = default;
    Packet_Optional_Scalar_FieldBuilder& operator=(Packet_Optional_Scalar_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>((a_.has_value() ? 0 : 1))) | (static_cast<uint8_t>((b_.has_value() ? 1 : 0)) << 1));
        if ((a_.has_value() ? 0 : 1) == 0) {
            pdl::packet::Builder::write_le<uint32_t, 3>(output, *a_);
        }
        if ((b_.has_value() ? 1 : 0) == 1) {
            pdl::packet::Builder::write_le<uint32_t, 4>(output, *b_);
        }
    }

    size_t GetSize() const override {
        return 1 + ((((a_.has_value() ? 0 : 1) == 0) ? 3 : 0) + (((b_.has_value() ? 1 : 0) == 1) ? 4 : 0));
    }

    std::string ToString() const { return ""; }

    std::optional<uint32_t> a_;
    std::optional<uint32_t> b_;
};

class Packet_Optional_Enum_FieldView {
public:
    static Packet_Optional_Enum_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_Optional_Enum_FieldView(parent);
    }

    std::optional<Enum16> GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::optional<Enum16> GetB() const { _ASSERT_VALID(valid_); return b_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Optional_Enum_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            if (span.size() < 2) {
                return false;
            }
            a_ = Enum16(span.read_le<uint16_t, 2>());
        }
        if (c1 == 1) {
            if (span.size() < 2) {
                return false;
            }
            b_ = Enum16(span.read_le<uint16_t, 2>());
        }
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    std::optional<Enum16> a_;
    std::optional<Enum16> b_;


};

class Packet_Optional_Enum_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_Optional_Enum_FieldBuilder() override = default;
    Packet_Optional_Enum_FieldBuilder() = default;
    explicit Packet_Optional_Enum_FieldBuilder(std::optional<Enum16> a, std::optional<Enum16> b) : a_(std::move(a)), b_(std::move(b)) {}
    Packet_Optional_Enum_FieldBuilder(Packet_Optional_Enum_FieldBuilder const&) = default;
    Packet_Optional_Enum_FieldBuilder(Packet_Optional_Enum_FieldBuilder&&) = default;
    Packet_Optional_Enum_FieldBuilder& operator=(Packet_Optional_Enum_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>((a_.has_value() ? 0 : 1))) | (static_cast<uint8_t>((b_.has_value() ? 1 : 0)) << 1));
        if ((a_.has_value() ? 0 : 1) == 0) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(*a_));
        }
        if ((b_.has_value() ? 1 : 0) == 1) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(*b_));
        }
    }

    size_t GetSize() const override {
        return 1 + ((((a_.has_value() ? 0 : 1) == 0) ? 2 : 0) + (((b_.has_value() ? 1 : 0) == 1) ? 2 : 0));
    }

    std::string ToString() const { return ""; }

    std::optional<Enum16> a_;
    std::optional<Enum16> b_;
};

class Packet_Optional_Struct_FieldView {
public:
    static Packet_Optional_Struct_FieldView Create(pdl::packet::slice const& parent) {
        return Packet_Optional_Struct_FieldView(parent);
    }

    std::optional<SizedStruct> GetA() const { _ASSERT_VALID(valid_); return a_; }

    std::optional<UnsizedStruct> GetB() const { _ASSERT_VALID(valid_); return b_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Optional_Struct_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            auto& opt_output = a_.emplace();
            if (!SizedStruct::Parse(span, &opt_output)) {
                return false;
            }
        }
        if (c1 == 1) {
            auto& opt_output = b_.emplace();
            if (!UnsizedStruct::Parse(span, &opt_output)) {
                return false;
            }
        }
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    std::optional<SizedStruct> a_;
    std::optional<UnsizedStruct> b_;


};

class Packet_Optional_Struct_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_Optional_Struct_FieldBuilder() override = default;
    Packet_Optional_Struct_FieldBuilder() = default;
    explicit Packet_Optional_Struct_FieldBuilder(std::optional<SizedStruct> a, std::optional<UnsizedStruct> b) : a_(std::move(a)), b_(std::move(b)) {}
    Packet_Optional_Struct_FieldBuilder(Packet_Optional_Struct_FieldBuilder const&) = default;
    Packet_Optional_Struct_FieldBuilder(Packet_Optional_Struct_FieldBuilder&&) = default;
    Packet_Optional_Struct_FieldBuilder& operator=(Packet_Optional_Struct_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>((a_.has_value() ? 0 : 1))) | (static_cast<uint8_t>((b_.has_value() ? 1 : 0)) << 1));
        if ((a_.has_value() ? 0 : 1) == 0) {
            a_->Serialize(output);
        }
        if ((b_.has_value() ? 1 : 0) == 1) {
            b_->Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + ((((a_.has_value() ? 0 : 1) == 0) ? a_->GetSize() : 0) + (((b_.has_value() ? 1 : 0) == 1) ? b_->GetSize() : 0));
    }

    std::string ToString() const { return ""; }

    std::optional<SizedStruct> a_;
    std::optional<UnsizedStruct> b_;
};

class ScalarChild_AView {
public:
    static ScalarChild_AView Create(ScalarParentView const& parent) {
        return ScalarChild_AView(parent);
    }

    uint8_t GetA() const { return 0; }

    uint8_t GetB() const { _ASSERT_VALID(valid_); return b_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ScalarChild_AView(ScalarParentView const& parent)
          : bytes_(parent.bytes_) {
        valid_ = Parse(parent);
    }

    bool Parse(ScalarParentView const& parent) {
        // Check validity of parent packet.
        if (!parent.IsValid()) { return false; }
        // Parse packet field values.
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 1) {
            return false;
        }
        b_ = span.read_le<uint8_t, 1>();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t payload_size_ {0};
    uint8_t b_;


};

class ScalarChild_ABuilder : public pdl::packet::Builder {
public:
    ~ScalarChild_ABuilder() override = default;
    ScalarChild_ABuilder() = default;
    explicit ScalarChild_ABuilder(uint8_t b) : b_(std::move(b)) {}
    ScalarChild_ABuilder(ScalarChild_ABuilder const&) = default;
    ScalarChild_ABuilder(ScalarChild_ABuilder&&) = default;
    ScalarChild_ABuilder& operator=(ScalarChild_ABuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(0x0 & 0xff)));
        size_t payload_size = 1;
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(b_ & 0xff)));
    }

    size_t GetSize() const override {
        return 3;
    }

    std::string ToString() const { return ""; }

    uint8_t payload_size_ {0};
    uint8_t b_{0};
};

class ScalarChild_BView {
public:
    static ScalarChild_BView Create(ScalarParentView const& parent) {
        return ScalarChild_BView(parent);
    }

    uint8_t GetA() const { return 1; }

    uint16_t GetC() const { _ASSERT_VALID(valid_); return c_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ScalarChild_BView(ScalarParentView const& parent)
          : bytes_(parent.bytes_) {
        valid_ = Parse(parent);
    }

    bool Parse(ScalarParentView const& parent) {
        // Check validity of parent packet.
        if (!parent.IsValid()) { return false; }
        // Parse packet field values.
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 2) {
            return false;
        }
        c_ = span.read_le<uint16_t, 2>();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t payload_size_ {0};
    uint16_t c_;


};

class ScalarChild_BBuilder : public pdl::packet::Builder {
public:
    ~ScalarChild_BBuilder() override = default;
    ScalarChild_BBuilder() = default;
    explicit ScalarChild_BBuilder(uint16_t c) : c_(std::move(c)) {}
    ScalarChild_BBuilder(ScalarChild_BBuilder const&) = default;
    ScalarChild_BBuilder(ScalarChild_BBuilder&&) = default;
    ScalarChild_BBuilder& operator=(ScalarChild_BBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(0x1 & 0xff)));
        size_t payload_size = 2;
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(c_ & 0xffff)));
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    uint8_t payload_size_ {0};
    uint16_t c_{0};
};

class EnumChild_AView {
public:
    static EnumChild_AView Create(EnumParentView const& parent) {
        return EnumChild_AView(parent);
    }

    Enum16 GetA() const { return Enum16::A; }

    uint8_t GetB() const { _ASSERT_VALID(valid_); return b_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit EnumChild_AView(EnumParentView const& parent)
          : bytes_(parent.bytes_) {
        valid_ = Parse(parent);
    }

    bool Parse(EnumParentView const& parent) {
        // Check validity of parent packet.
        if (!parent.IsValid()) { return false; }
        // Parse packet field values.
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 1) {
            return false;
        }
        b_ = span.read_le<uint8_t, 1>();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t payload_size_ {0};
    uint8_t b_;


};

class EnumChild_ABuilder : public pdl::packet::Builder {
public:
    ~EnumChild_ABuilder() override = default;
    EnumChild_ABuilder() = default;
    explicit EnumChild_ABuilder(uint8_t b) : b_(std::move(b)) {}
    EnumChild_ABuilder(EnumChild_ABuilder const&) = default;
    EnumChild_ABuilder(EnumChild_ABuilder&&) = default;
    EnumChild_ABuilder& operator=(EnumChild_ABuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(static_cast<uint16_t>(Enum16::A))));
        size_t payload_size = 1;
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(b_ & 0xff)));
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    uint8_t payload_size_ {0};
    uint8_t b_{0};
};

class EnumChild_BView {
public:
    static EnumChild_BView Create(EnumParentView const& parent) {
        return EnumChild_BView(parent);
    }

    Enum16 GetA() const { return Enum16::B; }

    uint16_t GetC() const { _ASSERT_VALID(valid_); return c_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit EnumChild_BView(EnumParentView const& parent)
          : bytes_(parent.bytes_) {
        valid_ = Parse(parent);
    }

    bool Parse(EnumParentView const& parent) {
        // Check validity of parent packet.
        if (!parent.IsValid()) { return false; }
        // Parse packet field values.
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 2) {
            return false;
        }
        c_ = span.read_le<uint16_t, 2>();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t payload_size_ {0};
    uint16_t c_;


};

class EnumChild_BBuilder : public pdl::packet::Builder {
public:
    ~EnumChild_BBuilder() override = default;
    EnumChild_BBuilder() = default;
    explicit EnumChild_BBuilder(uint16_t c) : c_(std::move(c)) {}
    EnumChild_BBuilder(EnumChild_BBuilder const&) = default;
    EnumChild_BBuilder(EnumChild_BBuilder&&) = default;
    EnumChild_BBuilder& operator=(EnumChild_BBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(static_cast<uint16_t>(Enum16::B))));
        size_t payload_size = 2;
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(c_ & 0xffff)));
    }

    size_t GetSize() const override {
        return 5;
    }

    std::string ToString() const { return ""; }

    uint8_t payload_size_ {0};
    uint16_t c_{0};
};

class AliasedChild_AView {
public:
    static AliasedChild_AView Create(EmptyParentView const& parent) {
        return AliasedChild_AView(parent);
    }

    uint8_t GetA() const { return 2; }

    uint8_t GetB() const { _ASSERT_VALID(valid_); return b_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit AliasedChild_AView(EmptyParentView const& parent)
          : bytes_(parent.bytes_) {
        valid_ = Parse(parent);
    }

    bool Parse(EmptyParentView const& parent) {
        // Check validity of parent packet.
        if (!parent.IsValid()) { return false; }
        // Parse packet field values.
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 1) {
            return false;
        }
        b_ = span.read_le<uint8_t, 1>();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t payload_size_ {0};
    uint8_t b_;


};

class AliasedChild_ABuilder : public pdl::packet::Builder {
public:
    ~AliasedChild_ABuilder() override = default;
    AliasedChild_ABuilder() = default;
    explicit AliasedChild_ABuilder(uint8_t b) : b_(std::move(b)) {}
    AliasedChild_ABuilder(AliasedChild_ABuilder const&) = default;
    AliasedChild_ABuilder(AliasedChild_ABuilder&&) = default;
    AliasedChild_ABuilder& operator=(AliasedChild_ABuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(0x2 & 0xff)));
        size_t payload_size = 1;
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(b_ & 0xff)));
    }

    size_t GetSize() const override {
        return 3;
    }

    std::string ToString() const { return ""; }

    uint8_t payload_size_ {0};
    uint8_t b_{0};
};

class AliasedChild_BView {
public:
    static AliasedChild_BView Create(EmptyParentView const& parent) {
        return AliasedChild_BView(parent);
    }

    uint8_t GetA() const { return 3; }

    uint16_t GetC() const { _ASSERT_VALID(valid_); return c_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit AliasedChild_BView(EmptyParentView const& parent)
          : bytes_(parent.bytes_) {
        valid_ = Parse(parent);
    }

    bool Parse(EmptyParentView const& parent) {
        // Check validity of parent packet.
        if (!parent.IsValid()) { return false; }
        // Parse packet field values.
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 2) {
            return false;
        }
        c_ = span.read_le<uint16_t, 2>();
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    uint8_t payload_size_ {0};
    uint16_t c_;


};

class AliasedChild_BBuilder : public pdl::packet::Builder {
public:
    ~AliasedChild_BBuilder() override = default;
    AliasedChild_BBuilder() = default;
    explicit AliasedChild_BBuilder(uint16_t c) : c_(std::move(c)) {}
    AliasedChild_BBuilder(AliasedChild_BBuilder const&) = default;
    AliasedChild_BBuilder(AliasedChild_BBuilder&&) = default;
    AliasedChild_BBuilder& operator=(AliasedChild_BBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(0x3 & 0xff)));
        size_t payload_size = 2;
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(payload_size)));
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(c_ & 0xffff)));
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    uint8_t payload_size_ {0};
    uint16_t c_{0};
};

class Struct_Scalar_Field : public pdl::packet::Builder {
public:
    ~Struct_Scalar_Field() override = default;
    Struct_Scalar_Field() = default;
    explicit Struct_Scalar_Field(uint8_t a, uint64_t c) : a_(std::move(a)), c_(std::move(c)) {}
    Struct_Scalar_Field(Struct_Scalar_Field const&) = default;
    Struct_Scalar_Field(Struct_Scalar_Field&&) = default;
    Struct_Scalar_Field& operator=(Struct_Scalar_Field const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Scalar_Field* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        output->a_ = (chunk0 >> 0) & 0x7f;
        output->c_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(a_ & 0x7f)) | (static_cast<uint64_t>(c_ & 0x1ffffffffffffff) << 7));
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint8_t a_{0};
    uint64_t c_{0};
};

class Struct_Enum_Field_ : public pdl::packet::Builder {
public:
    ~Struct_Enum_Field_() override = default;
    Struct_Enum_Field_() = default;
    explicit Struct_Enum_Field_(Enum7 a, uint64_t c) : a_(std::move(a)), c_(std::move(c)) {}
    Struct_Enum_Field_(Struct_Enum_Field_ const&) = default;
    Struct_Enum_Field_(Struct_Enum_Field_&&) = default;
    Struct_Enum_Field_& operator=(Struct_Enum_Field_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Enum_Field_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        output->a_ = Enum7((chunk0 >> 0) & 0x7f);
        output->c_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(static_cast<uint8_t>(a_))) | (static_cast<uint64_t>(c_ & 0x1ffffffffffffff) << 7));
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    Enum7 a_{Enum7::A};
    uint64_t c_{0};
};

class Struct_Enum_FieldView {
public:
    static Struct_Enum_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_Enum_FieldView(parent);
    }

    Struct_Enum_Field_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Enum_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Enum_Field_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Enum_Field_ s_;


};

class Struct_Enum_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_Enum_FieldBuilder() override = default;
    Struct_Enum_FieldBuilder() = default;
    explicit Struct_Enum_FieldBuilder(Struct_Enum_Field_ s) : s_(std::move(s)) {}
    Struct_Enum_FieldBuilder(Struct_Enum_FieldBuilder const&) = default;
    Struct_Enum_FieldBuilder(Struct_Enum_FieldBuilder&&) = default;
    Struct_Enum_FieldBuilder& operator=(Struct_Enum_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Enum_Field_ s_;
};

class Struct_Reserved_Field_ : public pdl::packet::Builder {
public:
    ~Struct_Reserved_Field_() override = default;
    Struct_Reserved_Field_() = default;
    explicit Struct_Reserved_Field_(uint8_t a, uint64_t c) : a_(std::move(a)), c_(std::move(c)) {}
    Struct_Reserved_Field_(Struct_Reserved_Field_ const&) = default;
    Struct_Reserved_Field_(Struct_Reserved_Field_&&) = default;
    Struct_Reserved_Field_& operator=(Struct_Reserved_Field_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Reserved_Field_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        output->a_ = (chunk0 >> 0) & 0x7f;
        output->c_ = (chunk0 >> 9) & 0x7fffffffffffff;
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(a_ & 0x7f)) | (static_cast<uint64_t>(c_ & 0x7fffffffffffff) << 9));
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint8_t a_{0};
    uint64_t c_{0};
};

class Struct_Reserved_FieldView {
public:
    static Struct_Reserved_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_Reserved_FieldView(parent);
    }

    Struct_Reserved_Field_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Reserved_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Reserved_Field_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Reserved_Field_ s_;


};

class Struct_Reserved_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_Reserved_FieldBuilder() override = default;
    Struct_Reserved_FieldBuilder() = default;
    explicit Struct_Reserved_FieldBuilder(Struct_Reserved_Field_ s) : s_(std::move(s)) {}
    Struct_Reserved_FieldBuilder(Struct_Reserved_FieldBuilder const&) = default;
    Struct_Reserved_FieldBuilder(Struct_Reserved_FieldBuilder&&) = default;
    Struct_Reserved_FieldBuilder& operator=(Struct_Reserved_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Reserved_Field_ s_;
};

class Struct_Size_Field_ : public pdl::packet::Builder {
public:
    ~Struct_Size_Field_() override = default;
    Struct_Size_Field_() = default;
    explicit Struct_Size_Field_(uint64_t a, std::vector<uint8_t> b) : a_(std::move(a)), b_(std::move(b)) {}
    Struct_Size_Field_(Struct_Size_Field_ const&) = default;
    Struct_Size_Field_(Struct_Size_Field_&&) = default;
    Struct_Size_Field_& operator=(Struct_Size_Field_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Size_Field_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        output->b_size_ = (chunk0 >> 0) & 0x7;
        output->a_ = (chunk0 >> 3) & 0x1fffffffffffffff;
        size_t limit = (span.size() > output->b_size_) ? (span.size() - output->b_size_) : 0;
        while (span.size() > limit) {
            if (span.size() < 1) return false;
            output->b_.push_back(span.read_le<uint8_t, 1>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t b_size = (b_.size() * 1);
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(b_size)) | (static_cast<uint64_t>(a_ & 0x1fffffffffffffff) << 3));
        for (auto const& element : b_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return 8 + ((b_.size() * 1));
    }

    std::string ToString() const { return ""; }

    uint8_t b_size_ {0};
    uint64_t a_{0};
    std::vector<uint8_t> b_;
};

class Struct_Size_FieldView {
public:
    static Struct_Size_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_Size_FieldView(parent);
    }

    Struct_Size_Field_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Size_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Size_Field_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Size_Field_ s_;


};

class Struct_Size_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_Size_FieldBuilder() override = default;
    Struct_Size_FieldBuilder() = default;
    explicit Struct_Size_FieldBuilder(Struct_Size_Field_ s) : s_(std::move(s)) {}
    Struct_Size_FieldBuilder(Struct_Size_FieldBuilder const&) = default;
    Struct_Size_FieldBuilder(Struct_Size_FieldBuilder&&) = default;
    Struct_Size_FieldBuilder& operator=(Struct_Size_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Size_Field_ s_;
};

class Struct_Count_Field_ : public pdl::packet::Builder {
public:
    ~Struct_Count_Field_() override = default;
    Struct_Count_Field_() = default;
    explicit Struct_Count_Field_(uint64_t a, std::vector<uint8_t> b) : a_(std::move(a)), b_(std::move(b)) {}
    Struct_Count_Field_(Struct_Count_Field_ const&) = default;
    Struct_Count_Field_(Struct_Count_Field_&&) = default;
    Struct_Count_Field_& operator=(Struct_Count_Field_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Count_Field_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        output->b_count_ = (chunk0 >> 0) & 0x7;
        output->a_ = (chunk0 >> 3) & 0x1fffffffffffffff;
        for (size_t n = 0; n < output->b_count_; n++) {
            if (span.size() < 1) return false;
            output->b_.push_back(span.read_le<uint8_t, 1>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(b_.size())) | (static_cast<uint64_t>(a_ & 0x1fffffffffffffff) << 3));
        for (auto const& element : b_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return 8 + ((b_.size() * 1));
    }

    std::string ToString() const { return ""; }

    uint8_t b_count_ {0};
    uint64_t a_{0};
    std::vector<uint8_t> b_;
};

class Struct_Count_FieldView {
public:
    static Struct_Count_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_Count_FieldView(parent);
    }

    Struct_Count_Field_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Count_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Count_Field_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Count_Field_ s_;


};

class Struct_Count_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_Count_FieldBuilder() override = default;
    Struct_Count_FieldBuilder() = default;
    explicit Struct_Count_FieldBuilder(Struct_Count_Field_ s) : s_(std::move(s)) {}
    Struct_Count_FieldBuilder(Struct_Count_FieldBuilder const&) = default;
    Struct_Count_FieldBuilder(Struct_Count_FieldBuilder&&) = default;
    Struct_Count_FieldBuilder& operator=(Struct_Count_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Count_Field_ s_;
};

class Struct_FixedScalar_Field_ : public pdl::packet::Builder {
public:
    ~Struct_FixedScalar_Field_() override = default;
    Struct_FixedScalar_Field_() = default;
    explicit Struct_FixedScalar_Field_(uint64_t b) : b_(std::move(b)) {}
    Struct_FixedScalar_Field_(Struct_FixedScalar_Field_ const&) = default;
    Struct_FixedScalar_Field_(Struct_FixedScalar_Field_&&) = default;
    Struct_FixedScalar_Field_& operator=(Struct_FixedScalar_Field_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_FixedScalar_Field_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        if (static_cast<uint64_t>((chunk0 >> 0) & 0x7f) != 0x7) {
            return false;
        }
        output->b_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(0x7)) | (static_cast<uint64_t>(b_ & 0x1ffffffffffffff) << 7));
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint64_t b_{0};
};

class Struct_FixedScalar_FieldView {
public:
    static Struct_FixedScalar_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_FixedScalar_FieldView(parent);
    }

    Struct_FixedScalar_Field_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_FixedScalar_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_FixedScalar_Field_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_FixedScalar_Field_ s_;


};

class Struct_FixedScalar_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_FixedScalar_FieldBuilder() override = default;
    Struct_FixedScalar_FieldBuilder() = default;
    explicit Struct_FixedScalar_FieldBuilder(Struct_FixedScalar_Field_ s) : s_(std::move(s)) {}
    Struct_FixedScalar_FieldBuilder(Struct_FixedScalar_FieldBuilder const&) = default;
    Struct_FixedScalar_FieldBuilder(Struct_FixedScalar_FieldBuilder&&) = default;
    Struct_FixedScalar_FieldBuilder& operator=(Struct_FixedScalar_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_FixedScalar_Field_ s_;
};

class Struct_FixedEnum_Field_ : public pdl::packet::Builder {
public:
    ~Struct_FixedEnum_Field_() override = default;
    Struct_FixedEnum_Field_() = default;
    explicit Struct_FixedEnum_Field_(uint64_t b) : b_(std::move(b)) {}
    Struct_FixedEnum_Field_(Struct_FixedEnum_Field_ const&) = default;
    Struct_FixedEnum_Field_(Struct_FixedEnum_Field_&&) = default;
    Struct_FixedEnum_Field_& operator=(Struct_FixedEnum_Field_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_FixedEnum_Field_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 8) {
            return false;
        }
        uint64_t chunk0 = span.read_le<uint64_t, 8>();
        if (Enum7((chunk0 >> 0) & 0x7f) != Enum7::A) {
            return false;
        }
        output->b_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint64_t, 8>(output, (static_cast<uint64_t>(Enum7::A)) | (static_cast<uint64_t>(b_ & 0x1ffffffffffffff) << 7));
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint64_t b_{0};
};

class Struct_FixedEnum_FieldView {
public:
    static Struct_FixedEnum_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_FixedEnum_FieldView(parent);
    }

    Struct_FixedEnum_Field_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_FixedEnum_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_FixedEnum_Field_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_FixedEnum_Field_ s_;


};

class Struct_FixedEnum_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_FixedEnum_FieldBuilder() override = default;
    Struct_FixedEnum_FieldBuilder() = default;
    explicit Struct_FixedEnum_FieldBuilder(Struct_FixedEnum_Field_ s) : s_(std::move(s)) {}
    Struct_FixedEnum_FieldBuilder(Struct_FixedEnum_FieldBuilder const&) = default;
    Struct_FixedEnum_FieldBuilder(Struct_FixedEnum_FieldBuilder&&) = default;
    Struct_FixedEnum_FieldBuilder& operator=(Struct_FixedEnum_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_FixedEnum_Field_ s_;
};

class Struct_ScalarGroup_Field_ : public pdl::packet::Builder {
public:
    ~Struct_ScalarGroup_Field_() override = default;
    Struct_ScalarGroup_Field_() = default;
    Struct_ScalarGroup_Field_(Struct_ScalarGroup_Field_ const&) = default;
    Struct_ScalarGroup_Field_(Struct_ScalarGroup_Field_&&) = default;
    Struct_ScalarGroup_Field_& operator=(Struct_ScalarGroup_Field_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_ScalarGroup_Field_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 2) {
            return false;
        }
        if (static_cast<uint64_t>(span.read_le<uint16_t, 2>()) != 0x2a) {
            return false;
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(0x2a)));
    }

    size_t GetSize() const override {
        return 2;
    }

    std::string ToString() const { return ""; }


};

class Struct_ScalarGroup_FieldView {
public:
    static Struct_ScalarGroup_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_ScalarGroup_FieldView(parent);
    }

    Struct_ScalarGroup_Field_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_ScalarGroup_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_ScalarGroup_Field_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_ScalarGroup_Field_ s_;


};

class Struct_ScalarGroup_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_ScalarGroup_FieldBuilder() override = default;
    Struct_ScalarGroup_FieldBuilder() = default;
    explicit Struct_ScalarGroup_FieldBuilder(Struct_ScalarGroup_Field_ s) : s_(std::move(s)) {}
    Struct_ScalarGroup_FieldBuilder(Struct_ScalarGroup_FieldBuilder const&) = default;
    Struct_ScalarGroup_FieldBuilder(Struct_ScalarGroup_FieldBuilder&&) = default;
    Struct_ScalarGroup_FieldBuilder& operator=(Struct_ScalarGroup_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_ScalarGroup_Field_ s_;
};

class Struct_EnumGroup_Field_ : public pdl::packet::Builder {
public:
    ~Struct_EnumGroup_Field_() override = default;
    Struct_EnumGroup_Field_() = default;
    Struct_EnumGroup_Field_(Struct_EnumGroup_Field_ const&) = default;
    Struct_EnumGroup_Field_(Struct_EnumGroup_Field_&&) = default;
    Struct_EnumGroup_Field_& operator=(Struct_EnumGroup_Field_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_EnumGroup_Field_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 2) {
            return false;
        }
        if (Enum16(span.read_le<uint16_t, 2>()) != Enum16::A) {
            return false;
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint16_t, 2>(output, (static_cast<uint16_t>(Enum16::A)));
    }

    size_t GetSize() const override {
        return 2;
    }

    std::string ToString() const { return ""; }


};

class Struct_EnumGroup_FieldView {
public:
    static Struct_EnumGroup_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_EnumGroup_FieldView(parent);
    }

    Struct_EnumGroup_Field_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_EnumGroup_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_EnumGroup_Field_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_EnumGroup_Field_ s_;


};

class Struct_EnumGroup_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_EnumGroup_FieldBuilder() override = default;
    Struct_EnumGroup_FieldBuilder() = default;
    explicit Struct_EnumGroup_FieldBuilder(Struct_EnumGroup_Field_ s) : s_(std::move(s)) {}
    Struct_EnumGroup_FieldBuilder(Struct_EnumGroup_FieldBuilder const&) = default;
    Struct_EnumGroup_FieldBuilder(Struct_EnumGroup_FieldBuilder&&) = default;
    Struct_EnumGroup_FieldBuilder& operator=(Struct_EnumGroup_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_EnumGroup_Field_ s_;
};

class Struct_Struct_FieldView {
public:
    static Struct_Struct_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_Struct_FieldView(parent);
    }

    SizedStruct const& GetA() const { _ASSERT_VALID(valid_); return a_; }

    UnsizedStruct const& GetB() const { _ASSERT_VALID(valid_); return b_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Struct_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!SizedStruct::Parse(span, &a_)) return false;
        if (!UnsizedStruct::Parse(span, &b_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    SizedStruct a_;
    UnsizedStruct b_;


};

class Struct_Struct_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_Struct_FieldBuilder() override = default;
    Struct_Struct_FieldBuilder() = default;
    explicit Struct_Struct_FieldBuilder(SizedStruct a, UnsizedStruct b) : a_(std::move(a)), b_(std::move(b)) {}
    Struct_Struct_FieldBuilder(Struct_Struct_FieldBuilder const&) = default;
    Struct_Struct_FieldBuilder(Struct_Struct_FieldBuilder&&) = default;
    Struct_Struct_FieldBuilder& operator=(Struct_Struct_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        a_.Serialize(output);
        b_.Serialize(output);
    }

    size_t GetSize() const override {
        return a_.GetSize() + b_.GetSize();
    }

    std::string ToString() const { return ""; }

    SizedStruct a_;
    UnsizedStruct b_;
};

class Struct_Array_Field_ByteElement_ConstantSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ByteElement_ConstantSize_() override = default;
    Struct_Array_Field_ByteElement_ConstantSize_() = default;
    explicit Struct_Array_Field_ByteElement_ConstantSize_(std::array<uint8_t, 4> array) : array_(std::move(array)) {}
    Struct_Array_Field_ByteElement_ConstantSize_(Struct_Array_Field_ByteElement_ConstantSize_ const&) = default;
    Struct_Array_Field_ByteElement_ConstantSize_(Struct_Array_Field_ByteElement_ConstantSize_&&) = default;
    Struct_Array_Field_ByteElement_ConstantSize_& operator=(Struct_Array_Field_ByteElement_ConstantSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_ByteElement_ConstantSize_* output) {
        pdl::packet::slice span = parent_span;
        for (int n = 0; n < 4; n++) {
            if (span.size() < 1) return false;
            output->array_[n] = span.read_le<uint8_t, 1>();
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 1);
    }

    std::string ToString() const { return ""; }

    std::array<uint8_t, 4> array_;
};

class Struct_Array_Field_ByteElement_ConstantSizeView {
public:
    static Struct_Array_Field_ByteElement_ConstantSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_ByteElement_ConstantSizeView(parent);
    }

    Struct_Array_Field_ByteElement_ConstantSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ByteElement_ConstantSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_ByteElement_ConstantSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_ByteElement_ConstantSize_ s_;


};

class Struct_Array_Field_ByteElement_ConstantSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ByteElement_ConstantSizeBuilder() override = default;
    Struct_Array_Field_ByteElement_ConstantSizeBuilder() = default;
    explicit Struct_Array_Field_ByteElement_ConstantSizeBuilder(Struct_Array_Field_ByteElement_ConstantSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_ByteElement_ConstantSizeBuilder(Struct_Array_Field_ByteElement_ConstantSizeBuilder const&) = default;
    Struct_Array_Field_ByteElement_ConstantSizeBuilder(Struct_Array_Field_ByteElement_ConstantSizeBuilder&&) = default;
    Struct_Array_Field_ByteElement_ConstantSizeBuilder& operator=(Struct_Array_Field_ByteElement_ConstantSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_ByteElement_ConstantSize_ s_;
};

class Struct_Array_Field_ByteElement_VariableSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ByteElement_VariableSize_() override = default;
    Struct_Array_Field_ByteElement_VariableSize_() = default;
    explicit Struct_Array_Field_ByteElement_VariableSize_(std::vector<uint8_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ByteElement_VariableSize_(Struct_Array_Field_ByteElement_VariableSize_ const&) = default;
    Struct_Array_Field_ByteElement_VariableSize_(Struct_Array_Field_ByteElement_VariableSize_&&) = default;
    Struct_Array_Field_ByteElement_VariableSize_& operator=(Struct_Array_Field_ByteElement_VariableSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_ByteElement_VariableSize_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_size_ = (chunk0 >> 0) & 0xf;
        size_t limit = (span.size() > output->array_size_) ? (span.size() - output->array_size_) : 0;
        while (span.size() > limit) {
            if (span.size() < 1) return false;
            output->array_.push_back(span.read_le<uint8_t, 1>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = (array_.size() * 1);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 1));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<uint8_t> array_;
};

class Struct_Array_Field_ByteElement_VariableSizeView {
public:
    static Struct_Array_Field_ByteElement_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_ByteElement_VariableSizeView(parent);
    }

    Struct_Array_Field_ByteElement_VariableSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ByteElement_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_ByteElement_VariableSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_ByteElement_VariableSize_ s_;


};

class Struct_Array_Field_ByteElement_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ByteElement_VariableSizeBuilder() override = default;
    Struct_Array_Field_ByteElement_VariableSizeBuilder() = default;
    explicit Struct_Array_Field_ByteElement_VariableSizeBuilder(Struct_Array_Field_ByteElement_VariableSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_ByteElement_VariableSizeBuilder(Struct_Array_Field_ByteElement_VariableSizeBuilder const&) = default;
    Struct_Array_Field_ByteElement_VariableSizeBuilder(Struct_Array_Field_ByteElement_VariableSizeBuilder&&) = default;
    Struct_Array_Field_ByteElement_VariableSizeBuilder& operator=(Struct_Array_Field_ByteElement_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_ByteElement_VariableSize_ s_;
};

class Struct_Array_Field_ByteElement_VariableCount_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ByteElement_VariableCount_() override = default;
    Struct_Array_Field_ByteElement_VariableCount_() = default;
    explicit Struct_Array_Field_ByteElement_VariableCount_(std::vector<uint8_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ByteElement_VariableCount_(Struct_Array_Field_ByteElement_VariableCount_ const&) = default;
    Struct_Array_Field_ByteElement_VariableCount_(Struct_Array_Field_ByteElement_VariableCount_&&) = default;
    Struct_Array_Field_ByteElement_VariableCount_& operator=(Struct_Array_Field_ByteElement_VariableCount_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_ByteElement_VariableCount_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_count_ = (chunk0 >> 0) & 0xf;
        for (size_t n = 0; n < output->array_count_; n++) {
            if (span.size() < 1) return false;
            output->array_.push_back(span.read_le<uint8_t, 1>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 1));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<uint8_t> array_;
};

class Struct_Array_Field_ByteElement_VariableCountView {
public:
    static Struct_Array_Field_ByteElement_VariableCountView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_ByteElement_VariableCountView(parent);
    }

    Struct_Array_Field_ByteElement_VariableCount_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ByteElement_VariableCountView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_ByteElement_VariableCount_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_ByteElement_VariableCount_ s_;


};

class Struct_Array_Field_ByteElement_VariableCountBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ByteElement_VariableCountBuilder() override = default;
    Struct_Array_Field_ByteElement_VariableCountBuilder() = default;
    explicit Struct_Array_Field_ByteElement_VariableCountBuilder(Struct_Array_Field_ByteElement_VariableCount_ s) : s_(std::move(s)) {}
    Struct_Array_Field_ByteElement_VariableCountBuilder(Struct_Array_Field_ByteElement_VariableCountBuilder const&) = default;
    Struct_Array_Field_ByteElement_VariableCountBuilder(Struct_Array_Field_ByteElement_VariableCountBuilder&&) = default;
    Struct_Array_Field_ByteElement_VariableCountBuilder& operator=(Struct_Array_Field_ByteElement_VariableCountBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_ByteElement_VariableCount_ s_;
};

class Struct_Array_Field_ByteElement_UnknownSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ByteElement_UnknownSize_() override = default;
    Struct_Array_Field_ByteElement_UnknownSize_() = default;
    explicit Struct_Array_Field_ByteElement_UnknownSize_(std::vector<uint8_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ByteElement_UnknownSize_(Struct_Array_Field_ByteElement_UnknownSize_ const&) = default;
    Struct_Array_Field_ByteElement_UnknownSize_(Struct_Array_Field_ByteElement_UnknownSize_&&) = default;
    Struct_Array_Field_ByteElement_UnknownSize_& operator=(Struct_Array_Field_ByteElement_UnknownSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_ByteElement_UnknownSize_* output) {
        pdl::packet::slice span = parent_span;
        while (span.size() > 0) {
            if (span.size() < 1) return false;
            output->array_.push_back(span.read_le<uint8_t, 1>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint8_t, 1>(output, static_cast<uint8_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 1);
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> array_;
};

class Struct_Array_Field_ByteElement_UnknownSizeView {
public:
    static Struct_Array_Field_ByteElement_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_ByteElement_UnknownSizeView(parent);
    }

    Struct_Array_Field_ByteElement_UnknownSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ByteElement_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_ByteElement_UnknownSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_ByteElement_UnknownSize_ s_;


};

class Struct_Array_Field_ByteElement_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ByteElement_UnknownSizeBuilder() override = default;
    Struct_Array_Field_ByteElement_UnknownSizeBuilder() = default;
    explicit Struct_Array_Field_ByteElement_UnknownSizeBuilder(Struct_Array_Field_ByteElement_UnknownSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_ByteElement_UnknownSizeBuilder(Struct_Array_Field_ByteElement_UnknownSizeBuilder const&) = default;
    Struct_Array_Field_ByteElement_UnknownSizeBuilder(Struct_Array_Field_ByteElement_UnknownSizeBuilder&&) = default;
    Struct_Array_Field_ByteElement_UnknownSizeBuilder& operator=(Struct_Array_Field_ByteElement_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_ByteElement_UnknownSize_ s_;
};

class Struct_Array_Field_ScalarElement_ConstantSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ScalarElement_ConstantSize_() override = default;
    Struct_Array_Field_ScalarElement_ConstantSize_() = default;
    explicit Struct_Array_Field_ScalarElement_ConstantSize_(std::array<uint16_t, 4> array) : array_(std::move(array)) {}
    Struct_Array_Field_ScalarElement_ConstantSize_(Struct_Array_Field_ScalarElement_ConstantSize_ const&) = default;
    Struct_Array_Field_ScalarElement_ConstantSize_(Struct_Array_Field_ScalarElement_ConstantSize_&&) = default;
    Struct_Array_Field_ScalarElement_ConstantSize_& operator=(Struct_Array_Field_ScalarElement_ConstantSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_ScalarElement_ConstantSize_* output) {
        pdl::packet::slice span = parent_span;
        for (int n = 0; n < 4; n++) {
            if (span.size() < 2) return false;
            output->array_[n] = span.read_le<uint16_t, 2>();
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 2);
    }

    std::string ToString() const { return ""; }

    std::array<uint16_t, 4> array_;
};

class Struct_Array_Field_ScalarElement_ConstantSizeView {
public:
    static Struct_Array_Field_ScalarElement_ConstantSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_ScalarElement_ConstantSizeView(parent);
    }

    Struct_Array_Field_ScalarElement_ConstantSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ScalarElement_ConstantSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_ScalarElement_ConstantSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_ScalarElement_ConstantSize_ s_;


};

class Struct_Array_Field_ScalarElement_ConstantSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ScalarElement_ConstantSizeBuilder() override = default;
    Struct_Array_Field_ScalarElement_ConstantSizeBuilder() = default;
    explicit Struct_Array_Field_ScalarElement_ConstantSizeBuilder(Struct_Array_Field_ScalarElement_ConstantSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_ScalarElement_ConstantSizeBuilder(Struct_Array_Field_ScalarElement_ConstantSizeBuilder const&) = default;
    Struct_Array_Field_ScalarElement_ConstantSizeBuilder(Struct_Array_Field_ScalarElement_ConstantSizeBuilder&&) = default;
    Struct_Array_Field_ScalarElement_ConstantSizeBuilder& operator=(Struct_Array_Field_ScalarElement_ConstantSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_ScalarElement_ConstantSize_ s_;
};

class Struct_Array_Field_ScalarElement_VariableSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ScalarElement_VariableSize_() override = default;
    Struct_Array_Field_ScalarElement_VariableSize_() = default;
    explicit Struct_Array_Field_ScalarElement_VariableSize_(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ScalarElement_VariableSize_(Struct_Array_Field_ScalarElement_VariableSize_ const&) = default;
    Struct_Array_Field_ScalarElement_VariableSize_(Struct_Array_Field_ScalarElement_VariableSize_&&) = default;
    Struct_Array_Field_ScalarElement_VariableSize_& operator=(Struct_Array_Field_ScalarElement_VariableSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_ScalarElement_VariableSize_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_size_ = (chunk0 >> 0) & 0xf;
        size_t limit = (span.size() > output->array_size_) ? (span.size() - output->array_size_) : 0;
        while (span.size() > limit) {
            if (span.size() < 2) return false;
            output->array_.push_back(span.read_le<uint16_t, 2>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = (array_.size() * 2);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 2));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<uint16_t> array_;
};

class Struct_Array_Field_ScalarElement_VariableSizeView {
public:
    static Struct_Array_Field_ScalarElement_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_ScalarElement_VariableSizeView(parent);
    }

    Struct_Array_Field_ScalarElement_VariableSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ScalarElement_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_ScalarElement_VariableSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_ScalarElement_VariableSize_ s_;


};

class Struct_Array_Field_ScalarElement_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ScalarElement_VariableSizeBuilder() override = default;
    Struct_Array_Field_ScalarElement_VariableSizeBuilder() = default;
    explicit Struct_Array_Field_ScalarElement_VariableSizeBuilder(Struct_Array_Field_ScalarElement_VariableSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_ScalarElement_VariableSizeBuilder(Struct_Array_Field_ScalarElement_VariableSizeBuilder const&) = default;
    Struct_Array_Field_ScalarElement_VariableSizeBuilder(Struct_Array_Field_ScalarElement_VariableSizeBuilder&&) = default;
    Struct_Array_Field_ScalarElement_VariableSizeBuilder& operator=(Struct_Array_Field_ScalarElement_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_ScalarElement_VariableSize_ s_;
};

class Struct_Array_Field_ScalarElement_VariableCount_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ScalarElement_VariableCount_() override = default;
    Struct_Array_Field_ScalarElement_VariableCount_() = default;
    explicit Struct_Array_Field_ScalarElement_VariableCount_(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ScalarElement_VariableCount_(Struct_Array_Field_ScalarElement_VariableCount_ const&) = default;
    Struct_Array_Field_ScalarElement_VariableCount_(Struct_Array_Field_ScalarElement_VariableCount_&&) = default;
    Struct_Array_Field_ScalarElement_VariableCount_& operator=(Struct_Array_Field_ScalarElement_VariableCount_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_ScalarElement_VariableCount_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_count_ = (chunk0 >> 0) & 0xf;
        for (size_t n = 0; n < output->array_count_; n++) {
            if (span.size() < 2) return false;
            output->array_.push_back(span.read_le<uint16_t, 2>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 2));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<uint16_t> array_;
};

class Struct_Array_Field_ScalarElement_VariableCountView {
public:
    static Struct_Array_Field_ScalarElement_VariableCountView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_ScalarElement_VariableCountView(parent);
    }

    Struct_Array_Field_ScalarElement_VariableCount_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ScalarElement_VariableCountView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_ScalarElement_VariableCount_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_ScalarElement_VariableCount_ s_;


};

class Struct_Array_Field_ScalarElement_VariableCountBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ScalarElement_VariableCountBuilder() override = default;
    Struct_Array_Field_ScalarElement_VariableCountBuilder() = default;
    explicit Struct_Array_Field_ScalarElement_VariableCountBuilder(Struct_Array_Field_ScalarElement_VariableCount_ s) : s_(std::move(s)) {}
    Struct_Array_Field_ScalarElement_VariableCountBuilder(Struct_Array_Field_ScalarElement_VariableCountBuilder const&) = default;
    Struct_Array_Field_ScalarElement_VariableCountBuilder(Struct_Array_Field_ScalarElement_VariableCountBuilder&&) = default;
    Struct_Array_Field_ScalarElement_VariableCountBuilder& operator=(Struct_Array_Field_ScalarElement_VariableCountBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_ScalarElement_VariableCount_ s_;
};

class Struct_Array_Field_ScalarElement_UnknownSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ScalarElement_UnknownSize_() override = default;
    Struct_Array_Field_ScalarElement_UnknownSize_() = default;
    explicit Struct_Array_Field_ScalarElement_UnknownSize_(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ScalarElement_UnknownSize_(Struct_Array_Field_ScalarElement_UnknownSize_ const&) = default;
    Struct_Array_Field_ScalarElement_UnknownSize_(Struct_Array_Field_ScalarElement_UnknownSize_&&) = default;
    Struct_Array_Field_ScalarElement_UnknownSize_& operator=(Struct_Array_Field_ScalarElement_UnknownSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_ScalarElement_UnknownSize_* output) {
        pdl::packet::slice span = parent_span;
        while (span.size() > 0) {
            if (span.size() < 2) return false;
            output->array_.push_back(span.read_le<uint16_t, 2>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 2);
    }

    std::string ToString() const { return ""; }

    std::vector<uint16_t> array_;
};

class Struct_Array_Field_ScalarElement_UnknownSizeView {
public:
    static Struct_Array_Field_ScalarElement_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_ScalarElement_UnknownSizeView(parent);
    }

    Struct_Array_Field_ScalarElement_UnknownSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ScalarElement_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_ScalarElement_UnknownSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_ScalarElement_UnknownSize_ s_;


};

class Struct_Array_Field_ScalarElement_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ScalarElement_UnknownSizeBuilder() override = default;
    Struct_Array_Field_ScalarElement_UnknownSizeBuilder() = default;
    explicit Struct_Array_Field_ScalarElement_UnknownSizeBuilder(Struct_Array_Field_ScalarElement_UnknownSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_ScalarElement_UnknownSizeBuilder(Struct_Array_Field_ScalarElement_UnknownSizeBuilder const&) = default;
    Struct_Array_Field_ScalarElement_UnknownSizeBuilder(Struct_Array_Field_ScalarElement_UnknownSizeBuilder&&) = default;
    Struct_Array_Field_ScalarElement_UnknownSizeBuilder& operator=(Struct_Array_Field_ScalarElement_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_ScalarElement_UnknownSize_ s_;
};

class Struct_Array_Field_EnumElement_ConstantSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_EnumElement_ConstantSize_() override = default;
    Struct_Array_Field_EnumElement_ConstantSize_() = default;
    explicit Struct_Array_Field_EnumElement_ConstantSize_(std::array<Enum16, 4> array) : array_(std::move(array)) {}
    Struct_Array_Field_EnumElement_ConstantSize_(Struct_Array_Field_EnumElement_ConstantSize_ const&) = default;
    Struct_Array_Field_EnumElement_ConstantSize_(Struct_Array_Field_EnumElement_ConstantSize_&&) = default;
    Struct_Array_Field_EnumElement_ConstantSize_& operator=(Struct_Array_Field_EnumElement_ConstantSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_EnumElement_ConstantSize_* output) {
        pdl::packet::slice span = parent_span;
        for (int n = 0; n < 4; n++) {
            if (span.size() < 2) return false;
            output->array_[n] = Enum16(span.read_le<uint16_t, 2>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 2);
    }

    std::string ToString() const { return ""; }

    std::array<Enum16, 4> array_;
};

class Struct_Array_Field_EnumElement_ConstantSizeView {
public:
    static Struct_Array_Field_EnumElement_ConstantSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_EnumElement_ConstantSizeView(parent);
    }

    Struct_Array_Field_EnumElement_ConstantSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_EnumElement_ConstantSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_EnumElement_ConstantSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_EnumElement_ConstantSize_ s_;


};

class Struct_Array_Field_EnumElement_ConstantSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_EnumElement_ConstantSizeBuilder() override = default;
    Struct_Array_Field_EnumElement_ConstantSizeBuilder() = default;
    explicit Struct_Array_Field_EnumElement_ConstantSizeBuilder(Struct_Array_Field_EnumElement_ConstantSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_EnumElement_ConstantSizeBuilder(Struct_Array_Field_EnumElement_ConstantSizeBuilder const&) = default;
    Struct_Array_Field_EnumElement_ConstantSizeBuilder(Struct_Array_Field_EnumElement_ConstantSizeBuilder&&) = default;
    Struct_Array_Field_EnumElement_ConstantSizeBuilder& operator=(Struct_Array_Field_EnumElement_ConstantSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_EnumElement_ConstantSize_ s_;
};

class Struct_Array_Field_EnumElement_VariableSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_EnumElement_VariableSize_() override = default;
    Struct_Array_Field_EnumElement_VariableSize_() = default;
    explicit Struct_Array_Field_EnumElement_VariableSize_(std::vector<Enum16> array) : array_(std::move(array)) {}
    Struct_Array_Field_EnumElement_VariableSize_(Struct_Array_Field_EnumElement_VariableSize_ const&) = default;
    Struct_Array_Field_EnumElement_VariableSize_(Struct_Array_Field_EnumElement_VariableSize_&&) = default;
    Struct_Array_Field_EnumElement_VariableSize_& operator=(Struct_Array_Field_EnumElement_VariableSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_EnumElement_VariableSize_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_size_ = (chunk0 >> 0) & 0xf;
        size_t limit = (span.size() > output->array_size_) ? (span.size() - output->array_size_) : 0;
        while (span.size() > limit) {
            if (span.size() < 2) return false;
            output->array_.push_back(Enum16(span.read_le<uint16_t, 2>()));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = (array_.size() * 2);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 2));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<Enum16> array_;
};

class Struct_Array_Field_EnumElement_VariableSizeView {
public:
    static Struct_Array_Field_EnumElement_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_EnumElement_VariableSizeView(parent);
    }

    Struct_Array_Field_EnumElement_VariableSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_EnumElement_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_EnumElement_VariableSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_EnumElement_VariableSize_ s_;


};

class Struct_Array_Field_EnumElement_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_EnumElement_VariableSizeBuilder() override = default;
    Struct_Array_Field_EnumElement_VariableSizeBuilder() = default;
    explicit Struct_Array_Field_EnumElement_VariableSizeBuilder(Struct_Array_Field_EnumElement_VariableSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_EnumElement_VariableSizeBuilder(Struct_Array_Field_EnumElement_VariableSizeBuilder const&) = default;
    Struct_Array_Field_EnumElement_VariableSizeBuilder(Struct_Array_Field_EnumElement_VariableSizeBuilder&&) = default;
    Struct_Array_Field_EnumElement_VariableSizeBuilder& operator=(Struct_Array_Field_EnumElement_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_EnumElement_VariableSize_ s_;
};

class Struct_Array_Field_EnumElement_VariableCount_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_EnumElement_VariableCount_() override = default;
    Struct_Array_Field_EnumElement_VariableCount_() = default;
    explicit Struct_Array_Field_EnumElement_VariableCount_(std::vector<Enum16> array) : array_(std::move(array)) {}
    Struct_Array_Field_EnumElement_VariableCount_(Struct_Array_Field_EnumElement_VariableCount_ const&) = default;
    Struct_Array_Field_EnumElement_VariableCount_(Struct_Array_Field_EnumElement_VariableCount_&&) = default;
    Struct_Array_Field_EnumElement_VariableCount_& operator=(Struct_Array_Field_EnumElement_VariableCount_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_EnumElement_VariableCount_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_count_ = (chunk0 >> 0) & 0xf;
        for (size_t n = 0; n < output->array_count_; n++) {
            if (span.size() < 2) return false;
            output->array_.push_back(Enum16(span.read_le<uint16_t, 2>()));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return 1 + ((array_.size() * 2));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<Enum16> array_;
};

class Struct_Array_Field_EnumElement_VariableCountView {
public:
    static Struct_Array_Field_EnumElement_VariableCountView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_EnumElement_VariableCountView(parent);
    }

    Struct_Array_Field_EnumElement_VariableCount_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_EnumElement_VariableCountView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_EnumElement_VariableCount_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_EnumElement_VariableCount_ s_;


};

class Struct_Array_Field_EnumElement_VariableCountBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_EnumElement_VariableCountBuilder() override = default;
    Struct_Array_Field_EnumElement_VariableCountBuilder() = default;
    explicit Struct_Array_Field_EnumElement_VariableCountBuilder(Struct_Array_Field_EnumElement_VariableCount_ s) : s_(std::move(s)) {}
    Struct_Array_Field_EnumElement_VariableCountBuilder(Struct_Array_Field_EnumElement_VariableCountBuilder const&) = default;
    Struct_Array_Field_EnumElement_VariableCountBuilder(Struct_Array_Field_EnumElement_VariableCountBuilder&&) = default;
    Struct_Array_Field_EnumElement_VariableCountBuilder& operator=(Struct_Array_Field_EnumElement_VariableCountBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_EnumElement_VariableCount_ s_;
};

class Struct_Array_Field_EnumElement_UnknownSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_EnumElement_UnknownSize_() override = default;
    Struct_Array_Field_EnumElement_UnknownSize_() = default;
    explicit Struct_Array_Field_EnumElement_UnknownSize_(std::vector<Enum16> array) : array_(std::move(array)) {}
    Struct_Array_Field_EnumElement_UnknownSize_(Struct_Array_Field_EnumElement_UnknownSize_ const&) = default;
    Struct_Array_Field_EnumElement_UnknownSize_(Struct_Array_Field_EnumElement_UnknownSize_&&) = default;
    Struct_Array_Field_EnumElement_UnknownSize_& operator=(Struct_Array_Field_EnumElement_UnknownSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_EnumElement_UnknownSize_* output) {
        pdl::packet::slice span = parent_span;
        while (span.size() > 0) {
            if (span.size() < 2) return false;
            output->array_.push_back(Enum16(span.read_le<uint16_t, 2>()));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
    }

    size_t GetSize() const override {
        return (array_.size() * 2);
    }

    std::string ToString() const { return ""; }

    std::vector<Enum16> array_;
};

class Struct_Array_Field_EnumElement_UnknownSizeView {
public:
    static Struct_Array_Field_EnumElement_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_EnumElement_UnknownSizeView(parent);
    }

    Struct_Array_Field_EnumElement_UnknownSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_EnumElement_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_EnumElement_UnknownSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_EnumElement_UnknownSize_ s_;


};

class Struct_Array_Field_EnumElement_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_EnumElement_UnknownSizeBuilder() override = default;
    Struct_Array_Field_EnumElement_UnknownSizeBuilder() = default;
    explicit Struct_Array_Field_EnumElement_UnknownSizeBuilder(Struct_Array_Field_EnumElement_UnknownSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_EnumElement_UnknownSizeBuilder(Struct_Array_Field_EnumElement_UnknownSizeBuilder const&) = default;
    Struct_Array_Field_EnumElement_UnknownSizeBuilder(Struct_Array_Field_EnumElement_UnknownSizeBuilder&&) = default;
    Struct_Array_Field_EnumElement_UnknownSizeBuilder& operator=(Struct_Array_Field_EnumElement_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_EnumElement_UnknownSize_ s_;
};

class Struct_Array_Field_SizedElement_ConstantSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_SizedElement_ConstantSize_() override = default;
    Struct_Array_Field_SizedElement_ConstantSize_() = default;
    explicit Struct_Array_Field_SizedElement_ConstantSize_(std::array<SizedStruct, 4> array) : array_(std::move(array)) {}
    Struct_Array_Field_SizedElement_ConstantSize_(Struct_Array_Field_SizedElement_ConstantSize_ const&) = default;
    Struct_Array_Field_SizedElement_ConstantSize_(Struct_Array_Field_SizedElement_ConstantSize_&&) = default;
    Struct_Array_Field_SizedElement_ConstantSize_& operator=(Struct_Array_Field_SizedElement_ConstantSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_SizedElement_ConstantSize_* output) {
        pdl::packet::slice span = parent_span;
        for (int n = 0; n < 4; n++) {
            if (!SizedStruct::Parse(span, &output->array_[n])) return false;
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::array<SizedStruct, 4> array_;
};

class Struct_Array_Field_SizedElement_ConstantSizeView {
public:
    static Struct_Array_Field_SizedElement_ConstantSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_SizedElement_ConstantSizeView(parent);
    }

    Struct_Array_Field_SizedElement_ConstantSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_SizedElement_ConstantSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_SizedElement_ConstantSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_SizedElement_ConstantSize_ s_;


};

class Struct_Array_Field_SizedElement_ConstantSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_SizedElement_ConstantSizeBuilder() override = default;
    Struct_Array_Field_SizedElement_ConstantSizeBuilder() = default;
    explicit Struct_Array_Field_SizedElement_ConstantSizeBuilder(Struct_Array_Field_SizedElement_ConstantSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_SizedElement_ConstantSizeBuilder(Struct_Array_Field_SizedElement_ConstantSizeBuilder const&) = default;
    Struct_Array_Field_SizedElement_ConstantSizeBuilder(Struct_Array_Field_SizedElement_ConstantSizeBuilder&&) = default;
    Struct_Array_Field_SizedElement_ConstantSizeBuilder& operator=(Struct_Array_Field_SizedElement_ConstantSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_SizedElement_ConstantSize_ s_;
};

class Struct_Array_Field_SizedElement_VariableSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_SizedElement_VariableSize_() override = default;
    Struct_Array_Field_SizedElement_VariableSize_() = default;
    explicit Struct_Array_Field_SizedElement_VariableSize_(std::vector<SizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_SizedElement_VariableSize_(Struct_Array_Field_SizedElement_VariableSize_ const&) = default;
    Struct_Array_Field_SizedElement_VariableSize_(Struct_Array_Field_SizedElement_VariableSize_&&) = default;
    Struct_Array_Field_SizedElement_VariableSize_& operator=(Struct_Array_Field_SizedElement_VariableSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_SizedElement_VariableSize_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_size_ = (chunk0 >> 0) & 0xf;
        size_t limit = (span.size() > output->array_size_) ? (span.size() - output->array_size_) : 0;
        while (span.size() > limit) {
            SizedStruct element;
            if (!SizedStruct::Parse(span, &element)) return false;
            output->array_.emplace_back(std::move(element));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& element) { return s + element.GetSize(); });
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<SizedStruct> array_;
};

class Struct_Array_Field_SizedElement_VariableSizeView {
public:
    static Struct_Array_Field_SizedElement_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_SizedElement_VariableSizeView(parent);
    }

    Struct_Array_Field_SizedElement_VariableSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_SizedElement_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_SizedElement_VariableSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_SizedElement_VariableSize_ s_;


};

class Struct_Array_Field_SizedElement_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_SizedElement_VariableSizeBuilder() override = default;
    Struct_Array_Field_SizedElement_VariableSizeBuilder() = default;
    explicit Struct_Array_Field_SizedElement_VariableSizeBuilder(Struct_Array_Field_SizedElement_VariableSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_SizedElement_VariableSizeBuilder(Struct_Array_Field_SizedElement_VariableSizeBuilder const&) = default;
    Struct_Array_Field_SizedElement_VariableSizeBuilder(Struct_Array_Field_SizedElement_VariableSizeBuilder&&) = default;
    Struct_Array_Field_SizedElement_VariableSizeBuilder& operator=(Struct_Array_Field_SizedElement_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_SizedElement_VariableSize_ s_;
};

class Struct_Array_Field_SizedElement_VariableCount_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_SizedElement_VariableCount_() override = default;
    Struct_Array_Field_SizedElement_VariableCount_() = default;
    explicit Struct_Array_Field_SizedElement_VariableCount_(std::vector<SizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_SizedElement_VariableCount_(Struct_Array_Field_SizedElement_VariableCount_ const&) = default;
    Struct_Array_Field_SizedElement_VariableCount_(Struct_Array_Field_SizedElement_VariableCount_&&) = default;
    Struct_Array_Field_SizedElement_VariableCount_& operator=(Struct_Array_Field_SizedElement_VariableCount_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_SizedElement_VariableCount_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_count_ = (chunk0 >> 0) & 0xf;
        for (size_t n = 0; n < output->array_count_; n++) {
            SizedStruct element;
            if (!SizedStruct::Parse(span, &element)) return false;
            output->array_.emplace_back(std::move(element));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<SizedStruct> array_;
};

class Struct_Array_Field_SizedElement_VariableCountView {
public:
    static Struct_Array_Field_SizedElement_VariableCountView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_SizedElement_VariableCountView(parent);
    }

    Struct_Array_Field_SizedElement_VariableCount_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_SizedElement_VariableCountView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_SizedElement_VariableCount_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_SizedElement_VariableCount_ s_;


};

class Struct_Array_Field_SizedElement_VariableCountBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_SizedElement_VariableCountBuilder() override = default;
    Struct_Array_Field_SizedElement_VariableCountBuilder() = default;
    explicit Struct_Array_Field_SizedElement_VariableCountBuilder(Struct_Array_Field_SizedElement_VariableCount_ s) : s_(std::move(s)) {}
    Struct_Array_Field_SizedElement_VariableCountBuilder(Struct_Array_Field_SizedElement_VariableCountBuilder const&) = default;
    Struct_Array_Field_SizedElement_VariableCountBuilder(Struct_Array_Field_SizedElement_VariableCountBuilder&&) = default;
    Struct_Array_Field_SizedElement_VariableCountBuilder& operator=(Struct_Array_Field_SizedElement_VariableCountBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_SizedElement_VariableCount_ s_;
};

class Struct_Array_Field_SizedElement_UnknownSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_SizedElement_UnknownSize_() override = default;
    Struct_Array_Field_SizedElement_UnknownSize_() = default;
    explicit Struct_Array_Field_SizedElement_UnknownSize_(std::vector<SizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_SizedElement_UnknownSize_(Struct_Array_Field_SizedElement_UnknownSize_ const&) = default;
    Struct_Array_Field_SizedElement_UnknownSize_(Struct_Array_Field_SizedElement_UnknownSize_&&) = default;
    Struct_Array_Field_SizedElement_UnknownSize_& operator=(Struct_Array_Field_SizedElement_UnknownSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_SizedElement_UnknownSize_* output) {
        pdl::packet::slice span = parent_span;
        while (span.size() > 0) {
            SizedStruct element;
            if (!SizedStruct::Parse(span, &element)) return false;
            output->array_.emplace_back(std::move(element));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<SizedStruct> array_;
};

class Struct_Array_Field_SizedElement_UnknownSizeView {
public:
    static Struct_Array_Field_SizedElement_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_SizedElement_UnknownSizeView(parent);
    }

    Struct_Array_Field_SizedElement_UnknownSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_SizedElement_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_SizedElement_UnknownSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_SizedElement_UnknownSize_ s_;


};

class Struct_Array_Field_SizedElement_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_SizedElement_UnknownSizeBuilder() override = default;
    Struct_Array_Field_SizedElement_UnknownSizeBuilder() = default;
    explicit Struct_Array_Field_SizedElement_UnknownSizeBuilder(Struct_Array_Field_SizedElement_UnknownSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_SizedElement_UnknownSizeBuilder(Struct_Array_Field_SizedElement_UnknownSizeBuilder const&) = default;
    Struct_Array_Field_SizedElement_UnknownSizeBuilder(Struct_Array_Field_SizedElement_UnknownSizeBuilder&&) = default;
    Struct_Array_Field_SizedElement_UnknownSizeBuilder& operator=(Struct_Array_Field_SizedElement_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_SizedElement_UnknownSize_ s_;
};

class Struct_Array_Field_UnsizedElement_ConstantSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_ConstantSize_() override = default;
    Struct_Array_Field_UnsizedElement_ConstantSize_() = default;
    explicit Struct_Array_Field_UnsizedElement_ConstantSize_(std::array<UnsizedStruct, 4> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_ConstantSize_(Struct_Array_Field_UnsizedElement_ConstantSize_ const&) = default;
    Struct_Array_Field_UnsizedElement_ConstantSize_(Struct_Array_Field_UnsizedElement_ConstantSize_&&) = default;
    Struct_Array_Field_UnsizedElement_ConstantSize_& operator=(Struct_Array_Field_UnsizedElement_ConstantSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_UnsizedElement_ConstantSize_* output) {
        pdl::packet::slice span = parent_span;
        for (int n = 0; n < 4; n++) {
            if (!UnsizedStruct::Parse(span, &output->array_[n])) return false;
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::array<UnsizedStruct, 4> array_;
};

class Struct_Array_Field_UnsizedElement_ConstantSizeView {
public:
    static Struct_Array_Field_UnsizedElement_ConstantSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_UnsizedElement_ConstantSizeView(parent);
    }

    Struct_Array_Field_UnsizedElement_ConstantSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_ConstantSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_UnsizedElement_ConstantSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_UnsizedElement_ConstantSize_ s_;


};

class Struct_Array_Field_UnsizedElement_ConstantSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_ConstantSizeBuilder() override = default;
    Struct_Array_Field_UnsizedElement_ConstantSizeBuilder() = default;
    explicit Struct_Array_Field_UnsizedElement_ConstantSizeBuilder(Struct_Array_Field_UnsizedElement_ConstantSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_UnsizedElement_ConstantSizeBuilder(Struct_Array_Field_UnsizedElement_ConstantSizeBuilder const&) = default;
    Struct_Array_Field_UnsizedElement_ConstantSizeBuilder(Struct_Array_Field_UnsizedElement_ConstantSizeBuilder&&) = default;
    Struct_Array_Field_UnsizedElement_ConstantSizeBuilder& operator=(Struct_Array_Field_UnsizedElement_ConstantSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_UnsizedElement_ConstantSize_ s_;
};

class Struct_Array_Field_UnsizedElement_VariableSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_VariableSize_() override = default;
    Struct_Array_Field_UnsizedElement_VariableSize_() = default;
    explicit Struct_Array_Field_UnsizedElement_VariableSize_(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_VariableSize_(Struct_Array_Field_UnsizedElement_VariableSize_ const&) = default;
    Struct_Array_Field_UnsizedElement_VariableSize_(Struct_Array_Field_UnsizedElement_VariableSize_&&) = default;
    Struct_Array_Field_UnsizedElement_VariableSize_& operator=(Struct_Array_Field_UnsizedElement_VariableSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_UnsizedElement_VariableSize_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_size_ = (chunk0 >> 0) & 0xf;
        size_t limit = (span.size() > output->array_size_) ? (span.size() - output->array_size_) : 0;
        while (span.size() > limit) {
            UnsizedStruct element;
            if (!UnsizedStruct::Parse(span, &element)) return false;
            output->array_.emplace_back(std::move(element));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& element) { return s + element.GetSize(); });
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<UnsizedStruct> array_;
};

class Struct_Array_Field_UnsizedElement_VariableSizeView {
public:
    static Struct_Array_Field_UnsizedElement_VariableSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_UnsizedElement_VariableSizeView(parent);
    }

    Struct_Array_Field_UnsizedElement_VariableSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_VariableSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_UnsizedElement_VariableSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_UnsizedElement_VariableSize_ s_;


};

class Struct_Array_Field_UnsizedElement_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_VariableSizeBuilder() override = default;
    Struct_Array_Field_UnsizedElement_VariableSizeBuilder() = default;
    explicit Struct_Array_Field_UnsizedElement_VariableSizeBuilder(Struct_Array_Field_UnsizedElement_VariableSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_UnsizedElement_VariableSizeBuilder(Struct_Array_Field_UnsizedElement_VariableSizeBuilder const&) = default;
    Struct_Array_Field_UnsizedElement_VariableSizeBuilder(Struct_Array_Field_UnsizedElement_VariableSizeBuilder&&) = default;
    Struct_Array_Field_UnsizedElement_VariableSizeBuilder& operator=(Struct_Array_Field_UnsizedElement_VariableSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_UnsizedElement_VariableSize_ s_;
};

class Struct_Array_Field_UnsizedElement_VariableCount_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_VariableCount_() override = default;
    Struct_Array_Field_UnsizedElement_VariableCount_() = default;
    explicit Struct_Array_Field_UnsizedElement_VariableCount_(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_VariableCount_(Struct_Array_Field_UnsizedElement_VariableCount_ const&) = default;
    Struct_Array_Field_UnsizedElement_VariableCount_(Struct_Array_Field_UnsizedElement_VariableCount_&&) = default;
    Struct_Array_Field_UnsizedElement_VariableCount_& operator=(Struct_Array_Field_UnsizedElement_VariableCount_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_UnsizedElement_VariableCount_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_count_ = (chunk0 >> 0) & 0xf;
        for (size_t n = 0; n < output->array_count_; n++) {
            UnsizedStruct element;
            if (!UnsizedStruct::Parse(span, &element)) return false;
            output->array_.emplace_back(std::move(element));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<UnsizedStruct> array_;
};

class Struct_Array_Field_UnsizedElement_VariableCountView {
public:
    static Struct_Array_Field_UnsizedElement_VariableCountView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_UnsizedElement_VariableCountView(parent);
    }

    Struct_Array_Field_UnsizedElement_VariableCount_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_VariableCountView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_UnsizedElement_VariableCount_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_UnsizedElement_VariableCount_ s_;


};

class Struct_Array_Field_UnsizedElement_VariableCountBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_VariableCountBuilder() override = default;
    Struct_Array_Field_UnsizedElement_VariableCountBuilder() = default;
    explicit Struct_Array_Field_UnsizedElement_VariableCountBuilder(Struct_Array_Field_UnsizedElement_VariableCount_ s) : s_(std::move(s)) {}
    Struct_Array_Field_UnsizedElement_VariableCountBuilder(Struct_Array_Field_UnsizedElement_VariableCountBuilder const&) = default;
    Struct_Array_Field_UnsizedElement_VariableCountBuilder(Struct_Array_Field_UnsizedElement_VariableCountBuilder&&) = default;
    Struct_Array_Field_UnsizedElement_VariableCountBuilder& operator=(Struct_Array_Field_UnsizedElement_VariableCountBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_UnsizedElement_VariableCount_ s_;
};

class Struct_Array_Field_UnsizedElement_UnknownSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_UnknownSize_() override = default;
    Struct_Array_Field_UnsizedElement_UnknownSize_() = default;
    explicit Struct_Array_Field_UnsizedElement_UnknownSize_(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_UnknownSize_(Struct_Array_Field_UnsizedElement_UnknownSize_ const&) = default;
    Struct_Array_Field_UnsizedElement_UnknownSize_(Struct_Array_Field_UnsizedElement_UnknownSize_&&) = default;
    Struct_Array_Field_UnsizedElement_UnknownSize_& operator=(Struct_Array_Field_UnsizedElement_UnknownSize_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_UnsizedElement_UnknownSize_* output) {
        pdl::packet::slice span = parent_span;
        while (span.size() > 0) {
            UnsizedStruct element;
            if (!UnsizedStruct::Parse(span, &element)) return false;
            output->array_.emplace_back(std::move(element));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<UnsizedStruct> array_;
};

class Struct_Array_Field_UnsizedElement_UnknownSizeView {
public:
    static Struct_Array_Field_UnsizedElement_UnknownSizeView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_UnsizedElement_UnknownSizeView(parent);
    }

    Struct_Array_Field_UnsizedElement_UnknownSize_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_UnknownSizeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_UnsizedElement_UnknownSize_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_UnsizedElement_UnknownSize_ s_;


};

class Struct_Array_Field_UnsizedElement_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_UnknownSizeBuilder() override = default;
    Struct_Array_Field_UnsizedElement_UnknownSizeBuilder() = default;
    explicit Struct_Array_Field_UnsizedElement_UnknownSizeBuilder(Struct_Array_Field_UnsizedElement_UnknownSize_ s) : s_(std::move(s)) {}
    Struct_Array_Field_UnsizedElement_UnknownSizeBuilder(Struct_Array_Field_UnsizedElement_UnknownSizeBuilder const&) = default;
    Struct_Array_Field_UnsizedElement_UnknownSizeBuilder(Struct_Array_Field_UnsizedElement_UnknownSizeBuilder&&) = default;
    Struct_Array_Field_UnsizedElement_UnknownSizeBuilder& operator=(Struct_Array_Field_UnsizedElement_UnknownSizeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_UnsizedElement_UnknownSize_ s_;
};

class Struct_Array_Field_UnsizedElement_SizeModifier_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_SizeModifier_() override = default;
    Struct_Array_Field_UnsizedElement_SizeModifier_() = default;
    explicit Struct_Array_Field_UnsizedElement_SizeModifier_(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_SizeModifier_(Struct_Array_Field_UnsizedElement_SizeModifier_ const&) = default;
    Struct_Array_Field_UnsizedElement_SizeModifier_(Struct_Array_Field_UnsizedElement_SizeModifier_&&) = default;
    Struct_Array_Field_UnsizedElement_SizeModifier_& operator=(Struct_Array_Field_UnsizedElement_SizeModifier_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_UnsizedElement_SizeModifier_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_size_ = (chunk0 >> 0) & 0xf;
        size_t limit = (span.size() > (output->array_size_ - 2)) ? (span.size() - (output->array_size_ - 2)) : 0;
        while (span.size() > limit) {
            UnsizedStruct element;
            if (!UnsizedStruct::Parse(span, &element)) return false;
            output->array_.emplace_back(std::move(element));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& element) { return s + element.GetSize(); }) +2);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            element.Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<UnsizedStruct> array_;
};

class Struct_Array_Field_UnsizedElement_SizeModifierView {
public:
    static Struct_Array_Field_UnsizedElement_SizeModifierView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_UnsizedElement_SizeModifierView(parent);
    }

    Struct_Array_Field_UnsizedElement_SizeModifier_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_SizeModifierView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_UnsizedElement_SizeModifier_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_UnsizedElement_SizeModifier_ s_;


};

class Struct_Array_Field_UnsizedElement_SizeModifierBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_SizeModifierBuilder() override = default;
    Struct_Array_Field_UnsizedElement_SizeModifierBuilder() = default;
    explicit Struct_Array_Field_UnsizedElement_SizeModifierBuilder(Struct_Array_Field_UnsizedElement_SizeModifier_ s) : s_(std::move(s)) {}
    Struct_Array_Field_UnsizedElement_SizeModifierBuilder(Struct_Array_Field_UnsizedElement_SizeModifierBuilder const&) = default;
    Struct_Array_Field_UnsizedElement_SizeModifierBuilder(Struct_Array_Field_UnsizedElement_SizeModifierBuilder&&) = default;
    Struct_Array_Field_UnsizedElement_SizeModifierBuilder& operator=(Struct_Array_Field_UnsizedElement_SizeModifierBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_UnsizedElement_SizeModifier_ s_;
};

class Struct_Array_Field_SizedElement_VariableSize_Padded_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_SizedElement_VariableSize_Padded_() override = default;
    Struct_Array_Field_SizedElement_VariableSize_Padded_() = default;
    explicit Struct_Array_Field_SizedElement_VariableSize_Padded_(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_SizedElement_VariableSize_Padded_(Struct_Array_Field_SizedElement_VariableSize_Padded_ const&) = default;
    Struct_Array_Field_SizedElement_VariableSize_Padded_(Struct_Array_Field_SizedElement_VariableSize_Padded_&&) = default;
    Struct_Array_Field_SizedElement_VariableSize_Padded_& operator=(Struct_Array_Field_SizedElement_VariableSize_Padded_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_SizedElement_VariableSize_Padded_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        output->array_size_ = (chunk0 >> 0) & 0xf;
        size_t array_start_size = span.size();
        size_t limit = (span.size() > output->array_size_) ? (span.size() - output->array_size_) : 0;
        while (span.size() > limit) {
            if (span.size() < 2) return false;
            output->array_.push_back(span.read_le<uint16_t, 2>());
        }
        if (array_start_size - span.size() < 16) {
            if (span.size() < 16 - (array_start_size - span.size())) return false;
            span.skip(16 - (array_start_size - span.size()));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = (array_.size() * 2);
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        size_t array_start = output.size();
        for (auto const& element : array_) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(element));
        }
        if (output.size() - array_start < 16) {
            output.resize(array_start + 16, 0);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::max<size_t>((array_.size() * 2), 16));
    }

    std::string ToString() const { return ""; }

    uint8_t array_size_ {0};
    std::vector<uint16_t> array_;
};

class Struct_Array_Field_SizedElement_VariableSize_PaddedView {
public:
    static Struct_Array_Field_SizedElement_VariableSize_PaddedView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_SizedElement_VariableSize_PaddedView(parent);
    }

    Struct_Array_Field_SizedElement_VariableSize_Padded_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_SizedElement_VariableSize_PaddedView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_SizedElement_VariableSize_Padded_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_SizedElement_VariableSize_Padded_ s_;


};

class Struct_Array_Field_SizedElement_VariableSize_PaddedBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_SizedElement_VariableSize_PaddedBuilder() override = default;
    Struct_Array_Field_SizedElement_VariableSize_PaddedBuilder() = default;
    explicit Struct_Array_Field_SizedElement_VariableSize_PaddedBuilder(Struct_Array_Field_SizedElement_VariableSize_Padded_ s) : s_(std::move(s)) {}
    Struct_Array_Field_SizedElement_VariableSize_PaddedBuilder(Struct_Array_Field_SizedElement_VariableSize_PaddedBuilder const&) = default;
    Struct_Array_Field_SizedElement_VariableSize_PaddedBuilder(Struct_Array_Field_SizedElement_VariableSize_PaddedBuilder&&) = default;
    Struct_Array_Field_SizedElement_VariableSize_PaddedBuilder& operator=(Struct_Array_Field_SizedElement_VariableSize_PaddedBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_SizedElement_VariableSize_Padded_ s_;
};

class Struct_Array_Field_UnsizedElement_VariableCount_Padded_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_VariableCount_Padded_() override = default;
    Struct_Array_Field_UnsizedElement_VariableCount_Padded_() = default;
    explicit Struct_Array_Field_UnsizedElement_VariableCount_Padded_(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_VariableCount_Padded_(Struct_Array_Field_UnsizedElement_VariableCount_Padded_ const&) = default;
    Struct_Array_Field_UnsizedElement_VariableCount_Padded_(Struct_Array_Field_UnsizedElement_VariableCount_Padded_&&) = default;
    Struct_Array_Field_UnsizedElement_VariableCount_Padded_& operator=(Struct_Array_Field_UnsizedElement_VariableCount_Padded_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Array_Field_UnsizedElement_VariableCount_Padded_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        output->array_count_ = span.read_le<uint8_t, 1>();
        size_t array_start_size = span.size();
        for (size_t n = 0; n < output->array_count_; n++) {
            UnsizedStruct element;
            if (!UnsizedStruct::Parse(span, &element)) return false;
            output->array_.emplace_back(std::move(element));
        }
        if (array_start_size - span.size() < 16) {
            if (span.size() < 16 - (array_start_size - span.size())) return false;
            span.skip(16 - (array_start_size - span.size()));
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        size_t array_start = output.size();
        for (auto const& element : array_) {
            element.Serialize(output);
        }
        if (output.size() - array_start < 16) {
            output.resize(array_start + 16, 0);
        }
    }

    size_t GetSize() const override {
        return 1 + (std::max<size_t>(std::accumulate(array_.begin(), array_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) { return s + e.GetSize(); }), 16));
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<UnsizedStruct> array_;
};

class Struct_Array_Field_UnsizedElement_VariableCount_PaddedView {
public:
    static Struct_Array_Field_UnsizedElement_VariableCount_PaddedView Create(pdl::packet::slice const& parent) {
        return Struct_Array_Field_UnsizedElement_VariableCount_PaddedView(parent);
    }

    Struct_Array_Field_UnsizedElement_VariableCount_Padded_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_VariableCount_PaddedView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Array_Field_UnsizedElement_VariableCount_Padded_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Array_Field_UnsizedElement_VariableCount_Padded_ s_;


};

class Struct_Array_Field_UnsizedElement_VariableCount_PaddedBuilder : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_VariableCount_PaddedBuilder() override = default;
    Struct_Array_Field_UnsizedElement_VariableCount_PaddedBuilder() = default;
    explicit Struct_Array_Field_UnsizedElement_VariableCount_PaddedBuilder(Struct_Array_Field_UnsizedElement_VariableCount_Padded_ s) : s_(std::move(s)) {}
    Struct_Array_Field_UnsizedElement_VariableCount_PaddedBuilder(Struct_Array_Field_UnsizedElement_VariableCount_PaddedBuilder const&) = default;
    Struct_Array_Field_UnsizedElement_VariableCount_PaddedBuilder(Struct_Array_Field_UnsizedElement_VariableCount_PaddedBuilder&&) = default;
    Struct_Array_Field_UnsizedElement_VariableCount_PaddedBuilder& operator=(Struct_Array_Field_UnsizedElement_VariableCount_PaddedBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_UnsizedElement_VariableCount_Padded_ s_;
};

class Struct_Optional_Scalar_Field_ : public pdl::packet::Builder {
public:
    ~Struct_Optional_Scalar_Field_() override = default;
    Struct_Optional_Scalar_Field_() = default;
    explicit Struct_Optional_Scalar_Field_(std::optional<uint32_t> a, std::optional<uint32_t> b) : a_(std::move(a)), b_(std::move(b)) {}
    Struct_Optional_Scalar_Field_(Struct_Optional_Scalar_Field_ const&) = default;
    Struct_Optional_Scalar_Field_(Struct_Optional_Scalar_Field_&&) = default;
    Struct_Optional_Scalar_Field_& operator=(Struct_Optional_Scalar_Field_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Optional_Scalar_Field_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            if (span.size() < 3) {
                return false;
            }
            output->a_ = span.read_le<uint32_t, 3>();
        }
        if (c1 == 1) {
            if (span.size() < 4) {
                return false;
            }
            output->b_ = span.read_le<uint32_t, 4>();
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>((a_.has_value() ? 0 : 1))) | (static_cast<uint8_t>((b_.has_value() ? 1 : 0)) << 1));
        if ((a_.has_value() ? 0 : 1) == 0) {
            pdl::packet::Builder::write_le<uint32_t, 3>(output, *a_);
        }
        if ((b_.has_value() ? 1 : 0) == 1) {
            pdl::packet::Builder::write_le<uint32_t, 4>(output, *b_);
        }
    }

    size_t GetSize() const override {
        return 1 + ((((a_.has_value() ? 0 : 1) == 0) ? 3 : 0) + (((b_.has_value() ? 1 : 0) == 1) ? 4 : 0));
    }

    std::string ToString() const { return ""; }

    std::optional<uint32_t> a_;
    std::optional<uint32_t> b_;
};

class Struct_Optional_Scalar_FieldView {
public:
    static Struct_Optional_Scalar_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_Optional_Scalar_FieldView(parent);
    }

    Struct_Optional_Scalar_Field_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Optional_Scalar_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Optional_Scalar_Field_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Optional_Scalar_Field_ s_;


};

class Struct_Optional_Scalar_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_Optional_Scalar_FieldBuilder() override = default;
    Struct_Optional_Scalar_FieldBuilder() = default;
    explicit Struct_Optional_Scalar_FieldBuilder(Struct_Optional_Scalar_Field_ s) : s_(std::move(s)) {}
    Struct_Optional_Scalar_FieldBuilder(Struct_Optional_Scalar_FieldBuilder const&) = default;
    Struct_Optional_Scalar_FieldBuilder(Struct_Optional_Scalar_FieldBuilder&&) = default;
    Struct_Optional_Scalar_FieldBuilder& operator=(Struct_Optional_Scalar_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Optional_Scalar_Field_ s_;
};

class Struct_Optional_Enum_Field_ : public pdl::packet::Builder {
public:
    ~Struct_Optional_Enum_Field_() override = default;
    Struct_Optional_Enum_Field_() = default;
    explicit Struct_Optional_Enum_Field_(std::optional<Enum16> a, std::optional<Enum16> b) : a_(std::move(a)), b_(std::move(b)) {}
    Struct_Optional_Enum_Field_(Struct_Optional_Enum_Field_ const&) = default;
    Struct_Optional_Enum_Field_(Struct_Optional_Enum_Field_&&) = default;
    Struct_Optional_Enum_Field_& operator=(Struct_Optional_Enum_Field_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Optional_Enum_Field_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            if (span.size() < 2) {
                return false;
            }
            output->a_ = Enum16(span.read_le<uint16_t, 2>());
        }
        if (c1 == 1) {
            if (span.size() < 2) {
                return false;
            }
            output->b_ = Enum16(span.read_le<uint16_t, 2>());
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>((a_.has_value() ? 0 : 1))) | (static_cast<uint8_t>((b_.has_value() ? 1 : 0)) << 1));
        if ((a_.has_value() ? 0 : 1) == 0) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(*a_));
        }
        if ((b_.has_value() ? 1 : 0) == 1) {
            pdl::packet::Builder::write_le<uint16_t, 2>(output, static_cast<uint16_t>(*b_));
        }
    }

    size_t GetSize() const override {
        return 1 + ((((a_.has_value() ? 0 : 1) == 0) ? 2 : 0) + (((b_.has_value() ? 1 : 0) == 1) ? 2 : 0));
    }

    std::string ToString() const { return ""; }

    std::optional<Enum16> a_;
    std::optional<Enum16> b_;
};

class Struct_Optional_Enum_FieldView {
public:
    static Struct_Optional_Enum_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_Optional_Enum_FieldView(parent);
    }

    Struct_Optional_Enum_Field_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Optional_Enum_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Optional_Enum_Field_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Optional_Enum_Field_ s_;


};

class Struct_Optional_Enum_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_Optional_Enum_FieldBuilder() override = default;
    Struct_Optional_Enum_FieldBuilder() = default;
    explicit Struct_Optional_Enum_FieldBuilder(Struct_Optional_Enum_Field_ s) : s_(std::move(s)) {}
    Struct_Optional_Enum_FieldBuilder(Struct_Optional_Enum_FieldBuilder const&) = default;
    Struct_Optional_Enum_FieldBuilder(Struct_Optional_Enum_FieldBuilder&&) = default;
    Struct_Optional_Enum_FieldBuilder& operator=(Struct_Optional_Enum_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Optional_Enum_Field_ s_;
};

class Struct_Optional_Struct_Field_ : public pdl::packet::Builder {
public:
    ~Struct_Optional_Struct_Field_() override = default;
    Struct_Optional_Struct_Field_() = default;
    explicit Struct_Optional_Struct_Field_(std::optional<SizedStruct> a, std::optional<UnsizedStruct> b) : a_(std::move(a)), b_(std::move(b)) {}
    Struct_Optional_Struct_Field_(Struct_Optional_Struct_Field_ const&) = default;
    Struct_Optional_Struct_Field_(Struct_Optional_Struct_Field_&&) = default;
    Struct_Optional_Struct_Field_& operator=(Struct_Optional_Struct_Field_ const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, Struct_Optional_Struct_Field_* output) {
        pdl::packet::slice span = parent_span;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            auto& opt_output = output->a_.emplace();
            if (!SizedStruct::Parse(span, &opt_output)) {
                return false;
            }
        }
        if (c1 == 1) {
            auto& opt_output = output->b_.emplace();
            if (!UnsizedStruct::Parse(span, &opt_output)) {
                return false;
            }
        }
        parent_span = span;
        return true;
    }

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>((a_.has_value() ? 0 : 1))) | (static_cast<uint8_t>((b_.has_value() ? 1 : 0)) << 1));
        if ((a_.has_value() ? 0 : 1) == 0) {
            a_->Serialize(output);
        }
        if ((b_.has_value() ? 1 : 0) == 1) {
            b_->Serialize(output);
        }
    }

    size_t GetSize() const override {
        return 1 + ((((a_.has_value() ? 0 : 1) == 0) ? a_->GetSize() : 0) + (((b_.has_value() ? 1 : 0) == 1) ? b_->GetSize() : 0));
    }

    std::string ToString() const { return ""; }

    std::optional<SizedStruct> a_;
    std::optional<UnsizedStruct> b_;
};

class Struct_Optional_Struct_FieldView {
public:
    static Struct_Optional_Struct_FieldView Create(pdl::packet::slice const& parent) {
        return Struct_Optional_Struct_FieldView(parent);
    }

    Struct_Optional_Struct_Field_ const& GetS() const { _ASSERT_VALID(valid_); return s_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Optional_Struct_FieldView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (!Struct_Optional_Struct_Field_::Parse(span, &s_)) return false;
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Struct_Optional_Struct_Field_ s_;


};

class Struct_Optional_Struct_FieldBuilder : public pdl::packet::Builder {
public:
    ~Struct_Optional_Struct_FieldBuilder() override = default;
    Struct_Optional_Struct_FieldBuilder() = default;
    explicit Struct_Optional_Struct_FieldBuilder(Struct_Optional_Struct_Field_ s) : s_(std::move(s)) {}
    Struct_Optional_Struct_FieldBuilder(Struct_Optional_Struct_FieldBuilder const&) = default;
    Struct_Optional_Struct_FieldBuilder(Struct_Optional_Struct_FieldBuilder&&) = default;
    Struct_Optional_Struct_FieldBuilder& operator=(Struct_Optional_Struct_FieldBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        s_.Serialize(output);
    }

    size_t GetSize() const override {
        return s_.GetSize();
    }

    std::string ToString() const { return ""; }

    Struct_Optional_Struct_Field_ s_;
};

enum class Enum_Incomplete_Truncated_Closed_ : uint8_t {
    A = 0x0,
    B = 0x1,
};

inline std::string Enum_Incomplete_Truncated_Closed_Text(Enum_Incomplete_Truncated_Closed_ tag) {
    switch (tag) {
        case Enum_Incomplete_Truncated_Closed_::A: return "A";
        case Enum_Incomplete_Truncated_Closed_::B: return "B";
        default:
            return std::string("Unknown Enum_Incomplete_Truncated_Closed_: " +
                   std::to_string(static_cast<uint64_t>(tag)));
    }
}

class Enum_Incomplete_Truncated_ClosedView {
public:
    static Enum_Incomplete_Truncated_ClosedView Create(pdl::packet::slice const& parent) {
        return Enum_Incomplete_Truncated_ClosedView(parent);
    }

    Enum_Incomplete_Truncated_Closed_ GetE() const { _ASSERT_VALID(valid_); return e_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Incomplete_Truncated_ClosedView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        e_ = Enum_Incomplete_Truncated_Closed_((chunk0 >> 0) & 0x7);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Enum_Incomplete_Truncated_Closed_ e_{Enum_Incomplete_Truncated_Closed_::A};


};

class Enum_Incomplete_Truncated_ClosedBuilder : public pdl::packet::Builder {
public:
    ~Enum_Incomplete_Truncated_ClosedBuilder() override = default;
    Enum_Incomplete_Truncated_ClosedBuilder() = default;
    explicit Enum_Incomplete_Truncated_ClosedBuilder(Enum_Incomplete_Truncated_Closed_ e) : e_(std::move(e)) {}
    Enum_Incomplete_Truncated_ClosedBuilder(Enum_Incomplete_Truncated_ClosedBuilder const&) = default;
    Enum_Incomplete_Truncated_ClosedBuilder(Enum_Incomplete_Truncated_ClosedBuilder&&) = default;
    Enum_Incomplete_Truncated_ClosedBuilder& operator=(Enum_Incomplete_Truncated_ClosedBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(static_cast<uint8_t>(e_))));
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    Enum_Incomplete_Truncated_Closed_ e_{Enum_Incomplete_Truncated_Closed_::A};
};

enum class Enum_Incomplete_Truncated_Open_ : uint8_t {
    A = 0x0,
    B = 0x1,
};

inline std::string Enum_Incomplete_Truncated_Open_Text(Enum_Incomplete_Truncated_Open_ tag) {
    switch (tag) {
        case Enum_Incomplete_Truncated_Open_::A: return "A";
        case Enum_Incomplete_Truncated_Open_::B: return "B";
        default:
            return std::string("Unknown Enum_Incomplete_Truncated_Open_: " +
                   std::to_string(static_cast<uint64_t>(tag)));
    }
}

class Enum_Incomplete_Truncated_OpenView {
public:
    static Enum_Incomplete_Truncated_OpenView Create(pdl::packet::slice const& parent) {
        return Enum_Incomplete_Truncated_OpenView(parent);
    }

    Enum_Incomplete_Truncated_Open_ GetE() const { _ASSERT_VALID(valid_); return e_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Incomplete_Truncated_OpenView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        e_ = Enum_Incomplete_Truncated_Open_((chunk0 >> 0) & 0x7);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Enum_Incomplete_Truncated_Open_ e_{Enum_Incomplete_Truncated_Open_::A};


};

class Enum_Incomplete_Truncated_OpenBuilder : public pdl::packet::Builder {
public:
    ~Enum_Incomplete_Truncated_OpenBuilder() override = default;
    Enum_Incomplete_Truncated_OpenBuilder() = default;
    explicit Enum_Incomplete_Truncated_OpenBuilder(Enum_Incomplete_Truncated_Open_ e) : e_(std::move(e)) {}
    Enum_Incomplete_Truncated_OpenBuilder(Enum_Incomplete_Truncated_OpenBuilder const&) = default;
    Enum_Incomplete_Truncated_OpenBuilder(Enum_Incomplete_Truncated_OpenBuilder&&) = default;
    Enum_Incomplete_Truncated_OpenBuilder& operator=(Enum_Incomplete_Truncated_OpenBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(static_cast<uint8_t>(e_))));
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    Enum_Incomplete_Truncated_Open_ e_{Enum_Incomplete_Truncated_Open_::A};
};

enum class Enum_Incomplete_Truncated_Closed_WithRange_ : uint8_t {
    A = 0x0,
};

inline std::string Enum_Incomplete_Truncated_Closed_WithRange_Text(Enum_Incomplete_Truncated_Closed_WithRange_ tag) {
    switch (tag) {
        case Enum_Incomplete_Truncated_Closed_WithRange_::A: return "A";
        default:
            return std::string("Unknown Enum_Incomplete_Truncated_Closed_WithRange_: " +
                   std::to_string(static_cast<uint64_t>(tag)));
    }
}

class Enum_Incomplete_Truncated_Closed_WithRangeView {
public:
    static Enum_Incomplete_Truncated_Closed_WithRangeView Create(pdl::packet::slice const& parent) {
        return Enum_Incomplete_Truncated_Closed_WithRangeView(parent);
    }

    Enum_Incomplete_Truncated_Closed_WithRange_ GetE() const { _ASSERT_VALID(valid_); return e_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Incomplete_Truncated_Closed_WithRangeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        e_ = Enum_Incomplete_Truncated_Closed_WithRange_((chunk0 >> 0) & 0x7);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Enum_Incomplete_Truncated_Closed_WithRange_ e_{Enum_Incomplete_Truncated_Closed_WithRange_::A};


};

class Enum_Incomplete_Truncated_Closed_WithRangeBuilder : public pdl::packet::Builder {
public:
    ~Enum_Incomplete_Truncated_Closed_WithRangeBuilder() override = default;
    Enum_Incomplete_Truncated_Closed_WithRangeBuilder() = default;
    explicit Enum_Incomplete_Truncated_Closed_WithRangeBuilder(Enum_Incomplete_Truncated_Closed_WithRange_ e) : e_(std::move(e)) {}
    Enum_Incomplete_Truncated_Closed_WithRangeBuilder(Enum_Incomplete_Truncated_Closed_WithRangeBuilder const&) = default;
    Enum_Incomplete_Truncated_Closed_WithRangeBuilder(Enum_Incomplete_Truncated_Closed_WithRangeBuilder&&) = default;
    Enum_Incomplete_Truncated_Closed_WithRangeBuilder& operator=(Enum_Incomplete_Truncated_Closed_WithRangeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(static_cast<uint8_t>(e_))));
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    Enum_Incomplete_Truncated_Closed_WithRange_ e_{Enum_Incomplete_Truncated_Closed_WithRange_::A};
};

enum class Enum_Incomplete_Truncated_Open_WithRange_ : uint8_t {
    A = 0x0,
};

inline std::string Enum_Incomplete_Truncated_Open_WithRange_Text(Enum_Incomplete_Truncated_Open_WithRange_ tag) {
    switch (tag) {
        case Enum_Incomplete_Truncated_Open_WithRange_::A: return "A";
        default:
            return std::string("Unknown Enum_Incomplete_Truncated_Open_WithRange_: " +
                   std::to_string(static_cast<uint64_t>(tag)));
    }
}

class Enum_Incomplete_Truncated_Open_WithRangeView {
public:
    static Enum_Incomplete_Truncated_Open_WithRangeView Create(pdl::packet::slice const& parent) {
        return Enum_Incomplete_Truncated_Open_WithRangeView(parent);
    }

    Enum_Incomplete_Truncated_Open_WithRange_ GetE() const { _ASSERT_VALID(valid_); return e_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Incomplete_Truncated_Open_WithRangeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        e_ = Enum_Incomplete_Truncated_Open_WithRange_((chunk0 >> 0) & 0x7);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Enum_Incomplete_Truncated_Open_WithRange_ e_{Enum_Incomplete_Truncated_Open_WithRange_::A};


};

class Enum_Incomplete_Truncated_Open_WithRangeBuilder : public pdl::packet::Builder {
public:
    ~Enum_Incomplete_Truncated_Open_WithRangeBuilder() override = default;
    Enum_Incomplete_Truncated_Open_WithRangeBuilder() = default;
    explicit Enum_Incomplete_Truncated_Open_WithRangeBuilder(Enum_Incomplete_Truncated_Open_WithRange_ e) : e_(std::move(e)) {}
    Enum_Incomplete_Truncated_Open_WithRangeBuilder(Enum_Incomplete_Truncated_Open_WithRangeBuilder const&) = default;
    Enum_Incomplete_Truncated_Open_WithRangeBuilder(Enum_Incomplete_Truncated_Open_WithRangeBuilder&&) = default;
    Enum_Incomplete_Truncated_Open_WithRangeBuilder& operator=(Enum_Incomplete_Truncated_Open_WithRangeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(static_cast<uint8_t>(e_))));
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    Enum_Incomplete_Truncated_Open_WithRange_ e_{Enum_Incomplete_Truncated_Open_WithRange_::A};
};

enum class Enum_Complete_Truncated_ : uint8_t {
    A = 0x0,
    B = 0x1,
    C = 0x2,
    D = 0x3,
    E = 0x4,
    F = 0x5,
    G = 0x6,
    H = 0x7,
};

inline std::string Enum_Complete_Truncated_Text(Enum_Complete_Truncated_ tag) {
    switch (tag) {
        case Enum_Complete_Truncated_::A: return "A";
        case Enum_Complete_Truncated_::B: return "B";
        case Enum_Complete_Truncated_::C: return "C";
        case Enum_Complete_Truncated_::D: return "D";
        case Enum_Complete_Truncated_::E: return "E";
        case Enum_Complete_Truncated_::F: return "F";
        case Enum_Complete_Truncated_::G: return "G";
        case Enum_Complete_Truncated_::H: return "H";
        default:
            return std::string("Unknown Enum_Complete_Truncated_: " +
                   std::to_string(static_cast<uint64_t>(tag)));
    }
}

class Enum_Complete_TruncatedView {
public:
    static Enum_Complete_TruncatedView Create(pdl::packet::slice const& parent) {
        return Enum_Complete_TruncatedView(parent);
    }

    Enum_Complete_Truncated_ GetE() const { _ASSERT_VALID(valid_); return e_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Complete_TruncatedView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        e_ = Enum_Complete_Truncated_((chunk0 >> 0) & 0x7);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Enum_Complete_Truncated_ e_{Enum_Complete_Truncated_::A};


};

class Enum_Complete_TruncatedBuilder : public pdl::packet::Builder {
public:
    ~Enum_Complete_TruncatedBuilder() override = default;
    Enum_Complete_TruncatedBuilder() = default;
    explicit Enum_Complete_TruncatedBuilder(Enum_Complete_Truncated_ e) : e_(std::move(e)) {}
    Enum_Complete_TruncatedBuilder(Enum_Complete_TruncatedBuilder const&) = default;
    Enum_Complete_TruncatedBuilder(Enum_Complete_TruncatedBuilder&&) = default;
    Enum_Complete_TruncatedBuilder& operator=(Enum_Complete_TruncatedBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(static_cast<uint8_t>(e_))));
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    Enum_Complete_Truncated_ e_{Enum_Complete_Truncated_::A};
};

enum class Enum_Complete_Truncated_WithRange_ : uint8_t {
    A = 0x0,
};

inline std::string Enum_Complete_Truncated_WithRange_Text(Enum_Complete_Truncated_WithRange_ tag) {
    switch (tag) {
        case Enum_Complete_Truncated_WithRange_::A: return "A";
        default:
            return std::string("Unknown Enum_Complete_Truncated_WithRange_: " +
                   std::to_string(static_cast<uint64_t>(tag)));
    }
}

class Enum_Complete_Truncated_WithRangeView {
public:
    static Enum_Complete_Truncated_WithRangeView Create(pdl::packet::slice const& parent) {
        return Enum_Complete_Truncated_WithRangeView(parent);
    }

    Enum_Complete_Truncated_WithRange_ GetE() const { _ASSERT_VALID(valid_); return e_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Complete_Truncated_WithRangeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        uint8_t chunk0 = span.read_le<uint8_t, 1>();
        e_ = Enum_Complete_Truncated_WithRange_((chunk0 >> 0) & 0x7);
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Enum_Complete_Truncated_WithRange_ e_{Enum_Complete_Truncated_WithRange_::A};


};

class Enum_Complete_Truncated_WithRangeBuilder : public pdl::packet::Builder {
public:
    ~Enum_Complete_Truncated_WithRangeBuilder() override = default;
    Enum_Complete_Truncated_WithRangeBuilder() = default;
    explicit Enum_Complete_Truncated_WithRangeBuilder(Enum_Complete_Truncated_WithRange_ e) : e_(std::move(e)) {}
    Enum_Complete_Truncated_WithRangeBuilder(Enum_Complete_Truncated_WithRangeBuilder const&) = default;
    Enum_Complete_Truncated_WithRangeBuilder(Enum_Complete_Truncated_WithRangeBuilder&&) = default;
    Enum_Complete_Truncated_WithRangeBuilder& operator=(Enum_Complete_Truncated_WithRangeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(static_cast<uint8_t>(e_))));
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    Enum_Complete_Truncated_WithRange_ e_{Enum_Complete_Truncated_WithRange_::A};
};

enum class Enum_Complete_WithRange_ : uint8_t {
    A = 0x0,
    B = 0x1,
};

inline std::string Enum_Complete_WithRange_Text(Enum_Complete_WithRange_ tag) {
    switch (tag) {
        case Enum_Complete_WithRange_::A: return "A";
        case Enum_Complete_WithRange_::B: return "B";
        default:
            return std::string("Unknown Enum_Complete_WithRange_: " +
                   std::to_string(static_cast<uint64_t>(tag)));
    }
}

class Enum_Complete_WithRangeView {
public:
    static Enum_Complete_WithRangeView Create(pdl::packet::slice const& parent) {
        return Enum_Complete_WithRangeView(parent);
    }

    Enum_Complete_WithRange_ GetE() const { _ASSERT_VALID(valid_); return e_; }

    std::string ToString() const { return ""; }

    bool IsValid() const {
        return valid_;
    }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Complete_WithRangeView(pdl::packet::slice const& parent)
          : bytes_(parent) {
        valid_ = Parse(parent);
    }

    bool Parse(pdl::packet::slice const& parent) {
        // Parse packet field values.
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return false;
        }
        e_ = Enum_Complete_WithRange_(span.read_le<uint8_t, 1>());
        return true;
    }

    bool valid_{false};
    pdl::packet::slice bytes_;
    Enum_Complete_WithRange_ e_{Enum_Complete_WithRange_::A};


};

class Enum_Complete_WithRangeBuilder : public pdl::packet::Builder {
public:
    ~Enum_Complete_WithRangeBuilder() override = default;
    Enum_Complete_WithRangeBuilder() = default;
    explicit Enum_Complete_WithRangeBuilder(Enum_Complete_WithRange_ e) : e_(std::move(e)) {}
    Enum_Complete_WithRangeBuilder(Enum_Complete_WithRangeBuilder const&) = default;
    Enum_Complete_WithRangeBuilder(Enum_Complete_WithRangeBuilder&&) = default;
    Enum_Complete_WithRangeBuilder& operator=(Enum_Complete_WithRangeBuilder const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_le<uint8_t, 1>(output, (static_cast<uint8_t>(static_cast<uint8_t>(e_))));
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    Enum_Complete_WithRange_ e_{Enum_Complete_WithRange_::A};
};
}  // le_backend
