# Java Generated Code Guide

## Usage

Java codegen is gated by the `java` feature.

```bash
cargo run --features "java" -- --output-format java <IN> --output-dir <OUT> --java-package <PACKAGE>
```

For example, `cargo run --features "java" -- --output-format java examples/testing.pdl --output-dir ./target --java-package a.b.testing` will generate Java code for the declarations in `examples/testing.pdl` as `./target/testing/a/b/*.java`. Each of these files will declare `package a.b.testing;`.

## Unsupported Features

- Fixed fields
- Groups
- Checksum declarations
- Custom fields
- Test declarations
- Constraints on >1st order ancestors

## Packets

Packets come with a builder, with the type of each field being the smallest Java integral type that fits the declared field width, or `boolean` if the field is 1 bit wide. Packet classes have a static `void fromBytes(byte[])` and an instance `byte[] toBytes()`. Packet classes override `hashCode`, `equals`, and `toString` as expected.

```
packet MyPacket {
  a: 8,
  b: 16,
}
```

```java
MyPacket packet1 = new MyPacket.Builder().setA((byte) 1).setB((short) 2).build();
MyPacket packet2 = MyPacket.fromBytes(new byte[] {1, 0, 2});
assert packet1.equals(packet2);
```

Packet inheritance maps to Java inheritance. Each parent generates an abstract class with a static `<PARENT_NAME> fromBytes(byte[])` that constructs an instance of one of its children. Packets with a `_payload_` generate a concrete "fallback" child (`Unknown<PARENT_NAME>`) with a raw `byte[] payload` member. An instance of the fallback child is constructed if a concrete child can't be determined based on constraint values or child size. Packets with a `_body_` throw an exception instead of constructing a fallback.

```
packet Parent {
  a: 8,
  _payload_,
}

packet Child : Parent(a = 1) {
  b: 16,
}
```

```java
switch (Parent.fromBytes(bytes)) {
  case Child c -> System.out.println(c);
  case UnknownParent other -> System.out.println(other);
}
```

```java
Child packet1 = new Child.Builder().setB((short) 1).build();
UnknownParent packet2 = new UnknownParent.Builder().setA((byte) 1).setPayload(new byte[] {2, 3}).build();
```

## Enums

Enums and their tags also map to a Java class hierarchy. Each enum generates an abstract class with a static `<ENUM_NAME> from<T>(<T>)` and instance `<T> to<T>()` where `<T>` is the smallest integral Java type that fits the enum's width. The enum has an inner subclass for each tag. Single-valued tags have singletons that shadow their class definitions while range and default tags have a constructor. Enum classes override `hashCode`, `equals`, and `toString` as expected.

```
enum MyEnum : 7 {
    A = 1,
    B = 2,
    C = 3,
    Other = ..
}
```

```java
MyEnum a = MyEnum.A;
MyEnum other = MyEnum.Other((byte) 4);
assert MyEnum.fromByte((byte) 2).equals(MyEnum.B);
```

```
enum NestedEnum : 3 {
    A = 0,
    B = 1..6 {
        X = 1,
        Y = 2,
    },
}
```

```java
switch (NestedEnum.fromByte(val)) {
  case NestedEnum.A a -> {
    System.out.println(a);
  }
  case NestedEnum.B b -> {
    switch (b) {
      case NestedEnum.B.X x -> {
        System.out.println(x);
      }
      case NestedEnum.B.Y y -> {
        System.out.println(y);
      }
    }
  }
}
```
