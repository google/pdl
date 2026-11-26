// File generated from tests/canonical/be_test_file.pdl, with the command
//   pdlc --output-format cxx tests/canonical/be_test_file.pdl
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

namespace be_backend {
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
class Packet_Enum_Field_PackedView;
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
class Packet_Array_Field_VariableElementSize_ConstantSizeView;
class Packet_Array_Field_VariableElementSize_VariableSizeView;
class Packet_Array_Field_VariableElementSize_VariableCountView;
class Packet_Array_Field_VariableElementSize_UnknownSizeView;
class Packet_Optional_Scalar_FieldView;
class Packet_Optional_Enum_FieldView;
class Packet_Optional_Struct_FieldView;
class ScalarChild_AView;
class ScalarChild_BView;
class EnumChild_AView;
class EnumChild_BView;
class AliasedChild_AView;
class AliasedChild_BView;
class ScalarChild_CView;
class ScalarGrandChild_CView;
class ScalarChild_DView;
class ConstrainedParentView;
class ConstrainedChild_AView;
class ConstrainedChild_BView;
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

inline std::string Enum7ToString(Enum7 tag) {
    switch (tag) {
        case Enum7::A: return "A";
        case Enum7::B: return "B";
        default:
            break;
    }

    return std::string("UNKNOWN(" +
           std::to_string(uint8_t(tag)) + ")");
}

inline bool IsValidEnum7(uint8_t value) {
    switch (value) {
        case 0x1:
        case 0x2:
            return true;
        default:
            return false;
    }
}

enum class Enum16 : uint16_t {
    A = 0xaabb,
    B = 0xccdd,
};

inline std::string Enum16ToString(Enum16 tag) {
    switch (tag) {
        case Enum16::A: return "A";
        case Enum16::B: return "B";
        default:
            break;
    }

    return std::string("UNKNOWN(" +
           std::to_string(uint16_t(tag)) + ")");
}

inline bool IsValidEnum16(uint16_t value) {
    switch (value) {
        case 0xaabb:
        case 0xccdd:
            return true;
        default:
            return false;
    }
}

class SizedStruct : public pdl::packet::Builder {
public:
    ~SizedStruct() override = default;
    SizedStruct() = default;
    SizedStruct(SizedStruct const&) = default;
    SizedStruct(SizedStruct&&) = default;
    explicit SizedStruct(uint8_t a) : a_(std::move(a)) {}
    SizedStruct& operator=(SizedStruct const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<SizedStruct> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        SizedStruct output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        output.a_ = span.read_be<uint8_t, 1>();
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(a_)));
        return true;
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
    UnsizedStruct(UnsizedStruct const&) = default;
    UnsizedStruct(UnsizedStruct&&) = default;
    explicit UnsizedStruct(std::vector<uint8_t> array) : array_(std::move(array)) {}
    UnsizedStruct& operator=(UnsizedStruct const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<UnsizedStruct> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        UnsizedStruct output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0x3;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if ((array_size_ % 1) != 0) {
            return std::nullopt;
        }
        auto array_count_ = array_size_ / 1;
        for (size_t n = 0; n < array_count_; n++) {
            output.array_.push_back(span.read_be<uint8_t, 1>());
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = 1 * array_.size();
        if (array_size > 0x3) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> array_;
};

class UnknownSizeStruct : public pdl::packet::Builder {
public:
    ~UnknownSizeStruct() override = default;
    UnknownSizeStruct() = default;
    UnknownSizeStruct(UnknownSizeStruct const&) = default;
    UnknownSizeStruct(UnknownSizeStruct&&) = default;
    explicit UnknownSizeStruct(std::vector<uint8_t> array) : array_(std::move(array)) {}
    UnknownSizeStruct& operator=(UnknownSizeStruct const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<UnknownSizeStruct> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        UnknownSizeStruct output;
        if ((span.size() % 1) != 0) {
            return std::nullopt;
        }
        auto array_count_ = span.size() / 1;
        for (size_t n = 0; n < array_count_; n++) {
            output.array_.push_back(span.read_be<uint8_t, 1>());
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> array_;
};

class ScalarParentView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<ScalarParentView> Decode(pdl::packet::slice const& parent) {
        ScalarParentView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return std::nullopt;
        }
        output.a_ = span.read_be<uint8_t, 1>();
        size_t payload_size_ = span.read_be<uint8_t, 1>();
        if (span.size() < payload_size_) return std::nullopt;
        output.payload_ = span.subrange(0, payload_size_);
        span.skip(payload_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return a_; }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ScalarParentView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    uint8_t a_;
    pdl::packet::slice payload_;

    friend class EmptyParentView;
    friend class ScalarChild_AView;
    friend class ScalarChild_BView;
    friend class ScalarChild_CView;
    friend class ScalarChild_DView;
};

class ScalarParentBuilder : public pdl::packet::Builder {
public:
    ~ScalarParentBuilder() override = default;
    ScalarParentBuilder() = default;
    explicit ScalarParentBuilder(uint8_t a, std::vector<uint8_t> payload) : a_(std::move(a)), payload_(std::move(payload)) {}
    ScalarParentBuilder(ScalarParentBuilder const&) = default;
    ScalarParentBuilder(ScalarParentBuilder&&) = default;
    ScalarParentBuilder& operator=(ScalarParentBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(a_)));
        size_t ScalarParent_payload_size = payload_.size();
        if (ScalarParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(ScalarParent_payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
        return true;
    }

    size_t GetSize() const override {
        return 2 + payload_.size();
    }

    std::string ToString() const { return ""; }

    uint8_t a_{0};
    std::vector<uint8_t> payload_;
};

class EnumParentView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<EnumParentView> Decode(pdl::packet::slice const& parent) {
        EnumParentView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 3) {
            return std::nullopt;
        }
        auto raw_a = span.read_be<uint16_t, 2>();
        if (!IsValidEnum16(raw_a)) {
           return std::nullopt;
        }
        output.a_ = Enum16(raw_a);
        size_t payload_size_ = span.read_be<uint8_t, 1>();
        if (span.size() < payload_size_) return std::nullopt;
        output.payload_ = span.subrange(0, payload_size_);
        span.skip(payload_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum16 GetA() const { return a_; }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit EnumParentView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    Enum16 a_{Enum16::A};
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!IsValidEnum16(uint16_t(a_))) {
            return false;
        }
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(uint16_t(a_))));
        size_t EnumParent_payload_size = payload_.size();
        if (EnumParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(EnumParent_payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
        return true;
    }

    size_t GetSize() const override {
        return 3 + payload_.size();
    }

    std::string ToString() const { return ""; }

    Enum16 a_{Enum16::A};
    std::vector<uint8_t> payload_;
};

class EmptyParentView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<EmptyParentView> Decode(ScalarParentView const& parent) {
        EmptyParentView output(parent.bytes_);
        output.a_ = parent.a_;
        pdl::packet::slice span = parent.payload_;
        output.payload_ = span;
        span.clear();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return a_; }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit EmptyParentView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    uint8_t a_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(a_)));
        size_t ScalarParent_payload_size = payload_.size();
        if (ScalarParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(ScalarParent_payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
        return true;
    }

    size_t GetSize() const override {
        return 2 + payload_.size();
    }

    std::string ToString() const { return ""; }

    uint8_t a_{0};
    std::vector<uint8_t> payload_;
};

class Packet_Scalar_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Scalar_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_Scalar_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        output.a_ = (chunk0 >> 0) & 0x7f;
        output.c_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return a_; }

    uint64_t GetC() const { return c_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Scalar_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (a_ > 0x7f) {
            return false;
        }
        if (c_ > 0x1ffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(a_)) | (static_cast<uint64_t>(c_) << 7));
        return true;
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
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Enum_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_Enum_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        auto raw_a = (chunk0 >> 0) & 0x7f;
        if (!IsValidEnum7(raw_a)) {
           return std::nullopt;
        }
        output.a_ = Enum7(raw_a);
        output.c_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum7 GetA() const { return a_; }

    uint64_t GetC() const { return c_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Enum_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (uint8_t(a_) > 0x7f) {
            return false;
        }
        if (!IsValidEnum7(uint8_t(a_))) {
            return false;
        }
        if (c_ > 0x1ffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(uint8_t(a_))) | (static_cast<uint64_t>(c_) << 7));
        return true;
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
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Reserved_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_Reserved_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        output.a_ = (chunk0 >> 0) & 0x7f;
        output.c_ = (chunk0 >> 9) & 0x7fffffffffffff;
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return a_; }

    uint64_t GetC() const { return c_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Reserved_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (a_ > 0x7f) {
            return false;
        }
        if (c_ > 0x7fffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(a_)) | (static_cast<uint64_t>(c_) << 9));
        return true;
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
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Size_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_Size_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        size_t b_size_ = (chunk0 >> 0) & 0x7;
        output.a_ = (chunk0 >> 3) & 0x1fffffffffffffff;
        if (span.size() < b_size_) {
            return std::nullopt;
        }
        if ((b_size_ % 1) != 0) {
            return std::nullopt;
        }
        output.b_ = span.subrange(0, b_size_);
        span.skip(b_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint64_t GetA() const { return a_; }

    std::vector<uint8_t> GetB() const {
        pdl::packet::slice span = b_;
        std::vector<uint8_t> elements;
        while (span.size() > 0 && span.size() >= 1) {
            elements.push_back(span.read_be<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Size_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t b_size = 1 * b_.size();
        if (b_size > 0x7) {
            return false;
        }
        if (a_ > 0x1fffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(b_size)) | (static_cast<uint64_t>(a_) << 3));
        for (auto const& element : b_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 8 + 1 * b_.size();
    }

    std::string ToString() const { return ""; }

    uint64_t a_{0};
    std::vector<uint8_t> b_;
};

class Packet_Count_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Count_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_Count_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        output.b_count_ = (chunk0 >> 0) & 0x7;
        output.a_ = (chunk0 >> 3) & 0x1fffffffffffffff;
        if (span.size() < 1 * output.b_count_) {
            return std::nullopt;
        }
        output.b_ = span.subrange(0, 1 * output.b_count_);
        span.skip(1 * output.b_count_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint64_t GetA() const { return a_; }

    std::vector<uint8_t> GetB() const {
        pdl::packet::slice span = b_;
        std::vector<uint8_t> elements;
        while (elements.size() < b_count_ && span.size() >= 1) {
            elements.push_back(span.read_be<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Count_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (b_.size() > 0x7) {
            return false;
        }
        if (a_ > 0x1fffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(b_.size())) | (static_cast<uint64_t>(a_) << 3));
        for (auto const& element : b_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 8 + 1 * b_.size();
    }

    std::string ToString() const { return ""; }

    uint64_t a_{0};
    std::vector<uint8_t> b_;
};

class Packet_FixedScalar_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_FixedScalar_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_FixedScalar_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        if (static_cast<uint64_t>((chunk0 >> 0) & 0x7f) != 0x7) {
            return std::nullopt;
        }
        output.b_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint64_t GetB() const { return b_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_FixedScalar_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (b_ > 0x1ffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(0x7)) | (static_cast<uint64_t>(b_) << 7));
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint64_t b_{0};
};

class Packet_FixedEnum_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_FixedEnum_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_FixedEnum_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        if (Enum7((chunk0 >> 0) & 0x7f) != Enum7::A) {
            return std::nullopt;
        }
        output.b_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint64_t GetB() const { return b_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_FixedEnum_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (b_ > 0x1ffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(Enum7::A)) | (static_cast<uint64_t>(b_) << 7));
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint64_t b_{0};
};

class Packet_Enum_Field_PackedView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Enum_Field_PackedView> Decode(pdl::packet::slice const& parent) {
        Packet_Enum_Field_PackedView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return std::nullopt;
        }
        uint16_t chunk0 = span.read_be<uint16_t, 2>();
        auto raw_a = (chunk0 >> 0) & 0x7f;
        if (!IsValidEnum7(raw_a)) {
           return std::nullopt;
        }
        output.a_ = Enum7(raw_a);
        auto raw_b = (chunk0 >> 7) & 0x7f;
        if (!IsValidEnum7(raw_b)) {
           return std::nullopt;
        }
        output.b_ = Enum7(raw_b);
        output.c_ = (chunk0 >> 14) & 0x3;
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum7 GetA() const { return a_; }

    Enum7 GetB() const { return b_; }

    uint8_t GetC() const { return c_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Enum_Field_PackedView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    Enum7 a_{Enum7::A};
    Enum7 b_{Enum7::A};
    uint8_t c_;


};

class Packet_Enum_Field_PackedBuilder : public pdl::packet::Builder {
public:
    ~Packet_Enum_Field_PackedBuilder() override = default;
    Packet_Enum_Field_PackedBuilder() = default;
    explicit Packet_Enum_Field_PackedBuilder(Enum7 a, Enum7 b, uint8_t c) : a_(std::move(a)), b_(std::move(b)), c_(std::move(c)) {}
    Packet_Enum_Field_PackedBuilder(Packet_Enum_Field_PackedBuilder const&) = default;
    Packet_Enum_Field_PackedBuilder(Packet_Enum_Field_PackedBuilder&&) = default;
    Packet_Enum_Field_PackedBuilder& operator=(Packet_Enum_Field_PackedBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (uint8_t(a_) > 0x7f) {
            return false;
        }
        if (!IsValidEnum7(uint8_t(a_))) {
            return false;
        }
        if (uint8_t(b_) > 0x7f) {
            return false;
        }
        if (!IsValidEnum7(uint8_t(b_))) {
            return false;
        }
        if (c_ > 0x3) {
            return false;
        }
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(uint8_t(a_))) | (static_cast<uint16_t>(uint8_t(b_)) << 7) | (static_cast<uint16_t>(c_) << 14));
        return true;
    }

    size_t GetSize() const override {
        return 2;
    }

    std::string ToString() const { return ""; }

    Enum7 a_{Enum7::A};
    Enum7 b_{Enum7::A};
    uint8_t c_{0};
};

class Packet_Payload_Field_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Payload_Field_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Payload_Field_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t payload_size_ = (chunk0 >> 0) & 0x7;
        if (span.size() < payload_size_) return std::nullopt;
        output.payload_ = span.subrange(0, payload_size_);
        span.skip(payload_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Payload_Field_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t Packet_Payload_Field_VariableSize_payload_size = payload_.size();
        if (Packet_Payload_Field_VariableSize_payload_size > 0x7) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(Packet_Payload_Field_VariableSize_payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
        return true;
    }

    size_t GetSize() const override {
        return 1 + payload_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> payload_;
};

class Packet_Payload_Field_SizeModifierView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Payload_Field_SizeModifierView> Decode(pdl::packet::slice const& parent) {
        Packet_Payload_Field_SizeModifierView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t payload_size_ = (chunk0 >> 0) & 0x7;
        if (span.size() < (payload_size_ - 2)) return std::nullopt;
        output.payload_ = span.subrange(0, (payload_size_ - 2));
        span.skip((payload_size_ - 2));
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Payload_Field_SizeModifierView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t Packet_Payload_Field_SizeModifier_payload_size = payload_.size();
        Packet_Payload_Field_SizeModifier_payload_size = Packet_Payload_Field_SizeModifier_payload_size +2;
        if (Packet_Payload_Field_SizeModifier_payload_size > 0x7) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(Packet_Payload_Field_SizeModifier_payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
        return true;
    }

    size_t GetSize() const override {
        return 1 + payload_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> payload_;
};

class Packet_Payload_Field_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Payload_Field_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Payload_Field_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 2) return std::nullopt;
        size_t payload_size = span.size() - 2;
        output.payload_ = span.subrange(0, payload_size);
        span.skip(payload_size);
        if (span.size() < 2) {
            return std::nullopt;
        }
        output.a_ = span.read_be<uint16_t, 2>();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    uint16_t GetA() const { return a_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Payload_Field_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        output.insert(output.end(), payload_.begin(), payload_.end());
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(a_)));
        return true;
    }

    size_t GetSize() const override {
        return 2 + payload_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> payload_;
    uint16_t a_{0};
};

class Packet_Payload_Field_UnknownSize_TerminalView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Payload_Field_UnknownSize_TerminalView> Decode(pdl::packet::slice const& parent) {
        Packet_Payload_Field_UnknownSize_TerminalView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return std::nullopt;
        }
        output.a_ = span.read_be<uint16_t, 2>();
        output.payload_ = span;
        span.clear();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint16_t GetA() const { return a_; }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Payload_Field_UnknownSize_TerminalView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(a_)));
        output.insert(output.end(), payload_.begin(), payload_.end());
        return true;
    }

    size_t GetSize() const override {
        return 2 + payload_.size();
    }

    std::string ToString() const { return ""; }

    uint16_t a_{0};
    std::vector<uint8_t> payload_;
};

class Packet_Body_Field_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Body_Field_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Body_Field_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t payload_size_ = (chunk0 >> 0) & 0x7;
        if (span.size() < payload_size_) return std::nullopt;
        output.payload_ = span.subrange(0, payload_size_);
        span.skip(payload_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Body_Field_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t Packet_Body_Field_VariableSize_payload_size = payload_.size();
        if (Packet_Body_Field_VariableSize_payload_size > 0x7) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(Packet_Body_Field_VariableSize_payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
        return true;
    }

    size_t GetSize() const override {
        return 1 + payload_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> payload_;
};

class Packet_Body_Field_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Body_Field_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Body_Field_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 2) return std::nullopt;
        size_t payload_size = span.size() - 2;
        output.payload_ = span.subrange(0, payload_size);
        span.skip(payload_size);
        if (span.size() < 2) {
            return std::nullopt;
        }
        output.a_ = span.read_be<uint16_t, 2>();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    uint16_t GetA() const { return a_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Body_Field_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        output.insert(output.end(), payload_.begin(), payload_.end());
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(a_)));
        return true;
    }

    size_t GetSize() const override {
        return 2 + payload_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> payload_;
    uint16_t a_{0};
};

class Packet_Body_Field_UnknownSize_TerminalView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Body_Field_UnknownSize_TerminalView> Decode(pdl::packet::slice const& parent) {
        Packet_Body_Field_UnknownSize_TerminalView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return std::nullopt;
        }
        output.a_ = span.read_be<uint16_t, 2>();
        output.payload_ = span;
        span.clear();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint16_t GetA() const { return a_; }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Body_Field_UnknownSize_TerminalView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(a_)));
        output.insert(output.end(), payload_.begin(), payload_.end());
        return true;
    }

    size_t GetSize() const override {
        return 2 + payload_.size();
    }

    std::string ToString() const { return ""; }

    uint16_t a_{0};
    std::vector<uint8_t> payload_;
};

class Packet_ScalarGroup_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_ScalarGroup_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_ScalarGroup_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return std::nullopt;
        }
        if (static_cast<uint64_t>(span.read_be<uint16_t, 2>()) != 0x2a) {
            return std::nullopt;
        }
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }


    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_ScalarGroup_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;



};

class Packet_ScalarGroup_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_ScalarGroup_FieldBuilder() override = default;
    Packet_ScalarGroup_FieldBuilder() = default;
    Packet_ScalarGroup_FieldBuilder(Packet_ScalarGroup_FieldBuilder const&) = default;
    Packet_ScalarGroup_FieldBuilder(Packet_ScalarGroup_FieldBuilder&&) = default;
    Packet_ScalarGroup_FieldBuilder& operator=(Packet_ScalarGroup_FieldBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(0x2a)));
        return true;
    }

    size_t GetSize() const override {
        return 2;
    }

    std::string ToString() const { return ""; }


};

class Packet_EnumGroup_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_EnumGroup_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_EnumGroup_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return std::nullopt;
        }
        if (Enum16(span.read_be<uint16_t, 2>()) != Enum16::A) {
            return std::nullopt;
        }
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }


    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_EnumGroup_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;



};

class Packet_EnumGroup_FieldBuilder : public pdl::packet::Builder {
public:
    ~Packet_EnumGroup_FieldBuilder() override = default;
    Packet_EnumGroup_FieldBuilder() = default;
    Packet_EnumGroup_FieldBuilder(Packet_EnumGroup_FieldBuilder const&) = default;
    Packet_EnumGroup_FieldBuilder(Packet_EnumGroup_FieldBuilder&&) = default;
    Packet_EnumGroup_FieldBuilder& operator=(Packet_EnumGroup_FieldBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(Enum16::A)));
        return true;
    }

    size_t GetSize() const override {
        return 2;
    }

    std::string ToString() const { return ""; }


};

class Packet_Struct_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Struct_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_Struct_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto a_value = SizedStruct::Decode(span);
        if (!a_value.has_value()) return std::nullopt;
        output.a_ = std::move(*a_value);
        auto b_value = UnsizedStruct::Decode(span);
        if (!b_value.has_value()) return std::nullopt;
        output.b_ = std::move(*b_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    SizedStruct const& GetA() const { return a_; }

    UnsizedStruct const& GetB() const { return b_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Struct_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!a_.Serialize(output)) {
            return false;
        }
        if (!b_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + b_.GetSize();
    }

    std::string ToString() const { return ""; }

    SizedStruct a_;
    UnsizedStruct b_;
};

class Packet_Array_Field_ByteElement_ConstantSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_ByteElement_ConstantSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_ByteElement_ConstantSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 4) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, 4);
        span.skip(4);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::array<uint8_t, 4> GetArray() const {
        pdl::packet::slice span = array_;
        std::array<uint8_t, 4> elements;
        for (int n = 0; n < 4; n++) {
            elements[n] = span.read_be<uint8_t, 1>();
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ByteElement_ConstantSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    std::array<uint8_t, 4> array_;
};

class Packet_Array_Field_ByteElement_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_ByteElement_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_ByteElement_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if ((array_size_ % 1) != 0) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<uint8_t> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<uint8_t> elements;
        while (span.size() > 0 && span.size() >= 1) {
            elements.push_back(span.read_be<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ByteElement_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t array_size = 1 * array_.size();
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> array_;
};

class Packet_Array_Field_ByteElement_VariableCountView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_ByteElement_VariableCountView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_ByteElement_VariableCountView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_count_ = (chunk0 >> 0) & 0xf;
        if (span.size() < 1 * output.array_count_) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, 1 * output.array_count_);
        span.skip(1 * output.array_count_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<uint8_t> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<uint8_t> elements;
        while (elements.size() < array_count_ && span.size() >= 1) {
            elements.push_back(span.read_be<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ByteElement_VariableCountView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (array_.size() > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> array_;
};

class Packet_Array_Field_ByteElement_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_ByteElement_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_ByteElement_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        if ((span.size() % 1) != 0) {
            return std::nullopt;
        }
        output.array_ = span;
        span.clear();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<uint8_t> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<uint8_t> elements;
        while (span.size() > 0 && span.size() >= 1) {
            elements.push_back(span.read_be<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ByteElement_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> array_;
};

class Packet_Array_Field_ScalarElement_ConstantSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_ScalarElement_ConstantSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_ScalarElement_ConstantSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, 8);
        span.skip(8);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::array<uint16_t, 4> GetArray() const {
        pdl::packet::slice span = array_;
        std::array<uint16_t, 4> elements;
        for (int n = 0; n < 4; n++) {
            elements[n] = span.read_be<uint16_t, 2>();
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ScalarElement_ConstantSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    std::array<uint16_t, 4> array_;
};

class Packet_Array_Field_ScalarElement_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_ScalarElement_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_ScalarElement_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if ((array_size_ % 2) != 0) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<uint16_t> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<uint16_t> elements;
        while (span.size() > 0 && span.size() >= 2) {
            elements.push_back(span.read_be<uint16_t, 2>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ScalarElement_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t array_size = 2 * array_.size();
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint16_t> array_;
};

class Packet_Array_Field_ScalarElement_VariableCountView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_ScalarElement_VariableCountView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_ScalarElement_VariableCountView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_count_ = (chunk0 >> 0) & 0xf;
        if (span.size() < 2 * output.array_count_) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, 2 * output.array_count_);
        span.skip(2 * output.array_count_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<uint16_t> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<uint16_t> elements;
        while (elements.size() < array_count_ && span.size() >= 2) {
            elements.push_back(span.read_be<uint16_t, 2>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ScalarElement_VariableCountView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (array_.size() > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint16_t> array_;
};

class Packet_Array_Field_ScalarElement_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_ScalarElement_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_ScalarElement_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        if ((span.size() % 2) != 0) {
            return std::nullopt;
        }
        output.array_ = span;
        span.clear();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<uint16_t> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<uint16_t> elements;
        while (span.size() > 0 && span.size() >= 2) {
            elements.push_back(span.read_be<uint16_t, 2>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_ScalarElement_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint16_t> array_;
};

class Packet_Array_Field_EnumElement_ConstantSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_EnumElement_ConstantSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_EnumElement_ConstantSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 8) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, 8);
        span.skip(8);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::array<Enum16, 4> GetArray() const {
        pdl::packet::slice span = array_;
        std::array<Enum16, 4> elements;
        for (int n = 0; n < 4; n++) {
            elements[n] = Enum16(span.read_be<uint16_t, 2>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_EnumElement_ConstantSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    std::array<Enum16, 4> array_;
};

class Packet_Array_Field_EnumElement_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_EnumElement_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_EnumElement_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if ((array_size_ % 2) != 0) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<Enum16> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<Enum16> elements;
        while (span.size() > 0 && span.size() >= 2) {
            elements.push_back(Enum16(span.read_be<uint16_t, 2>()));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_EnumElement_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t array_size = 2 * array_.size();
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<Enum16> array_;
};

class Packet_Array_Field_EnumElement_VariableCountView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_EnumElement_VariableCountView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_EnumElement_VariableCountView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_count_ = (chunk0 >> 0) & 0xf;
        if (span.size() < 2 * output.array_count_) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, 2 * output.array_count_);
        span.skip(2 * output.array_count_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<Enum16> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<Enum16> elements;
        while (elements.size() < array_count_ && span.size() >= 2) {
            elements.push_back(Enum16(span.read_be<uint16_t, 2>()));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_EnumElement_VariableCountView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (array_.size() > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<Enum16> array_;
};

class Packet_Array_Field_EnumElement_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_EnumElement_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_EnumElement_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        if ((span.size() % 2) != 0) {
            return std::nullopt;
        }
        output.array_ = span;
        span.clear();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<Enum16> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<Enum16> elements;
        while (span.size() > 0 && span.size() >= 2) {
            elements.push_back(Enum16(span.read_be<uint16_t, 2>()));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_EnumElement_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<Enum16> array_;
};

class Packet_Array_Field_SizedElement_ConstantSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_SizedElement_ConstantSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_SizedElement_ConstantSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 4) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, 4);
        span.skip(4);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::array<SizedStruct, 4> GetArray() const {
        pdl::packet::slice span = array_;
        std::array<SizedStruct, 4> elements;
        for (int n = 0; n < 4; n++) {
            if (auto element = SizedStruct::Decode(span)) elements[n] = std::move(*element);
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_SizedElement_ConstantSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    std::array<SizedStruct, 4> array_;
};

class Packet_Array_Field_SizedElement_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_SizedElement_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_SizedElement_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if ((array_size_ % 1) != 0) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<SizedStruct> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<SizedStruct> elements;
        while (span.size() > 0) {
            auto element = SizedStruct::Decode(span);
            if (!element.has_value()) break;
            elements.emplace_back(std::move(*element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_SizedElement_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t array_size = 1 * array_.size();
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<SizedStruct> array_;
};

class Packet_Array_Field_SizedElement_VariableCountView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_SizedElement_VariableCountView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_SizedElement_VariableCountView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_count_ = (chunk0 >> 0) & 0xf;
        if (span.size() < 1 * output.array_count_) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, 1 * output.array_count_);
        span.skip(1 * output.array_count_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<SizedStruct> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<SizedStruct> elements;
        while (elements.size() < array_count_) {
            auto element = SizedStruct::Decode(span);
            if (!element.has_value()) break;
            elements.emplace_back(std::move(*element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_SizedElement_VariableCountView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (array_.size() > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<SizedStruct> array_;
};

class Packet_Array_Field_SizedElement_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_SizedElement_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_SizedElement_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        if ((span.size() % 1) != 0) {
            return std::nullopt;
        }
        output.array_ = span;
        span.clear();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<SizedStruct> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<SizedStruct> elements;
        while (span.size() > 0) {
            auto element = SizedStruct::Decode(span);
            if (!element.has_value()) break;
            elements.emplace_back(std::move(*element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_SizedElement_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<SizedStruct> array_;
};

class Packet_Array_Field_UnsizedElement_ConstantSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_UnsizedElement_ConstantSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_UnsizedElement_ConstantSizeView output(parent);
        pdl::packet::slice span = parent;
        output.array_ = span;
        for (size_t n = 0; n < 4; n++) {
            if (!UnsizedStruct::Decode(span).has_value()) {
                return std::nullopt;
            }
        }
        output.array_ = output.array_.subrange(0, output.array_.size() - span.size());
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::array<UnsizedStruct, 4> GetArray() const {
        pdl::packet::slice span = array_;
        std::array<UnsizedStruct, 4> elements;
        for (int n = 0; n < 4; n++) {
            if (auto element = UnsizedStruct::Decode(span)) elements[n] = std::move(*element);
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_ConstantSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::array<UnsizedStruct, 4> array_;
};

class Packet_Array_Field_UnsizedElement_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_UnsizedElement_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_UnsizedElement_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<UnsizedStruct> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<UnsizedStruct> elements;
        while (span.size() > 0) {
            auto element = UnsizedStruct::Decode(span);
            if (!element.has_value()) break;
            elements.emplace_back(std::move(*element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t array_size = 0;
        for (auto const& elt : array_) {
            array_size += elt.GetSize();
        }
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<UnsizedStruct> array_;
};

class Packet_Array_Field_UnsizedElement_VariableCountView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_UnsizedElement_VariableCountView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_UnsizedElement_VariableCountView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_count_ = (chunk0 >> 0) & 0xf;
        output.array_ = span;
        for (size_t n = 0; n < output.array_count_; n++) {
            if (!UnsizedStruct::Decode(span).has_value()) {
                return std::nullopt;
            }
        }
        output.array_ = output.array_.subrange(0, output.array_.size() - span.size());
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<UnsizedStruct> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<UnsizedStruct> elements;
        while (elements.size() < array_count_) {
            auto element = UnsizedStruct::Decode(span);
            if (!element.has_value()) break;
            elements.emplace_back(std::move(*element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_VariableCountView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (array_.size() > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<UnsizedStruct> array_;
};

class Packet_Array_Field_UnsizedElement_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_UnsizedElement_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_UnsizedElement_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        output.array_ = span;
        while (span.size() > 0) {
            if (!UnsizedStruct::Decode(span).has_value()) {
                return std::nullopt;
            }
        }
        output.array_ = output.array_.subrange(0, output.array_.size() - span.size());
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<UnsizedStruct> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<UnsizedStruct> elements;
        while (span.size() > 0) {
            auto element = UnsizedStruct::Decode(span);
            if (!element.has_value()) break;
            elements.emplace_back(std::move(*element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<UnsizedStruct> array_;
};

class Packet_Array_Field_UnsizedElement_SizeModifierView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_UnsizedElement_SizeModifierView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_UnsizedElement_SizeModifierView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        array_size_ = array_size_ - +2;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<UnsizedStruct> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<UnsizedStruct> elements;
        while (span.size() > 0) {
            auto element = UnsizedStruct::Decode(span);
            if (!element.has_value()) break;
            elements.emplace_back(std::move(*element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_SizeModifierView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t array_size = 0;
        for (auto const& elt : array_) {
            array_size += elt.GetSize();
        }
        array_size = array_size +2;
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<UnsizedStruct> array_;
};

class Packet_Array_Field_SizedElement_VariableSize_PaddedView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_SizedElement_VariableSize_PaddedView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_SizedElement_VariableSize_PaddedView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        size_t array_start_size = span.size();
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if ((array_size_ % 2) != 0) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        if (array_start_size - span.size() < 16) {
            if (span.size() < 16 - (array_start_size - span.size())) return std::nullopt;
            span.skip(16 - (array_start_size - span.size()));
        }
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<uint16_t> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<uint16_t> elements;
        while (span.size() > 0 && span.size() >= 2) {
            elements.push_back(span.read_be<uint16_t, 2>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_SizedElement_VariableSize_PaddedView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t array_size = 2 * array_.size();
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        size_t array_start = output.size();
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        if (output.size() - array_start > 16) {
            return false;
        }
        if (output.size() - array_start < 16) {
            output.resize(array_start + 16, 0);
        }
        return true;
    }

    size_t GetSize() const override {
        return 17;
    }

    std::string ToString() const { return ""; }

    std::vector<uint16_t> array_;
};

class Packet_Array_Field_UnsizedElement_VariableCount_PaddedView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_UnsizedElement_VariableCount_PaddedView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_UnsizedElement_VariableCount_PaddedView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        output.array_count_ = span.read_be<uint8_t, 1>();
        size_t array_start_size = span.size();
        output.array_ = span;
        for (size_t n = 0; n < output.array_count_; n++) {
            if (!UnsizedStruct::Decode(span).has_value()) {
                return std::nullopt;
            }
        }
        output.array_ = output.array_.subrange(0, output.array_.size() - span.size());
        if (array_start_size - span.size() < 16) {
            if (span.size() < 16 - (array_start_size - span.size())) return std::nullopt;
            span.skip(16 - (array_start_size - span.size()));
        }
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<UnsizedStruct> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<UnsizedStruct> elements;
        while (elements.size() < array_count_) {
            auto element = UnsizedStruct::Decode(span);
            if (!element.has_value()) break;
            elements.emplace_back(std::move(*element));
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_UnsizedElement_VariableCount_PaddedView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (array_.size() > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        size_t array_start = output.size();
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        if (output.size() - array_start > 16) {
            return false;
        }
        if (output.size() - array_start < 16) {
            output.resize(array_start + 16, 0);
        }
        return true;
    }

    size_t GetSize() const override {
        return 17;
    }

    std::string ToString() const { return ""; }

    std::vector<UnsizedStruct> array_;
};

class Packet_Array_Field_VariableElementSize_ConstantSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_VariableElementSize_ConstantSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_VariableElementSize_ConstantSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_element_size_ = (chunk0 >> 0) & 0xf;
        if (output.array_element_size_ == 0) {
            return std::nullopt;
        }
        if (span.size() < output.array_element_size_ * 4) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, output.array_element_size_ * 4);
        span.skip(output.array_element_size_ * 4);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::array<UnknownSizeStruct, 4> GetArray() const {
        pdl::packet::slice span = array_;
        std::array<UnknownSizeStruct, 4> elements;
        for (int n = 0; n < 4; n++) {
            auto element_span = span.subrange(0, array_element_size_);
            if (auto element = UnknownSizeStruct::Decode(element_span)) elements[n] = std::move(*element);
            span.skip(array_element_size_);
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_VariableElementSize_ConstantSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    uint8_t array_element_size_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_VariableElementSize_ConstantSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_VariableElementSize_ConstantSizeBuilder() override = default;
    Packet_Array_Field_VariableElementSize_ConstantSizeBuilder() = default;
    explicit Packet_Array_Field_VariableElementSize_ConstantSizeBuilder(std::array<UnknownSizeStruct, 4> array) : array_(std::move(array)) {}
    Packet_Array_Field_VariableElementSize_ConstantSizeBuilder(Packet_Array_Field_VariableElementSize_ConstantSizeBuilder const&) = default;
    Packet_Array_Field_VariableElementSize_ConstantSizeBuilder(Packet_Array_Field_VariableElementSize_ConstantSizeBuilder&&) = default;
    Packet_Array_Field_VariableElementSize_ConstantSizeBuilder& operator=(Packet_Array_Field_VariableElementSize_ConstantSizeBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t array_element_size = array_.empty() ? 0 : array_[0].GetSize();
        for (auto const& elt : array_) {
            if (elt.GetSize() != array_element_size) {
                return false;
            }
        }
        if (array_element_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_element_size)));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::array<UnknownSizeStruct, 4> array_;
};

class Packet_Array_Field_VariableElementSize_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_VariableElementSize_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_VariableElementSize_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        uint8_t chunk1 = span.read_be<uint8_t, 1>();
        output.array_element_size_ = (chunk1 >> 0) & 0xf;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if (output.array_element_size_ == 0 || (array_size_ % output.array_element_size_) != 0) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, array_size_);
        span.skip(array_size_);
        if ((span.size() % 1) != 0) {
            return std::nullopt;
        }
        output.tail_ = span;
        span.clear();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<UnknownSizeStruct> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<UnknownSizeStruct> elements;
        while (span.size() > 0) {
            auto element_span = span.subrange(0, array_element_size_);
            auto element = UnknownSizeStruct::Decode(element_span);
            if (!element.has_value()) break;
            elements.emplace_back(std::move(*element));
            span.skip(array_element_size_);
        }
        return elements;
    }

    std::vector<uint8_t> GetTail() const {
        pdl::packet::slice span = tail_;
        std::vector<uint8_t> elements;
        while (span.size() > 0 && span.size() >= 1) {
            elements.push_back(span.read_be<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_VariableElementSize_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    uint8_t array_element_size_ {0};
    pdl::packet::slice array_;
    pdl::packet::slice tail_;


};

class Packet_Array_Field_VariableElementSize_VariableSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_VariableElementSize_VariableSizeBuilder() override = default;
    Packet_Array_Field_VariableElementSize_VariableSizeBuilder() = default;
    explicit Packet_Array_Field_VariableElementSize_VariableSizeBuilder(std::vector<UnknownSizeStruct> array, std::vector<uint8_t> tail) : array_(std::move(array)), tail_(std::move(tail)) {}
    Packet_Array_Field_VariableElementSize_VariableSizeBuilder(Packet_Array_Field_VariableElementSize_VariableSizeBuilder const&) = default;
    Packet_Array_Field_VariableElementSize_VariableSizeBuilder(Packet_Array_Field_VariableElementSize_VariableSizeBuilder&&) = default;
    Packet_Array_Field_VariableElementSize_VariableSizeBuilder& operator=(Packet_Array_Field_VariableElementSize_VariableSizeBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t array_size = 0;
        for (auto const& elt : array_) {
            array_size += elt.GetSize();
        }
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        size_t array_element_size = array_.empty() ? 0 : array_[0].GetSize();
        for (auto const& elt : array_) {
            if (elt.GetSize() != array_element_size) {
                return false;
            }
        }
        if (array_element_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_element_size)));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        for (auto const& element : tail_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 2 + std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); }) + 1 * tail_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<UnknownSizeStruct> array_;
    std::vector<uint8_t> tail_;
};

class Packet_Array_Field_VariableElementSize_VariableCountView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_VariableElementSize_VariableCountView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_VariableElementSize_VariableCountView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 2) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_count_ = (chunk0 >> 0) & 0xf;
        uint8_t chunk1 = span.read_be<uint8_t, 1>();
        output.array_element_size_ = (chunk1 >> 0) & 0xf;
        if (output.array_element_size_ == 0) {
            return std::nullopt;
        }
        if (span.size() < output.array_element_size_ * output.array_count_) {
            return std::nullopt;
        }
        output.array_ = span.subrange(0, output.array_element_size_ * output.array_count_);
        span.skip(output.array_element_size_ * output.array_count_);
        if ((span.size() % 1) != 0) {
            return std::nullopt;
        }
        output.tail_ = span;
        span.clear();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<UnknownSizeStruct> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<UnknownSizeStruct> elements;
        while (elements.size() < array_count_) {
            auto element_span = span.subrange(0, array_element_size_);
            auto element = UnknownSizeStruct::Decode(element_span);
            if (!element.has_value()) break;
            elements.emplace_back(std::move(*element));
            span.skip(array_element_size_);
        }
        return elements;
    }

    std::vector<uint8_t> GetTail() const {
        pdl::packet::slice span = tail_;
        std::vector<uint8_t> elements;
        while (span.size() > 0 && span.size() >= 1) {
            elements.push_back(span.read_be<uint8_t, 1>());
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_VariableElementSize_VariableCountView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    uint8_t array_count_ {0};
    uint8_t array_element_size_ {0};
    pdl::packet::slice array_;
    pdl::packet::slice tail_;


};

class Packet_Array_Field_VariableElementSize_VariableCountBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_VariableElementSize_VariableCountBuilder() override = default;
    Packet_Array_Field_VariableElementSize_VariableCountBuilder() = default;
    explicit Packet_Array_Field_VariableElementSize_VariableCountBuilder(std::vector<UnknownSizeStruct> array, std::vector<uint8_t> tail) : array_(std::move(array)), tail_(std::move(tail)) {}
    Packet_Array_Field_VariableElementSize_VariableCountBuilder(Packet_Array_Field_VariableElementSize_VariableCountBuilder const&) = default;
    Packet_Array_Field_VariableElementSize_VariableCountBuilder(Packet_Array_Field_VariableElementSize_VariableCountBuilder&&) = default;
    Packet_Array_Field_VariableElementSize_VariableCountBuilder& operator=(Packet_Array_Field_VariableElementSize_VariableCountBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (array_.size() > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        size_t array_element_size = array_.empty() ? 0 : array_[0].GetSize();
        for (auto const& elt : array_) {
            if (elt.GetSize() != array_element_size) {
                return false;
            }
        }
        if (array_element_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_element_size)));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        for (auto const& element : tail_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 2 + std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); }) + 1 * tail_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<UnknownSizeStruct> array_;
    std::vector<uint8_t> tail_;
};

class Packet_Array_Field_VariableElementSize_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Array_Field_VariableElementSize_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Packet_Array_Field_VariableElementSize_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_element_size_ = (chunk0 >> 0) & 0xf;
        if (output.array_element_size_ == 0 || (span.size() % output.array_element_size_) != 0) {
            return std::nullopt;
        }
        output.array_ = span;
        span.clear();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::vector<UnknownSizeStruct> GetArray() const {
        pdl::packet::slice span = array_;
        std::vector<UnknownSizeStruct> elements;
        while (span.size() > 0) {
            auto element_span = span.subrange(0, array_element_size_);
            auto element = UnknownSizeStruct::Decode(element_span);
            if (!element.has_value()) break;
            elements.emplace_back(std::move(*element));
            span.skip(array_element_size_);
        }
        return elements;
    }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Array_Field_VariableElementSize_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    uint8_t array_element_size_ {0};
    pdl::packet::slice array_;


};

class Packet_Array_Field_VariableElementSize_UnknownSizeBuilder : public pdl::packet::Builder {
public:
    ~Packet_Array_Field_VariableElementSize_UnknownSizeBuilder() override = default;
    Packet_Array_Field_VariableElementSize_UnknownSizeBuilder() = default;
    explicit Packet_Array_Field_VariableElementSize_UnknownSizeBuilder(std::vector<UnknownSizeStruct> array) : array_(std::move(array)) {}
    Packet_Array_Field_VariableElementSize_UnknownSizeBuilder(Packet_Array_Field_VariableElementSize_UnknownSizeBuilder const&) = default;
    Packet_Array_Field_VariableElementSize_UnknownSizeBuilder(Packet_Array_Field_VariableElementSize_UnknownSizeBuilder&&) = default;
    Packet_Array_Field_VariableElementSize_UnknownSizeBuilder& operator=(Packet_Array_Field_VariableElementSize_UnknownSizeBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        size_t array_element_size = array_.empty() ? 0 : array_[0].GetSize();
        for (auto const& elt : array_) {
            if (elt.GetSize() != array_element_size) {
                return false;
            }
        }
        if (array_element_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_element_size)));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<UnknownSizeStruct> array_;
};

class Packet_Optional_Scalar_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Optional_Scalar_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_Optional_Scalar_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            if (span.size() < 3) {
                return std::nullopt;
            }
            output.a_ = span.read_be<uint32_t, 3>();
        }
        if (c1 == 1) {
            if (span.size() < 4) {
                return std::nullopt;
            }
            output.b_ = span.read_be<uint32_t, 4>();
        }
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::optional<uint32_t> GetA() const { return a_; }

    std::optional<uint32_t> GetB() const { return b_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Optional_Scalar_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        uint8_t c0 = a_.has_value() ? 0 : 1;
        if (c0 != (a_.has_value() ? 0 : 1)) {
            return false;
        }
        uint8_t c1 = b_.has_value() ? 1 : 0;
        if (c1 != (b_.has_value() ? 1 : 0)) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(c0)) | (static_cast<uint8_t>(c1) << 1));
        if (a_.has_value()) {
            if (*a_ > 0xffffff) {
                return false;
            }
            pdl::packet::Builder::write_be<uint32_t, 3>(output, *a_);
        }
        if (b_.has_value()) {
            pdl::packet::Builder::write_be<uint32_t, 4>(output, *b_);
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + (a_.has_value() ? 3 : 0) + (b_.has_value() ? 4 : 0);
    }

    std::string ToString() const { return ""; }

    std::optional<uint32_t> a_;
    std::optional<uint32_t> b_;
};

class Packet_Optional_Enum_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Optional_Enum_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_Optional_Enum_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            if (span.size() < 2) {
                return std::nullopt;
            }
            auto raw_value = span.read_be<uint16_t, 2>();
            if (!IsValidEnum16(raw_value)) {
               return std::nullopt;
            }
            output.a_ = Enum16(raw_value);
        }
        if (c1 == 1) {
            if (span.size() < 2) {
                return std::nullopt;
            }
            auto raw_value = span.read_be<uint16_t, 2>();
            if (!IsValidEnum16(raw_value)) {
               return std::nullopt;
            }
            output.b_ = Enum16(raw_value);
        }
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::optional<Enum16> GetA() const { return a_; }

    std::optional<Enum16> GetB() const { return b_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Optional_Enum_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        uint8_t c0 = a_.has_value() ? 0 : 1;
        if (c0 != (a_.has_value() ? 0 : 1)) {
            return false;
        }
        uint8_t c1 = b_.has_value() ? 1 : 0;
        if (c1 != (b_.has_value() ? 1 : 0)) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(c0)) | (static_cast<uint8_t>(c1) << 1));
        if (a_.has_value()) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(*a_));
        }
        if (b_.has_value()) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(*b_));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + (a_.has_value() ? 2 : 0) + (b_.has_value() ? 2 : 0);
    }

    std::string ToString() const { return ""; }

    std::optional<Enum16> a_;
    std::optional<Enum16> b_;
};

class Packet_Optional_Struct_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Packet_Optional_Struct_FieldView> Decode(pdl::packet::slice const& parent) {
        Packet_Optional_Struct_FieldView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            output.a_ = SizedStruct::Decode(span);
            if (!output.a_.has_value()) {
                return std::nullopt;
            }
        }
        if (c1 == 1) {
            output.b_ = UnsizedStruct::Decode(span);
            if (!output.b_.has_value()) {
                return std::nullopt;
            }
        }
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    std::optional<SizedStruct> GetA() const { return a_; }

    std::optional<UnsizedStruct> GetB() const { return b_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Packet_Optional_Struct_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        uint8_t c0 = a_.has_value() ? 0 : 1;
        if (c0 != (a_.has_value() ? 0 : 1)) {
            return false;
        }
        uint8_t c1 = b_.has_value() ? 1 : 0;
        if (c1 != (b_.has_value() ? 1 : 0)) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(c0)) | (static_cast<uint8_t>(c1) << 1));
        if (a_.has_value()) {
            if (!a_->Serialize(output)) {
                return false;
            }
        }
        if (b_.has_value()) {
            if (!b_->Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + (a_.has_value() ? 1 : 0) + (b_.has_value() ? b_->GetSize() : 0);
    }

    std::string ToString() const { return ""; }

    std::optional<SizedStruct> a_;
    std::optional<UnsizedStruct> b_;
};

class ScalarChild_AView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<ScalarChild_AView> Decode(ScalarParentView const& parent) {
        ScalarChild_AView output(parent.bytes_);
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 1) {
            return std::nullopt;
        }
        output.b_ = span.read_be<uint8_t, 1>();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return 0; }

    uint8_t GetB() const { return b_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ScalarChild_AView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(0)));
        size_t ScalarParent_payload_size = 1;
        if (ScalarParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(ScalarParent_payload_size)));
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(b_)));
        return true;
    }

    size_t GetSize() const override {
        return 3;
    }

    std::string ToString() const { return ""; }

    uint8_t b_{0};
};

class ScalarChild_BView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<ScalarChild_BView> Decode(ScalarParentView const& parent) {
        ScalarChild_BView output(parent.bytes_);
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 2) {
            return std::nullopt;
        }
        output.c_ = span.read_be<uint16_t, 2>();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return 1; }

    uint16_t GetC() const { return c_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ScalarChild_BView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(1)));
        size_t ScalarParent_payload_size = 2;
        if (ScalarParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(ScalarParent_payload_size)));
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(c_)));
        return true;
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    uint16_t c_{0};
};

class EnumChild_AView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<EnumChild_AView> Decode(EnumParentView const& parent) {
        EnumChild_AView output(parent.bytes_);
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 1) {
            return std::nullopt;
        }
        output.b_ = span.read_be<uint8_t, 1>();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum16 GetA() const { return Enum16::A; }

    uint8_t GetB() const { return b_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit EnumChild_AView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(Enum16::A)));
        size_t EnumParent_payload_size = 1;
        if (EnumParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(EnumParent_payload_size)));
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(b_)));
        return true;
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    uint8_t b_{0};
};

class EnumChild_BView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<EnumChild_BView> Decode(EnumParentView const& parent) {
        EnumChild_BView output(parent.bytes_);
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 2) {
            return std::nullopt;
        }
        output.c_ = span.read_be<uint16_t, 2>();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum16 GetA() const { return Enum16::B; }

    uint16_t GetC() const { return c_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit EnumChild_BView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(Enum16::B)));
        size_t EnumParent_payload_size = 2;
        if (EnumParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(EnumParent_payload_size)));
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(c_)));
        return true;
    }

    size_t GetSize() const override {
        return 5;
    }

    std::string ToString() const { return ""; }

    uint16_t c_{0};
};

class AliasedChild_AView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<AliasedChild_AView> Decode(EmptyParentView const& parent) {
        AliasedChild_AView output(parent.bytes_);
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 1) {
            return std::nullopt;
        }
        output.b_ = span.read_be<uint8_t, 1>();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return 2; }

    uint8_t GetB() const { return b_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit AliasedChild_AView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(2)));
        size_t ScalarParent_payload_size = 1;
        if (ScalarParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(ScalarParent_payload_size)));
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(b_)));
        return true;
    }

    size_t GetSize() const override {
        return 3;
    }

    std::string ToString() const { return ""; }

    uint8_t b_{0};
};

class AliasedChild_BView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<AliasedChild_BView> Decode(EmptyParentView const& parent) {
        AliasedChild_BView output(parent.bytes_);
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 2) {
            return std::nullopt;
        }
        output.c_ = span.read_be<uint16_t, 2>();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return 3; }

    uint16_t GetC() const { return c_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit AliasedChild_BView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(3)));
        size_t ScalarParent_payload_size = 2;
        if (ScalarParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(ScalarParent_payload_size)));
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(c_)));
        return true;
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    uint16_t c_{0};
};

class ScalarChild_CView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<ScalarChild_CView> Decode(ScalarParentView const& parent) {
        ScalarChild_CView output(parent.bytes_);
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 1) {
            return std::nullopt;
        }
        output.b_ = span.read_be<uint8_t, 1>();
        output.payload_ = span;
        span.clear();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return 4; }

    uint8_t GetB() const { return b_; }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ScalarChild_CView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    uint8_t b_;
    pdl::packet::slice payload_;

    friend class ScalarGrandChild_CView;
};

class ScalarChild_CBuilder : public pdl::packet::Builder {
public:
    ~ScalarChild_CBuilder() override = default;
    ScalarChild_CBuilder() = default;
    explicit ScalarChild_CBuilder(uint8_t b, std::vector<uint8_t> payload) : b_(std::move(b)), payload_(std::move(payload)) {}
    ScalarChild_CBuilder(ScalarChild_CBuilder const&) = default;
    ScalarChild_CBuilder(ScalarChild_CBuilder&&) = default;
    ScalarChild_CBuilder& operator=(ScalarChild_CBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(4)));
        size_t ScalarParent_payload_size = 1 + payload_.size();
        if (ScalarParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(ScalarParent_payload_size)));
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(b_)));
        output.insert(output.end(), payload_.begin(), payload_.end());
        return true;
    }

    size_t GetSize() const override {
        return 3 + payload_.size();
    }

    std::string ToString() const { return ""; }

    uint8_t b_{0};
    std::vector<uint8_t> payload_;
};

class ScalarGrandChild_CView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<ScalarGrandChild_CView> Decode(ScalarChild_CView const& parent) {
        ScalarGrandChild_CView output(parent.bytes_);
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 2) {
            return std::nullopt;
        }
        output.c_ = span.read_be<uint16_t, 2>();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return 4; }

    uint8_t GetB() const { return 1; }

    uint16_t GetC() const { return c_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ScalarGrandChild_CView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    uint16_t c_;


};

class ScalarGrandChild_CBuilder : public pdl::packet::Builder {
public:
    ~ScalarGrandChild_CBuilder() override = default;
    ScalarGrandChild_CBuilder() = default;
    explicit ScalarGrandChild_CBuilder(uint16_t c) : c_(std::move(c)) {}
    ScalarGrandChild_CBuilder(ScalarGrandChild_CBuilder const&) = default;
    ScalarGrandChild_CBuilder(ScalarGrandChild_CBuilder&&) = default;
    ScalarGrandChild_CBuilder& operator=(ScalarGrandChild_CBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(4)));
        size_t ScalarParent_payload_size = 3;
        if (ScalarParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(ScalarParent_payload_size)));
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(1)));
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(c_)));
        return true;
    }

    size_t GetSize() const override {
        return 5;
    }

    std::string ToString() const { return ""; }

    uint16_t c_{0};
};

class ScalarChild_DView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<ScalarChild_DView> Decode(ScalarParentView const& parent) {
        ScalarChild_DView output(parent.bytes_);
        pdl::packet::slice span = parent.payload_;
        if (span.size() < 1) {
            return std::nullopt;
        }
        size_t payload_size_ = span.read_be<uint8_t, 1>();
        if (span.size() < payload_size_) return std::nullopt;
        output.payload_ = span.subrange(0, payload_size_);
        span.skip(payload_size_);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return 5; }

    pdl::packet::slice GetPayload() const {
        return payload_;
    }
    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ScalarChild_DView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    pdl::packet::slice payload_;


};

class ScalarChild_DBuilder : public pdl::packet::Builder {
public:
    ~ScalarChild_DBuilder() override = default;
    ScalarChild_DBuilder() = default;
    explicit ScalarChild_DBuilder(std::vector<uint8_t> payload) : payload_(std::move(payload)) {}
    ScalarChild_DBuilder(ScalarChild_DBuilder const&) = default;
    ScalarChild_DBuilder(ScalarChild_DBuilder&&) = default;
    ScalarChild_DBuilder& operator=(ScalarChild_DBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(5)));
        size_t ScalarParent_payload_size = 1 + payload_.size();
        if (ScalarParent_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(ScalarParent_payload_size)));
        size_t ScalarChild_D_payload_size = payload_.size();
        if (ScalarChild_D_payload_size > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(ScalarChild_D_payload_size)));
        output.insert(output.end(), payload_.begin(), payload_.end());
        return true;
    }

    size_t GetSize() const override {
        return 3 + payload_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> payload_;
};

class ConstrainedParentView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<ConstrainedParentView> Decode(pdl::packet::slice const& parent) {
        ConstrainedParentView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        output.a_ = span.read_be<uint8_t, 1>();
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    uint8_t GetA() const { return a_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ConstrainedParentView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;
    uint8_t a_;

    friend class ConstrainedChild_AView;
    friend class ConstrainedChild_BView;
};

class ConstrainedParentBuilder : public pdl::packet::Builder {
public:
    ~ConstrainedParentBuilder() override = default;
    ConstrainedParentBuilder() = default;
    explicit ConstrainedParentBuilder(uint8_t a) : a_(std::move(a)) {}
    ConstrainedParentBuilder(ConstrainedParentBuilder const&) = default;
    ConstrainedParentBuilder(ConstrainedParentBuilder&&) = default;
    ConstrainedParentBuilder& operator=(ConstrainedParentBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(a_)));
        return true;
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    uint8_t a_{0};
};

class ConstrainedChild_AView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<ConstrainedChild_AView> Decode(ConstrainedParentView const& parent) {
        ConstrainedChild_AView output(parent.bytes_);

        return output;
    }

    uint8_t GetA() const { return 1; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ConstrainedChild_AView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;



};

class ConstrainedChild_ABuilder : public pdl::packet::Builder {
public:
    ~ConstrainedChild_ABuilder() override = default;
    ConstrainedChild_ABuilder() = default;
    ConstrainedChild_ABuilder(ConstrainedChild_ABuilder const&) = default;
    ConstrainedChild_ABuilder(ConstrainedChild_ABuilder&&) = default;
    ConstrainedChild_ABuilder& operator=(ConstrainedChild_ABuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(1)));
        return true;
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }


};

class ConstrainedChild_BView {
public:
    // Decode the packet from the parent packet view.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<ConstrainedChild_BView> Decode(ConstrainedParentView const& parent) {
        ConstrainedChild_BView output(parent.bytes_);

        return output;
    }

    uint8_t GetA() const { return 2; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit ConstrainedChild_BView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

    pdl::packet::slice bytes_;



};

class ConstrainedChild_BBuilder : public pdl::packet::Builder {
public:
    ~ConstrainedChild_BBuilder() override = default;
    ConstrainedChild_BBuilder() = default;
    ConstrainedChild_BBuilder(ConstrainedChild_BBuilder const&) = default;
    ConstrainedChild_BBuilder(ConstrainedChild_BBuilder&&) = default;
    ConstrainedChild_BBuilder& operator=(ConstrainedChild_BBuilder const&) = default;

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(2)));
        return true;
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }


};

class Struct_Scalar_Field : public pdl::packet::Builder {
public:
    ~Struct_Scalar_Field() override = default;
    Struct_Scalar_Field() = default;
    Struct_Scalar_Field(Struct_Scalar_Field const&) = default;
    Struct_Scalar_Field(Struct_Scalar_Field&&) = default;
    explicit Struct_Scalar_Field(uint8_t a, uint64_t c) : a_(std::move(a)), c_(std::move(c)) {}
    Struct_Scalar_Field& operator=(Struct_Scalar_Field const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Scalar_Field> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Scalar_Field output;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        output.a_ = (chunk0 >> 0) & 0x7f;
        output.c_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (a_ > 0x7f) {
            return false;
        }
        if (c_ > 0x1ffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(a_)) | (static_cast<uint64_t>(c_) << 7));
        return true;
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
    Struct_Enum_Field_(Struct_Enum_Field_ const&) = default;
    Struct_Enum_Field_(Struct_Enum_Field_&&) = default;
    explicit Struct_Enum_Field_(Enum7 a, uint64_t c) : a_(std::move(a)), c_(std::move(c)) {}
    Struct_Enum_Field_& operator=(Struct_Enum_Field_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Enum_Field_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Enum_Field_ output;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        auto raw_a = (chunk0 >> 0) & 0x7f;
        if (!IsValidEnum7(raw_a)) {
           return std::nullopt;
        }
        output.a_ = Enum7(raw_a);
        output.c_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (uint8_t(a_) > 0x7f) {
            return false;
        }
        if (!IsValidEnum7(uint8_t(a_))) {
            return false;
        }
        if (c_ > 0x1ffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(uint8_t(a_))) | (static_cast<uint64_t>(c_) << 7));
        return true;
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
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Enum_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_Enum_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Enum_Field_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Enum_Field_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Enum_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    Struct_Enum_Field_ s_;
};

class Struct_Reserved_Field_ : public pdl::packet::Builder {
public:
    ~Struct_Reserved_Field_() override = default;
    Struct_Reserved_Field_() = default;
    Struct_Reserved_Field_(Struct_Reserved_Field_ const&) = default;
    Struct_Reserved_Field_(Struct_Reserved_Field_&&) = default;
    explicit Struct_Reserved_Field_(uint8_t a, uint64_t c) : a_(std::move(a)), c_(std::move(c)) {}
    Struct_Reserved_Field_& operator=(Struct_Reserved_Field_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Reserved_Field_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Reserved_Field_ output;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        output.a_ = (chunk0 >> 0) & 0x7f;
        output.c_ = (chunk0 >> 9) & 0x7fffffffffffff;
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (a_ > 0x7f) {
            return false;
        }
        if (c_ > 0x7fffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(a_)) | (static_cast<uint64_t>(c_) << 9));
        return true;
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
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Reserved_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_Reserved_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Reserved_Field_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Reserved_Field_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Reserved_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    Struct_Reserved_Field_ s_;
};

class Struct_Size_Field_ : public pdl::packet::Builder {
public:
    ~Struct_Size_Field_() override = default;
    Struct_Size_Field_() = default;
    Struct_Size_Field_(Struct_Size_Field_ const&) = default;
    Struct_Size_Field_(Struct_Size_Field_&&) = default;
    explicit Struct_Size_Field_(uint64_t a, std::vector<uint8_t> b) : a_(std::move(a)), b_(std::move(b)) {}
    Struct_Size_Field_& operator=(Struct_Size_Field_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Size_Field_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Size_Field_ output;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        size_t b_size_ = (chunk0 >> 0) & 0x7;
        output.a_ = (chunk0 >> 3) & 0x1fffffffffffffff;
        if (span.size() < b_size_) {
            return std::nullopt;
        }
        if ((b_size_ % 1) != 0) {
            return std::nullopt;
        }
        auto b_count_ = b_size_ / 1;
        for (size_t n = 0; n < b_count_; n++) {
            output.b_.push_back(span.read_be<uint8_t, 1>());
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        size_t b_size = 1 * b_.size();
        if (b_size > 0x7) {
            return false;
        }
        if (a_ > 0x1fffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(b_size)) | (static_cast<uint64_t>(a_) << 3));
        for (auto const& element : b_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 8 + 1 * b_.size();
    }

    std::string ToString() const { return ""; }

    uint64_t a_{0};
    std::vector<uint8_t> b_;
};

class Struct_Size_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Size_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_Size_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Size_Field_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Size_Field_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Size_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Count_Field_(Struct_Count_Field_ const&) = default;
    Struct_Count_Field_(Struct_Count_Field_&&) = default;
    explicit Struct_Count_Field_(uint64_t a, std::vector<uint8_t> b) : a_(std::move(a)), b_(std::move(b)) {}
    Struct_Count_Field_& operator=(Struct_Count_Field_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Count_Field_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Count_Field_ output;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        output.b_count_ = (chunk0 >> 0) & 0x7;
        output.a_ = (chunk0 >> 3) & 0x1fffffffffffffff;
        if (span.size() < 1 * output.b_count_) {
            return std::nullopt;
        }
        for (size_t n = 0; n < output.b_count_; n++) {
            output.b_.push_back(span.read_be<uint8_t, 1>());
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (b_.size() > 0x7) {
            return false;
        }
        if (a_ > 0x1fffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(b_.size())) | (static_cast<uint64_t>(a_) << 3));
        for (auto const& element : b_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 8 + 1 * b_.size();
    }

    std::string ToString() const { return ""; }

    uint8_t b_count_ {0};
    uint64_t a_{0};
    std::vector<uint8_t> b_;
};

class Struct_Count_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Count_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_Count_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Count_Field_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Count_Field_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Count_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_FixedScalar_Field_(Struct_FixedScalar_Field_ const&) = default;
    Struct_FixedScalar_Field_(Struct_FixedScalar_Field_&&) = default;
    explicit Struct_FixedScalar_Field_(uint64_t b) : b_(std::move(b)) {}
    Struct_FixedScalar_Field_& operator=(Struct_FixedScalar_Field_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_FixedScalar_Field_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_FixedScalar_Field_ output;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        if (static_cast<uint64_t>((chunk0 >> 0) & 0x7f) != 0x7) {
            return std::nullopt;
        }
        output.b_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (b_ > 0x1ffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(0x7)) | (static_cast<uint64_t>(b_) << 7));
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint64_t b_{0};
};

class Struct_FixedScalar_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_FixedScalar_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_FixedScalar_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_FixedScalar_Field_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_FixedScalar_Field_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_FixedScalar_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    Struct_FixedScalar_Field_ s_;
};

class Struct_FixedEnum_Field_ : public pdl::packet::Builder {
public:
    ~Struct_FixedEnum_Field_() override = default;
    Struct_FixedEnum_Field_() = default;
    Struct_FixedEnum_Field_(Struct_FixedEnum_Field_ const&) = default;
    Struct_FixedEnum_Field_(Struct_FixedEnum_Field_&&) = default;
    explicit Struct_FixedEnum_Field_(uint64_t b) : b_(std::move(b)) {}
    Struct_FixedEnum_Field_& operator=(Struct_FixedEnum_Field_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_FixedEnum_Field_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_FixedEnum_Field_ output;
        if (span.size() < 8) {
            return std::nullopt;
        }
        uint64_t chunk0 = span.read_be<uint64_t, 8>();
        if (Enum7((chunk0 >> 0) & 0x7f) != Enum7::A) {
            return std::nullopt;
        }
        output.b_ = (chunk0 >> 7) & 0x1ffffffffffffff;
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (b_ > 0x1ffffffffffffff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint64_t, 8>(output, (static_cast<uint64_t>(Enum7::A)) | (static_cast<uint64_t>(b_) << 7));
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    uint64_t b_{0};
};

class Struct_FixedEnum_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_FixedEnum_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_FixedEnum_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_FixedEnum_Field_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_FixedEnum_Field_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_FixedEnum_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 8;
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

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_ScalarGroup_Field_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_ScalarGroup_Field_ output;
        if (span.size() < 2) {
            return std::nullopt;
        }
        if (static_cast<uint64_t>(span.read_be<uint16_t, 2>()) != 0x2a) {
            return std::nullopt;
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(0x2a)));
        return true;
    }

    size_t GetSize() const override {
        return 2;
    }

    std::string ToString() const { return ""; }


};

class Struct_ScalarGroup_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_ScalarGroup_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_ScalarGroup_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_ScalarGroup_Field_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_ScalarGroup_Field_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_ScalarGroup_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 2;
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

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_EnumGroup_Field_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_EnumGroup_Field_ output;
        if (span.size() < 2) {
            return std::nullopt;
        }
        if (Enum16(span.read_be<uint16_t, 2>()) != Enum16::A) {
            return std::nullopt;
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        pdl::packet::Builder::write_be<uint16_t, 2>(output, (static_cast<uint16_t>(Enum16::A)));
        return true;
    }

    size_t GetSize() const override {
        return 2;
    }

    std::string ToString() const { return ""; }


};

class Struct_EnumGroup_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_EnumGroup_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_EnumGroup_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_EnumGroup_Field_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_EnumGroup_Field_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_EnumGroup_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 2;
    }

    std::string ToString() const { return ""; }

    Struct_EnumGroup_Field_ s_;
};

class Struct_Struct_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Struct_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_Struct_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto a_value = SizedStruct::Decode(span);
        if (!a_value.has_value()) return std::nullopt;
        output.a_ = std::move(*a_value);
        auto b_value = UnsizedStruct::Decode(span);
        if (!b_value.has_value()) return std::nullopt;
        output.b_ = std::move(*b_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    SizedStruct const& GetA() const { return a_; }

    UnsizedStruct const& GetB() const { return b_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Struct_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!a_.Serialize(output)) {
            return false;
        }
        if (!b_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + b_.GetSize();
    }

    std::string ToString() const { return ""; }

    SizedStruct a_;
    UnsizedStruct b_;
};

class Struct_Array_Field_ByteElement_ConstantSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ByteElement_ConstantSize_() override = default;
    Struct_Array_Field_ByteElement_ConstantSize_() = default;
    Struct_Array_Field_ByteElement_ConstantSize_(Struct_Array_Field_ByteElement_ConstantSize_ const&) = default;
    Struct_Array_Field_ByteElement_ConstantSize_(Struct_Array_Field_ByteElement_ConstantSize_&&) = default;
    explicit Struct_Array_Field_ByteElement_ConstantSize_(std::array<uint8_t, 4> array) : array_(std::move(array)) {}
    Struct_Array_Field_ByteElement_ConstantSize_& operator=(Struct_Array_Field_ByteElement_ConstantSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_ByteElement_ConstantSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_ByteElement_ConstantSize_ output;
        if (span.size() < 4) {
            return std::nullopt;
        }
        for (size_t n = 0; n < 4; n++) {
            output.array_[n] = span.read_be<uint8_t, 1>();
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    std::array<uint8_t, 4> array_;
};

class Struct_Array_Field_ByteElement_ConstantSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_ByteElement_ConstantSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_ByteElement_ConstantSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_ByteElement_ConstantSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_ByteElement_ConstantSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ByteElement_ConstantSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_ByteElement_ConstantSize_ s_;
};

class Struct_Array_Field_ByteElement_VariableSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ByteElement_VariableSize_() override = default;
    Struct_Array_Field_ByteElement_VariableSize_() = default;
    Struct_Array_Field_ByteElement_VariableSize_(Struct_Array_Field_ByteElement_VariableSize_ const&) = default;
    Struct_Array_Field_ByteElement_VariableSize_(Struct_Array_Field_ByteElement_VariableSize_&&) = default;
    explicit Struct_Array_Field_ByteElement_VariableSize_(std::vector<uint8_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ByteElement_VariableSize_& operator=(Struct_Array_Field_ByteElement_VariableSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_ByteElement_VariableSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_ByteElement_VariableSize_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if ((array_size_ % 1) != 0) {
            return std::nullopt;
        }
        auto array_count_ = array_size_ / 1;
        for (size_t n = 0; n < array_count_; n++) {
            output.array_.push_back(span.read_be<uint8_t, 1>());
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = 1 * array_.size();
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> array_;
};

class Struct_Array_Field_ByteElement_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_ByteElement_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_ByteElement_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_ByteElement_VariableSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_ByteElement_VariableSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ByteElement_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_ByteElement_VariableCount_(Struct_Array_Field_ByteElement_VariableCount_ const&) = default;
    Struct_Array_Field_ByteElement_VariableCount_(Struct_Array_Field_ByteElement_VariableCount_&&) = default;
    explicit Struct_Array_Field_ByteElement_VariableCount_(std::vector<uint8_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ByteElement_VariableCount_& operator=(Struct_Array_Field_ByteElement_VariableCount_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_ByteElement_VariableCount_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_ByteElement_VariableCount_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_count_ = (chunk0 >> 0) & 0xf;
        if (span.size() < 1 * output.array_count_) {
            return std::nullopt;
        }
        for (size_t n = 0; n < output.array_count_; n++) {
            output.array_.push_back(span.read_be<uint8_t, 1>());
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (array_.size() > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<uint8_t> array_;
};

class Struct_Array_Field_ByteElement_VariableCountView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_ByteElement_VariableCountView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_ByteElement_VariableCountView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_ByteElement_VariableCount_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_ByteElement_VariableCount_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ByteElement_VariableCountView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_ByteElement_UnknownSize_(Struct_Array_Field_ByteElement_UnknownSize_ const&) = default;
    Struct_Array_Field_ByteElement_UnknownSize_(Struct_Array_Field_ByteElement_UnknownSize_&&) = default;
    explicit Struct_Array_Field_ByteElement_UnknownSize_(std::vector<uint8_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ByteElement_UnknownSize_& operator=(Struct_Array_Field_ByteElement_UnknownSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_ByteElement_UnknownSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_ByteElement_UnknownSize_ output;
        if ((span.size() % 1) != 0) {
            return std::nullopt;
        }
        auto array_count_ = span.size() / 1;
        for (size_t n = 0; n < array_count_; n++) {
            output.array_.push_back(span.read_be<uint8_t, 1>());
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint8_t, 1>(output, uint8_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint8_t> array_;
};

class Struct_Array_Field_ByteElement_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_ByteElement_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_ByteElement_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_ByteElement_UnknownSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_ByteElement_UnknownSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ByteElement_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_ScalarElement_ConstantSize_(Struct_Array_Field_ScalarElement_ConstantSize_ const&) = default;
    Struct_Array_Field_ScalarElement_ConstantSize_(Struct_Array_Field_ScalarElement_ConstantSize_&&) = default;
    explicit Struct_Array_Field_ScalarElement_ConstantSize_(std::array<uint16_t, 4> array) : array_(std::move(array)) {}
    Struct_Array_Field_ScalarElement_ConstantSize_& operator=(Struct_Array_Field_ScalarElement_ConstantSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_ScalarElement_ConstantSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_ScalarElement_ConstantSize_ output;
        if (span.size() < 8) {
            return std::nullopt;
        }
        for (size_t n = 0; n < 4; n++) {
            output.array_[n] = span.read_be<uint16_t, 2>();
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    std::array<uint16_t, 4> array_;
};

class Struct_Array_Field_ScalarElement_ConstantSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_ScalarElement_ConstantSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_ScalarElement_ConstantSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_ScalarElement_ConstantSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_ScalarElement_ConstantSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ScalarElement_ConstantSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_ScalarElement_ConstantSize_ s_;
};

class Struct_Array_Field_ScalarElement_VariableSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_ScalarElement_VariableSize_() override = default;
    Struct_Array_Field_ScalarElement_VariableSize_() = default;
    Struct_Array_Field_ScalarElement_VariableSize_(Struct_Array_Field_ScalarElement_VariableSize_ const&) = default;
    Struct_Array_Field_ScalarElement_VariableSize_(Struct_Array_Field_ScalarElement_VariableSize_&&) = default;
    explicit Struct_Array_Field_ScalarElement_VariableSize_(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ScalarElement_VariableSize_& operator=(Struct_Array_Field_ScalarElement_VariableSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_ScalarElement_VariableSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_ScalarElement_VariableSize_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if ((array_size_ % 2) != 0) {
            return std::nullopt;
        }
        auto array_count_ = array_size_ / 2;
        for (size_t n = 0; n < array_count_; n++) {
            output.array_.push_back(span.read_be<uint16_t, 2>());
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = 2 * array_.size();
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint16_t> array_;
};

class Struct_Array_Field_ScalarElement_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_ScalarElement_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_ScalarElement_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_ScalarElement_VariableSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_ScalarElement_VariableSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ScalarElement_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_ScalarElement_VariableCount_(Struct_Array_Field_ScalarElement_VariableCount_ const&) = default;
    Struct_Array_Field_ScalarElement_VariableCount_(Struct_Array_Field_ScalarElement_VariableCount_&&) = default;
    explicit Struct_Array_Field_ScalarElement_VariableCount_(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ScalarElement_VariableCount_& operator=(Struct_Array_Field_ScalarElement_VariableCount_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_ScalarElement_VariableCount_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_ScalarElement_VariableCount_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_count_ = (chunk0 >> 0) & 0xf;
        if (span.size() < 2 * output.array_count_) {
            return std::nullopt;
        }
        for (size_t n = 0; n < output.array_count_; n++) {
            output.array_.push_back(span.read_be<uint16_t, 2>());
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (array_.size() > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<uint16_t> array_;
};

class Struct_Array_Field_ScalarElement_VariableCountView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_ScalarElement_VariableCountView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_ScalarElement_VariableCountView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_ScalarElement_VariableCount_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_ScalarElement_VariableCount_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ScalarElement_VariableCountView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_ScalarElement_UnknownSize_(Struct_Array_Field_ScalarElement_UnknownSize_ const&) = default;
    Struct_Array_Field_ScalarElement_UnknownSize_(Struct_Array_Field_ScalarElement_UnknownSize_&&) = default;
    explicit Struct_Array_Field_ScalarElement_UnknownSize_(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_ScalarElement_UnknownSize_& operator=(Struct_Array_Field_ScalarElement_UnknownSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_ScalarElement_UnknownSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_ScalarElement_UnknownSize_ output;
        if ((span.size() % 2) != 0) {
            return std::nullopt;
        }
        auto array_count_ = span.size() / 2;
        for (size_t n = 0; n < array_count_; n++) {
            output.array_.push_back(span.read_be<uint16_t, 2>());
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<uint16_t> array_;
};

class Struct_Array_Field_ScalarElement_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_ScalarElement_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_ScalarElement_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_ScalarElement_UnknownSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_ScalarElement_UnknownSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_ScalarElement_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_EnumElement_ConstantSize_(Struct_Array_Field_EnumElement_ConstantSize_ const&) = default;
    Struct_Array_Field_EnumElement_ConstantSize_(Struct_Array_Field_EnumElement_ConstantSize_&&) = default;
    explicit Struct_Array_Field_EnumElement_ConstantSize_(std::array<Enum16, 4> array) : array_(std::move(array)) {}
    Struct_Array_Field_EnumElement_ConstantSize_& operator=(Struct_Array_Field_EnumElement_ConstantSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_EnumElement_ConstantSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_EnumElement_ConstantSize_ output;
        if (span.size() < 8) {
            return std::nullopt;
        }
        for (size_t n = 0; n < 4; n++) {
            auto raw_value = span.read_be<uint16_t, 2>();
            if (!IsValidEnum16(raw_value)) {
                return std::nullopt;
            }
            output.array_[n] = Enum16(raw_value);
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    std::array<Enum16, 4> array_;
};

class Struct_Array_Field_EnumElement_ConstantSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_EnumElement_ConstantSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_EnumElement_ConstantSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_EnumElement_ConstantSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_EnumElement_ConstantSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_EnumElement_ConstantSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 8;
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_EnumElement_ConstantSize_ s_;
};

class Struct_Array_Field_EnumElement_VariableSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_EnumElement_VariableSize_() override = default;
    Struct_Array_Field_EnumElement_VariableSize_() = default;
    Struct_Array_Field_EnumElement_VariableSize_(Struct_Array_Field_EnumElement_VariableSize_ const&) = default;
    Struct_Array_Field_EnumElement_VariableSize_(Struct_Array_Field_EnumElement_VariableSize_&&) = default;
    explicit Struct_Array_Field_EnumElement_VariableSize_(std::vector<Enum16> array) : array_(std::move(array)) {}
    Struct_Array_Field_EnumElement_VariableSize_& operator=(Struct_Array_Field_EnumElement_VariableSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_EnumElement_VariableSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_EnumElement_VariableSize_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if ((array_size_ % 2) != 0) {
            return std::nullopt;
        }
        auto array_count_ = array_size_ / 2;
        for (size_t n = 0; n < array_count_; n++) {
            auto raw_value = span.read_be<uint16_t, 2>();
            if (!IsValidEnum16(raw_value)) {
                return std::nullopt;
            }
            output.array_.push_back(Enum16(raw_value));
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = 2 * array_.size();
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<Enum16> array_;
};

class Struct_Array_Field_EnumElement_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_EnumElement_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_EnumElement_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_EnumElement_VariableSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_EnumElement_VariableSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_EnumElement_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_EnumElement_VariableCount_(Struct_Array_Field_EnumElement_VariableCount_ const&) = default;
    Struct_Array_Field_EnumElement_VariableCount_(Struct_Array_Field_EnumElement_VariableCount_&&) = default;
    explicit Struct_Array_Field_EnumElement_VariableCount_(std::vector<Enum16> array) : array_(std::move(array)) {}
    Struct_Array_Field_EnumElement_VariableCount_& operator=(Struct_Array_Field_EnumElement_VariableCount_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_EnumElement_VariableCount_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_EnumElement_VariableCount_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_count_ = (chunk0 >> 0) & 0xf;
        if (span.size() < 2 * output.array_count_) {
            return std::nullopt;
        }
        for (size_t n = 0; n < output.array_count_; n++) {
            auto raw_value = span.read_be<uint16_t, 2>();
            if (!IsValidEnum16(raw_value)) {
                return std::nullopt;
            }
            output.array_.push_back(Enum16(raw_value));
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (array_.size() > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<Enum16> array_;
};

class Struct_Array_Field_EnumElement_VariableCountView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_EnumElement_VariableCountView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_EnumElement_VariableCountView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_EnumElement_VariableCount_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_EnumElement_VariableCount_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_EnumElement_VariableCountView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_EnumElement_UnknownSize_(Struct_Array_Field_EnumElement_UnknownSize_ const&) = default;
    Struct_Array_Field_EnumElement_UnknownSize_(Struct_Array_Field_EnumElement_UnknownSize_&&) = default;
    explicit Struct_Array_Field_EnumElement_UnknownSize_(std::vector<Enum16> array) : array_(std::move(array)) {}
    Struct_Array_Field_EnumElement_UnknownSize_& operator=(Struct_Array_Field_EnumElement_UnknownSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_EnumElement_UnknownSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_EnumElement_UnknownSize_ output;
        if ((span.size() % 2) != 0) {
            return std::nullopt;
        }
        auto array_count_ = span.size() / 2;
        for (size_t n = 0; n < array_count_; n++) {
            auto raw_value = span.read_be<uint16_t, 2>();
            if (!IsValidEnum16(raw_value)) {
                return std::nullopt;
            }
            output.array_.push_back(Enum16(raw_value));
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        return true;
    }

    size_t GetSize() const override {
        return 2 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<Enum16> array_;
};

class Struct_Array_Field_EnumElement_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_EnumElement_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_EnumElement_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_EnumElement_UnknownSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_EnumElement_UnknownSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_EnumElement_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_SizedElement_ConstantSize_(Struct_Array_Field_SizedElement_ConstantSize_ const&) = default;
    Struct_Array_Field_SizedElement_ConstantSize_(Struct_Array_Field_SizedElement_ConstantSize_&&) = default;
    explicit Struct_Array_Field_SizedElement_ConstantSize_(std::array<SizedStruct, 4> array) : array_(std::move(array)) {}
    Struct_Array_Field_SizedElement_ConstantSize_& operator=(Struct_Array_Field_SizedElement_ConstantSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_SizedElement_ConstantSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_SizedElement_ConstantSize_ output;
        if (span.size() < 4) {
            return std::nullopt;
        }
        for (size_t n = 0; n < 4; n++) {
            auto out = SizedStruct::Decode(span);
            if (!out.has_value()) {
                return std::nullopt;
            }
            output.array_[n] = std::move(*out);
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    std::array<SizedStruct, 4> array_;
};

class Struct_Array_Field_SizedElement_ConstantSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_SizedElement_ConstantSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_SizedElement_ConstantSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_SizedElement_ConstantSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_SizedElement_ConstantSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_SizedElement_ConstantSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 4;
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_SizedElement_ConstantSize_ s_;
};

class Struct_Array_Field_SizedElement_VariableSize_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_SizedElement_VariableSize_() override = default;
    Struct_Array_Field_SizedElement_VariableSize_() = default;
    Struct_Array_Field_SizedElement_VariableSize_(Struct_Array_Field_SizedElement_VariableSize_ const&) = default;
    Struct_Array_Field_SizedElement_VariableSize_(Struct_Array_Field_SizedElement_VariableSize_&&) = default;
    explicit Struct_Array_Field_SizedElement_VariableSize_(std::vector<SizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_SizedElement_VariableSize_& operator=(Struct_Array_Field_SizedElement_VariableSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_SizedElement_VariableSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_SizedElement_VariableSize_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if ((array_size_ % 1) != 0) {
            return std::nullopt;
        }
        auto array_count_ = array_size_ / 1;
        for (size_t n = 0; n < array_count_; n++) {
            auto out = SizedStruct::Decode(span);
            if (!out.has_value()) {
                return std::nullopt;
            }
            output.array_.push_back(std::move(*out));
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = 1 * array_.size();
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<SizedStruct> array_;
};

class Struct_Array_Field_SizedElement_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_SizedElement_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_SizedElement_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_SizedElement_VariableSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_SizedElement_VariableSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_SizedElement_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_SizedElement_VariableCount_(Struct_Array_Field_SizedElement_VariableCount_ const&) = default;
    Struct_Array_Field_SizedElement_VariableCount_(Struct_Array_Field_SizedElement_VariableCount_&&) = default;
    explicit Struct_Array_Field_SizedElement_VariableCount_(std::vector<SizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_SizedElement_VariableCount_& operator=(Struct_Array_Field_SizedElement_VariableCount_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_SizedElement_VariableCount_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_SizedElement_VariableCount_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_count_ = (chunk0 >> 0) & 0xf;
        if (span.size() < 1 * output.array_count_) {
            return std::nullopt;
        }
        for (size_t n = 0; n < output.array_count_; n++) {
            auto out = SizedStruct::Decode(span);
            if (!out.has_value()) {
                return std::nullopt;
            }
            output.array_.push_back(std::move(*out));
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (array_.size() > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<SizedStruct> array_;
};

class Struct_Array_Field_SizedElement_VariableCountView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_SizedElement_VariableCountView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_SizedElement_VariableCountView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_SizedElement_VariableCount_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_SizedElement_VariableCount_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_SizedElement_VariableCountView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_SizedElement_UnknownSize_(Struct_Array_Field_SizedElement_UnknownSize_ const&) = default;
    Struct_Array_Field_SizedElement_UnknownSize_(Struct_Array_Field_SizedElement_UnknownSize_&&) = default;
    explicit Struct_Array_Field_SizedElement_UnknownSize_(std::vector<SizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_SizedElement_UnknownSize_& operator=(Struct_Array_Field_SizedElement_UnknownSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_SizedElement_UnknownSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_SizedElement_UnknownSize_ output;
        if ((span.size() % 1) != 0) {
            return std::nullopt;
        }
        auto array_count_ = span.size() / 1;
        for (size_t n = 0; n < array_count_; n++) {
            auto out = SizedStruct::Decode(span);
            if (!out.has_value()) {
                return std::nullopt;
            }
            output.array_.push_back(std::move(*out));
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 * array_.size();
    }

    std::string ToString() const { return ""; }

    std::vector<SizedStruct> array_;
};

class Struct_Array_Field_SizedElement_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_SizedElement_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_SizedElement_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_SizedElement_UnknownSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_SizedElement_UnknownSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_SizedElement_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_UnsizedElement_ConstantSize_(Struct_Array_Field_UnsizedElement_ConstantSize_ const&) = default;
    Struct_Array_Field_UnsizedElement_ConstantSize_(Struct_Array_Field_UnsizedElement_ConstantSize_&&) = default;
    explicit Struct_Array_Field_UnsizedElement_ConstantSize_(std::array<UnsizedStruct, 4> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_ConstantSize_& operator=(Struct_Array_Field_UnsizedElement_ConstantSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_UnsizedElement_ConstantSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_UnsizedElement_ConstantSize_ output;
        for (size_t n = 0; n < 4; n++) {
            auto out = UnsizedStruct::Decode(span);
            if (!out.has_value()) {
                return std::nullopt;
            }
            output.array_[n] = std::move(*out);
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::array<UnsizedStruct, 4> array_;
};

class Struct_Array_Field_UnsizedElement_ConstantSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_UnsizedElement_ConstantSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_UnsizedElement_ConstantSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_UnsizedElement_ConstantSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_UnsizedElement_ConstantSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_ConstantSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_UnsizedElement_VariableSize_(Struct_Array_Field_UnsizedElement_VariableSize_ const&) = default;
    Struct_Array_Field_UnsizedElement_VariableSize_(Struct_Array_Field_UnsizedElement_VariableSize_&&) = default;
    explicit Struct_Array_Field_UnsizedElement_VariableSize_(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_VariableSize_& operator=(Struct_Array_Field_UnsizedElement_VariableSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_UnsizedElement_VariableSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_UnsizedElement_VariableSize_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        auto array_span = span.subrange(0, array_size_);
        while (array_span.size() > 0) {
            auto out = UnsizedStruct::Decode(array_span);
            if (!out.has_value()) {
                return std::nullopt;
            }
            output.array_.push_back(std::move(*out));
        }
        span.skip(array_size_);
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = 0;
        for (auto const& elt : array_) {
            array_size += elt.GetSize();
        }
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<UnsizedStruct> array_;
};

class Struct_Array_Field_UnsizedElement_VariableSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_UnsizedElement_VariableSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_UnsizedElement_VariableSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_UnsizedElement_VariableSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_UnsizedElement_VariableSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_VariableSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_UnsizedElement_VariableCount_(Struct_Array_Field_UnsizedElement_VariableCount_ const&) = default;
    Struct_Array_Field_UnsizedElement_VariableCount_(Struct_Array_Field_UnsizedElement_VariableCount_&&) = default;
    explicit Struct_Array_Field_UnsizedElement_VariableCount_(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_VariableCount_& operator=(Struct_Array_Field_UnsizedElement_VariableCount_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_UnsizedElement_VariableCount_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_UnsizedElement_VariableCount_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.array_count_ = (chunk0 >> 0) & 0xf;
        for (size_t n = 0; n < output.array_count_; n++) {
            auto out = UnsizedStruct::Decode(span);
            if (!out.has_value()) {
                return std::nullopt;
            }
            output.array_.push_back(std::move(*out));
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (array_.size() > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<UnsizedStruct> array_;
};

class Struct_Array_Field_UnsizedElement_VariableCountView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_UnsizedElement_VariableCountView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_UnsizedElement_VariableCountView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_UnsizedElement_VariableCount_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_UnsizedElement_VariableCount_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_VariableCountView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_UnsizedElement_UnknownSize_(Struct_Array_Field_UnsizedElement_UnknownSize_ const&) = default;
    Struct_Array_Field_UnsizedElement_UnknownSize_(Struct_Array_Field_UnsizedElement_UnknownSize_&&) = default;
    explicit Struct_Array_Field_UnsizedElement_UnknownSize_(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_UnknownSize_& operator=(Struct_Array_Field_UnsizedElement_UnknownSize_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_UnsizedElement_UnknownSize_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_UnsizedElement_UnknownSize_ output;
        while (span.size() > 0) {
            auto out = UnsizedStruct::Decode(span);
            if (!out.has_value()) {
                return std::nullopt;
            }
            output.array_.push_back(std::move(*out));
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<UnsizedStruct> array_;
};

class Struct_Array_Field_UnsizedElement_UnknownSizeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_UnsizedElement_UnknownSizeView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_UnsizedElement_UnknownSizeView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_UnsizedElement_UnknownSize_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_UnsizedElement_UnknownSize_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_UnknownSizeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_UnsizedElement_SizeModifier_(Struct_Array_Field_UnsizedElement_SizeModifier_ const&) = default;
    Struct_Array_Field_UnsizedElement_SizeModifier_(Struct_Array_Field_UnsizedElement_SizeModifier_&&) = default;
    explicit Struct_Array_Field_UnsizedElement_SizeModifier_(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_SizeModifier_& operator=(Struct_Array_Field_UnsizedElement_SizeModifier_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_UnsizedElement_SizeModifier_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_UnsizedElement_SizeModifier_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        array_size_ = array_size_ - +2;
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        auto array_span = span.subrange(0, array_size_);
        while (array_span.size() > 0) {
            auto out = UnsizedStruct::Decode(array_span);
            if (!out.has_value()) {
                return std::nullopt;
            }
            output.array_.push_back(std::move(*out));
        }
        span.skip(array_size_);
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = 0;
        for (auto const& elt : array_) {
            array_size += elt.GetSize();
        }
        array_size = array_size +2;
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + std::accumulate(array_.begin(), array_.end(), size_t(0), [](size_t s, auto const& elt) { return s + elt.GetSize(); });
    }

    std::string ToString() const { return ""; }

    std::vector<UnsizedStruct> array_;
};

class Struct_Array_Field_UnsizedElement_SizeModifierView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_UnsizedElement_SizeModifierView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_UnsizedElement_SizeModifierView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_UnsizedElement_SizeModifier_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_UnsizedElement_SizeModifier_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_SizeModifierView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Array_Field_SizedElement_VariableSize_Padded_(Struct_Array_Field_SizedElement_VariableSize_Padded_ const&) = default;
    Struct_Array_Field_SizedElement_VariableSize_Padded_(Struct_Array_Field_SizedElement_VariableSize_Padded_&&) = default;
    explicit Struct_Array_Field_SizedElement_VariableSize_Padded_(std::vector<uint16_t> array) : array_(std::move(array)) {}
    Struct_Array_Field_SizedElement_VariableSize_Padded_& operator=(Struct_Array_Field_SizedElement_VariableSize_Padded_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_SizedElement_VariableSize_Padded_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_SizedElement_VariableSize_Padded_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        size_t array_size_ = (chunk0 >> 0) & 0xf;
        size_t array_start_size = span.size();
        if (span.size() < array_size_) {
            return std::nullopt;
        }
        if ((array_size_ % 2) != 0) {
            return std::nullopt;
        }
        auto array_count_ = array_size_ / 2;
        for (size_t n = 0; n < array_count_; n++) {
            output.array_.push_back(span.read_be<uint16_t, 2>());
        }
        if (array_start_size - span.size() < 16) {
            if (span.size() < 16 - (array_start_size - span.size())) return std::nullopt;
            span.skip(16 - (array_start_size - span.size()));
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        size_t array_size = 2 * array_.size();
        if (array_size > 0xf) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_size)));
        size_t array_start = output.size();
        for (auto const& element : array_) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(element));
        }
        if (output.size() - array_start > 16) {
            return false;
        }
        if (output.size() - array_start < 16) {
            output.resize(array_start + 16, 0);
        }
        return true;
    }

    size_t GetSize() const override {
        return 17;
    }

    std::string ToString() const { return ""; }

    std::vector<uint16_t> array_;
};

class Struct_Array_Field_SizedElement_VariableSize_PaddedView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_SizedElement_VariableSize_PaddedView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_SizedElement_VariableSize_PaddedView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_SizedElement_VariableSize_Padded_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_SizedElement_VariableSize_Padded_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_SizedElement_VariableSize_PaddedView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 17;
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_SizedElement_VariableSize_Padded_ s_;
};

class Struct_Array_Field_UnsizedElement_VariableCount_Padded_ : public pdl::packet::Builder {
public:
    ~Struct_Array_Field_UnsizedElement_VariableCount_Padded_() override = default;
    Struct_Array_Field_UnsizedElement_VariableCount_Padded_() = default;
    Struct_Array_Field_UnsizedElement_VariableCount_Padded_(Struct_Array_Field_UnsizedElement_VariableCount_Padded_ const&) = default;
    Struct_Array_Field_UnsizedElement_VariableCount_Padded_(Struct_Array_Field_UnsizedElement_VariableCount_Padded_&&) = default;
    explicit Struct_Array_Field_UnsizedElement_VariableCount_Padded_(std::vector<UnsizedStruct> array) : array_(std::move(array)) {}
    Struct_Array_Field_UnsizedElement_VariableCount_Padded_& operator=(Struct_Array_Field_UnsizedElement_VariableCount_Padded_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Array_Field_UnsizedElement_VariableCount_Padded_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Array_Field_UnsizedElement_VariableCount_Padded_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        output.array_count_ = span.read_be<uint8_t, 1>();
        size_t array_start_size = span.size();
        for (size_t n = 0; n < output.array_count_; n++) {
            auto out = UnsizedStruct::Decode(span);
            if (!out.has_value()) {
                return std::nullopt;
            }
            output.array_.push_back(std::move(*out));
        }
        if (array_start_size - span.size() < 16) {
            if (span.size() < 16 - (array_start_size - span.size())) return std::nullopt;
            span.skip(16 - (array_start_size - span.size()));
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        if (array_.size() > 0xff) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(array_.size())));
        size_t array_start = output.size();
        for (auto const& element : array_) {
            if (!element.Serialize(output)) {
                return false;
            }
        }
        if (output.size() - array_start > 16) {
            return false;
        }
        if (output.size() - array_start < 16) {
            output.resize(array_start + 16, 0);
        }
        return true;
    }

    size_t GetSize() const override {
        return 17;
    }

    std::string ToString() const { return ""; }

    uint8_t array_count_ {0};
    std::vector<UnsizedStruct> array_;
};

class Struct_Array_Field_UnsizedElement_VariableCount_PaddedView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Array_Field_UnsizedElement_VariableCount_PaddedView> Decode(pdl::packet::slice const& parent) {
        Struct_Array_Field_UnsizedElement_VariableCount_PaddedView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Array_Field_UnsizedElement_VariableCount_Padded_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Array_Field_UnsizedElement_VariableCount_Padded_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Array_Field_UnsizedElement_VariableCount_PaddedView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
    }

    size_t GetSize() const override {
        return 17;
    }

    std::string ToString() const { return ""; }

    Struct_Array_Field_UnsizedElement_VariableCount_Padded_ s_;
};

class Struct_Optional_Scalar_Field_ : public pdl::packet::Builder {
public:
    ~Struct_Optional_Scalar_Field_() override = default;
    Struct_Optional_Scalar_Field_() = default;
    Struct_Optional_Scalar_Field_(Struct_Optional_Scalar_Field_ const&) = default;
    Struct_Optional_Scalar_Field_(Struct_Optional_Scalar_Field_&&) = default;
    explicit Struct_Optional_Scalar_Field_(std::optional<uint32_t> a, std::optional<uint32_t> b) : a_(std::move(a)), b_(std::move(b)) {}
    Struct_Optional_Scalar_Field_& operator=(Struct_Optional_Scalar_Field_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Optional_Scalar_Field_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Optional_Scalar_Field_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            if (span.size() < 3) {
                return std::nullopt;
            }
            output.a_ = span.read_be<uint32_t, 3>();
        }
        if (c1 == 1) {
            if (span.size() < 4) {
                return std::nullopt;
            }
            output.b_ = span.read_be<uint32_t, 4>();
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        uint8_t c0 = a_.has_value() ? 0 : 1;
        if (c0 != (a_.has_value() ? 0 : 1)) {
            return false;
        }
        uint8_t c1 = b_.has_value() ? 1 : 0;
        if (c1 != (b_.has_value() ? 1 : 0)) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(c0)) | (static_cast<uint8_t>(c1) << 1));
        if (a_.has_value()) {
            if (*a_ > 0xffffff) {
                return false;
            }
            pdl::packet::Builder::write_be<uint32_t, 3>(output, *a_);
        }
        if (b_.has_value()) {
            pdl::packet::Builder::write_be<uint32_t, 4>(output, *b_);
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + (a_.has_value() ? 3 : 0) + (b_.has_value() ? 4 : 0);
    }

    std::string ToString() const { return ""; }

    std::optional<uint32_t> a_;
    std::optional<uint32_t> b_;
};

class Struct_Optional_Scalar_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Optional_Scalar_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_Optional_Scalar_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Optional_Scalar_Field_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Optional_Scalar_Field_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Optional_Scalar_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Optional_Enum_Field_(Struct_Optional_Enum_Field_ const&) = default;
    Struct_Optional_Enum_Field_(Struct_Optional_Enum_Field_&&) = default;
    explicit Struct_Optional_Enum_Field_(std::optional<Enum16> a, std::optional<Enum16> b) : a_(std::move(a)), b_(std::move(b)) {}
    Struct_Optional_Enum_Field_& operator=(Struct_Optional_Enum_Field_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Optional_Enum_Field_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Optional_Enum_Field_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            if (span.size() < 2) {
                return std::nullopt;
            }
            auto raw_value = span.read_be<uint16_t, 2>();
            if (!IsValidEnum16(raw_value)) {
               return std::nullopt;
            }
            output.a_ = Enum16(raw_value);
        }
        if (c1 == 1) {
            if (span.size() < 2) {
                return std::nullopt;
            }
            auto raw_value = span.read_be<uint16_t, 2>();
            if (!IsValidEnum16(raw_value)) {
               return std::nullopt;
            }
            output.b_ = Enum16(raw_value);
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        uint8_t c0 = a_.has_value() ? 0 : 1;
        if (c0 != (a_.has_value() ? 0 : 1)) {
            return false;
        }
        uint8_t c1 = b_.has_value() ? 1 : 0;
        if (c1 != (b_.has_value() ? 1 : 0)) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(c0)) | (static_cast<uint8_t>(c1) << 1));
        if (a_.has_value()) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(*a_));
        }
        if (b_.has_value()) {
            pdl::packet::Builder::write_be<uint16_t, 2>(output, uint16_t(*b_));
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + (a_.has_value() ? 2 : 0) + (b_.has_value() ? 2 : 0);
    }

    std::string ToString() const { return ""; }

    std::optional<Enum16> a_;
    std::optional<Enum16> b_;
};

class Struct_Optional_Enum_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Optional_Enum_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_Optional_Enum_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Optional_Enum_Field_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Optional_Enum_Field_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Optional_Enum_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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
    Struct_Optional_Struct_Field_(Struct_Optional_Struct_Field_ const&) = default;
    Struct_Optional_Struct_Field_(Struct_Optional_Struct_Field_&&) = default;
    explicit Struct_Optional_Struct_Field_(std::optional<SizedStruct> a, std::optional<UnsizedStruct> b) : a_(std::move(a)), b_(std::move(b)) {}
    Struct_Optional_Struct_Field_& operator=(Struct_Optional_Struct_Field_ const&) = default;

    // Decode the struct from the input span.
    // Returns std::nullopt if the struct is invalid, otherwise the input
    // span is updated to the remaining bytes.
    static std::optional<Struct_Optional_Struct_Field_> Decode(pdl::packet::slice& parent_span) {
        pdl::packet::slice span = parent_span;
        Struct_Optional_Struct_Field_ output;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        uint8_t c0 = (chunk0 >> 0) & 0x1;
        uint8_t c1 = (chunk0 >> 1) & 0x1;
        if (c0 == 0) {
            output.a_ = SizedStruct::Decode(span);
            if (!output.a_.has_value()) {
                return std::nullopt;
            }
        }
        if (c1 == 1) {
            output.b_ = UnsizedStruct::Decode(span);
            if (!output.b_.has_value()) {
                return std::nullopt;
            }
        }
        parent_span = span;
        return output;
    }

    bool Serialize(std::vector<uint8_t>& output) const override {
        uint8_t c0 = a_.has_value() ? 0 : 1;
        if (c0 != (a_.has_value() ? 0 : 1)) {
            return false;
        }
        uint8_t c1 = b_.has_value() ? 1 : 0;
        if (c1 != (b_.has_value() ? 1 : 0)) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(c0)) | (static_cast<uint8_t>(c1) << 1));
        if (a_.has_value()) {
            if (!a_->Serialize(output)) {
                return false;
            }
        }
        if (b_.has_value()) {
            if (!b_->Serialize(output)) {
                return false;
            }
        }
        return true;
    }

    size_t GetSize() const override {
        return 1 + (a_.has_value() ? 1 : 0) + (b_.has_value() ? b_->GetSize() : 0);
    }

    std::string ToString() const { return ""; }

    std::optional<SizedStruct> a_;
    std::optional<UnsizedStruct> b_;
};

class Struct_Optional_Struct_FieldView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Struct_Optional_Struct_FieldView> Decode(pdl::packet::slice const& parent) {
        Struct_Optional_Struct_FieldView output(parent);
        pdl::packet::slice span = parent;
        auto s_value = Struct_Optional_Struct_Field_::Decode(span);
        if (!s_value.has_value()) return std::nullopt;
        output.s_ = std::move(*s_value);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Struct_Optional_Struct_Field_ const& GetS() const { return s_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Struct_Optional_Struct_FieldView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (!s_.Serialize(output)) {
            return false;
        }
        return true;
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

inline std::string Enum_Incomplete_Truncated_Closed_ToString(Enum_Incomplete_Truncated_Closed_ tag) {
    switch (tag) {
        case Enum_Incomplete_Truncated_Closed_::A: return "A";
        case Enum_Incomplete_Truncated_Closed_::B: return "B";
        default:
            break;
    }

    return std::string("UNKNOWN(" +
           std::to_string(uint8_t(tag)) + ")");
}

inline bool IsValidEnum_Incomplete_Truncated_Closed_(uint8_t value) {
    switch (value) {
        case 0x0:
        case 0x1:
            return true;
        default:
            return false;
    }
}

class Enum_Incomplete_Truncated_ClosedView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Enum_Incomplete_Truncated_ClosedView> Decode(pdl::packet::slice const& parent) {
        Enum_Incomplete_Truncated_ClosedView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        auto raw_e = (chunk0 >> 0) & 0x7;
        if (!IsValidEnum_Incomplete_Truncated_Closed_(raw_e)) {
           return std::nullopt;
        }
        output.e_ = Enum_Incomplete_Truncated_Closed_(raw_e);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum_Incomplete_Truncated_Closed_ GetE() const { return e_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Incomplete_Truncated_ClosedView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (uint8_t(e_) > 0x7) {
            return false;
        }
        if (!IsValidEnum_Incomplete_Truncated_Closed_(uint8_t(e_))) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(uint8_t(e_))));
        return true;
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

inline std::string Enum_Incomplete_Truncated_Open_ToString(Enum_Incomplete_Truncated_Open_ tag) {
    switch (tag) {
        case Enum_Incomplete_Truncated_Open_::A: return "A";
        case Enum_Incomplete_Truncated_Open_::B: return "B";
        default:
            break;
    }

    return std::string("UNKNOWN(" +
           std::to_string(uint8_t(tag)) + ")");
}

class Enum_Incomplete_Truncated_OpenView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Enum_Incomplete_Truncated_OpenView> Decode(pdl::packet::slice const& parent) {
        Enum_Incomplete_Truncated_OpenView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.e_ = Enum_Incomplete_Truncated_Open_((chunk0 >> 0) & 0x7);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum_Incomplete_Truncated_Open_ GetE() const { return e_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Incomplete_Truncated_OpenView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (uint8_t(e_) > 0x7) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(uint8_t(e_))));
        return true;
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    Enum_Incomplete_Truncated_Open_ e_{Enum_Incomplete_Truncated_Open_::A};
};

enum class Enum_Incomplete_Truncated_Closed_WithRange_ : uint8_t {
    A = 0x0,
    X = 0x1,
    Y = 0x2,
};

inline std::string Enum_Incomplete_Truncated_Closed_WithRange_ToString(Enum_Incomplete_Truncated_Closed_WithRange_ tag) {
    switch (tag) {
        case Enum_Incomplete_Truncated_Closed_WithRange_::A: return "A";
        case Enum_Incomplete_Truncated_Closed_WithRange_::X: return "X";
        case Enum_Incomplete_Truncated_Closed_WithRange_::Y: return "Y";
        default:
            break;
    }
    if (uint8_t(tag) >= 1 && uint8_t(tag) <= 6) {
        return std::string("B(" + std::to_string(uint8_t(tag)) + ")");
    }
    return std::string("UNKNOWN(" +
           std::to_string(uint8_t(tag)) + ")");
}

inline bool IsValidEnum_Incomplete_Truncated_Closed_WithRange_(uint8_t value) {
    return value == 0x0
        || (0x1 <= value && value <= 0x6);
}

class Enum_Incomplete_Truncated_Closed_WithRangeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Enum_Incomplete_Truncated_Closed_WithRangeView> Decode(pdl::packet::slice const& parent) {
        Enum_Incomplete_Truncated_Closed_WithRangeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        auto raw_e = (chunk0 >> 0) & 0x7;
        if (!IsValidEnum_Incomplete_Truncated_Closed_WithRange_(raw_e)) {
           return std::nullopt;
        }
        output.e_ = Enum_Incomplete_Truncated_Closed_WithRange_(raw_e);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum_Incomplete_Truncated_Closed_WithRange_ GetE() const { return e_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Incomplete_Truncated_Closed_WithRangeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (uint8_t(e_) > 0x7) {
            return false;
        }
        if (!IsValidEnum_Incomplete_Truncated_Closed_WithRange_(uint8_t(e_))) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(uint8_t(e_))));
        return true;
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    Enum_Incomplete_Truncated_Closed_WithRange_ e_{Enum_Incomplete_Truncated_Closed_WithRange_::A};
};

enum class Enum_Incomplete_Truncated_Open_WithRange_ : uint8_t {
    A = 0x0,
    X = 0x1,
    Y = 0x2,
};

inline std::string Enum_Incomplete_Truncated_Open_WithRange_ToString(Enum_Incomplete_Truncated_Open_WithRange_ tag) {
    switch (tag) {
        case Enum_Incomplete_Truncated_Open_WithRange_::A: return "A";
        case Enum_Incomplete_Truncated_Open_WithRange_::X: return "X";
        case Enum_Incomplete_Truncated_Open_WithRange_::Y: return "Y";
        default:
            break;
    }
    if (uint8_t(tag) >= 1 && uint8_t(tag) <= 6) {
        return std::string("B(" + std::to_string(uint8_t(tag)) + ")");
    }
    return std::string("UNKNOWN(" +
           std::to_string(uint8_t(tag)) + ")");
}

class Enum_Incomplete_Truncated_Open_WithRangeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Enum_Incomplete_Truncated_Open_WithRangeView> Decode(pdl::packet::slice const& parent) {
        Enum_Incomplete_Truncated_Open_WithRangeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.e_ = Enum_Incomplete_Truncated_Open_WithRange_((chunk0 >> 0) & 0x7);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum_Incomplete_Truncated_Open_WithRange_ GetE() const { return e_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Incomplete_Truncated_Open_WithRangeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (uint8_t(e_) > 0x7) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(uint8_t(e_))));
        return true;
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

inline std::string Enum_Complete_Truncated_ToString(Enum_Complete_Truncated_ tag) {
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
            break;
    }

    return std::string("UNKNOWN(" +
           std::to_string(uint8_t(tag)) + ")");
}

class Enum_Complete_TruncatedView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Enum_Complete_TruncatedView> Decode(pdl::packet::slice const& parent) {
        Enum_Complete_TruncatedView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.e_ = Enum_Complete_Truncated_((chunk0 >> 0) & 0x7);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum_Complete_Truncated_ GetE() const { return e_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Complete_TruncatedView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (uint8_t(e_) > 0x7) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(uint8_t(e_))));
        return true;
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    Enum_Complete_Truncated_ e_{Enum_Complete_Truncated_::A};
};

enum class Enum_Complete_Truncated_WithRange_ : uint8_t {
    A = 0x0,
    X = 0x1,
    Y = 0x2,
};

inline std::string Enum_Complete_Truncated_WithRange_ToString(Enum_Complete_Truncated_WithRange_ tag) {
    switch (tag) {
        case Enum_Complete_Truncated_WithRange_::A: return "A";
        case Enum_Complete_Truncated_WithRange_::X: return "X";
        case Enum_Complete_Truncated_WithRange_::Y: return "Y";
        default:
            break;
    }
    if (uint8_t(tag) >= 1 && uint8_t(tag) <= 7) {
        return std::string("B(" + std::to_string(uint8_t(tag)) + ")");
    }
    return std::string("UNKNOWN(" +
           std::to_string(uint8_t(tag)) + ")");
}

class Enum_Complete_Truncated_WithRangeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Enum_Complete_Truncated_WithRangeView> Decode(pdl::packet::slice const& parent) {
        Enum_Complete_Truncated_WithRangeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        uint8_t chunk0 = span.read_be<uint8_t, 1>();
        output.e_ = Enum_Complete_Truncated_WithRange_((chunk0 >> 0) & 0x7);
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum_Complete_Truncated_WithRange_ GetE() const { return e_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Complete_Truncated_WithRangeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        if (uint8_t(e_) > 0x7) {
            return false;
        }
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(uint8_t(e_))));
        return true;
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

inline std::string Enum_Complete_WithRange_ToString(Enum_Complete_WithRange_ tag) {
    switch (tag) {
        case Enum_Complete_WithRange_::A: return "A";
        case Enum_Complete_WithRange_::B: return "B";
        default:
            break;
    }
    if (uint8_t(tag) >= 2 && uint8_t(tag) <= 255) {
        return std::string("C(" + std::to_string(uint8_t(tag)) + ")");
    }
    return std::string("UNKNOWN(" +
           std::to_string(uint8_t(tag)) + ")");
}

class Enum_Complete_WithRangeView {
public:
    // Decode the packet from the packet bytes.
    // Returns std::nullopt if the packet is invalid.
    static std::optional<Enum_Complete_WithRangeView> Decode(pdl::packet::slice const& parent) {
        Enum_Complete_WithRangeView output(parent);
        pdl::packet::slice span = parent;
        if (span.size() < 1) {
            return std::nullopt;
        }
        output.e_ = Enum_Complete_WithRange_(span.read_be<uint8_t, 1>());
        if (span.size() > 0) {
            return std::nullopt;
        }
        return output;
    }

    Enum_Complete_WithRange_ GetE() const { return e_; }

    std::string ToString() const { return ""; }

    pdl::packet::slice bytes() const {
        return bytes_;
    }

protected:
    explicit Enum_Complete_WithRangeView(pdl::packet::slice const& bytes) : bytes_(bytes) {}

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

    bool Serialize(std::vector<uint8_t>& output) const override {
        output.reserve(GetSize());
        pdl::packet::Builder::write_be<uint8_t, 1>(output, (static_cast<uint8_t>(uint8_t(e_))));
        return true;
    }

    size_t GetSize() const override {
        return 1;
    }

    std::string ToString() const { return ""; }

    Enum_Complete_WithRange_ e_{Enum_Complete_WithRange_::A};
};
}  // be_backend
