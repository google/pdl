use std::collections::{hash_map::Entry, HashMap};

use crate::ast;

pub struct Schema<'a> {
    pub packets_and_structs: HashMap<&'a str, PacketOrStruct<'a>>,
    pub enums: HashMap<&'a str, Enum<'a>>,
}

pub struct PacketOrStruct<'a> {
    pub computed_offsets: HashMap<ComputedOffsetId<'a>, ComputedOffset<'a>>,
    pub computed_values: HashMap<ComputedValueId<'a>, ComputedValue<'a>>,
    /// whether the parse of this packet needs to know its length,
    /// or if the packet can determine its own length
    pub length: PacketOrStructLength,
}

pub enum PacketOrStructLength {
    Static(usize),
    Dynamic,
    NeedsExternal,
}

pub struct Enum<'a> {
    pub tags: &'a [ast::Tag],
    pub width: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ComputedValueId<'a> {
    // needed for array fields + varlength structs - note that this is in OCTETS, not BITS
    // this always works since array entries are either structs (which are byte-aligned) or integer-octet-width scalars
    FieldSize(&'a str),

    // needed for arrays with fixed element size (otherwise codegen will loop!)
    FieldElementSize(&'a str), // note that this is in OCTETS, not BITS
    FieldCount(&'a str),

    Custom(u16),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ComputedOffsetId<'a> {
    // these quantities are known by the runtime
    HeaderStart,

    // if the packet needs its length, this will be supplied. otherwise it will be computed
    PacketEnd,

    // these quantities will be computed and stored in computed_values
    FieldOffset(&'a str),    // needed for all fields, measured in BITS
    FieldEndOffset(&'a str), // needed only for Payload + Body fields, as well as variable-size structs (not arrays), measured in BITS
    Custom(u16),
    TrailerStart,
}

pub enum ComputedValue<'a> {
    Constant(usize),
    CountStructsUpToSize {
        base_id: ComputedOffsetId<'a>,
        size: ComputedValueId<'a>,
        struct_type: &'a str,
    },
    SizeOfNStructs {
        base_id: ComputedOffsetId<'a>,
        n: ComputedValueId<'a>,
        struct_type: &'a str,
    },
    Product(ComputedValueId<'a>, ComputedValueId<'a>),
    Divide(ComputedValueId<'a>, ComputedValueId<'a>),
    Difference(ComputedOffsetId<'a>, ComputedOffsetId<'a>),
    ValueAt {
        offset: ComputedOffsetId<'a>,
        width: usize,
    },
}

#[derive(Copy, Clone)]
pub enum ComputedOffset<'a> {
    ConstantPlusOffsetInBits(ComputedOffsetId<'a>, i64),
    SumWithOctets(ComputedOffsetId<'a>, ComputedValueId<'a>),
    Alias(ComputedOffsetId<'a>),
}

pub fn generate(file: &ast::File) -> Result<Schema, String> {
    let mut schema = Schema { packets_and_structs: HashMap::new(), enums: HashMap::new() };
    match file.endianness.value {
        ast::EndiannessValue::LittleEndian => {}
        _ => unimplemented!("Only little_endian endianness supported"),
    };

    for decl in &file.declarations {
        process_decl(&mut schema, decl);
    }

    Ok(schema)
}

fn process_decl<'a>(schema: &mut Schema<'a>, decl: &'a ast::Decl) {
    match decl {
        ast::Decl::Enum { id, tags, width, .. } => process_enum(schema, id, tags, *width),
        ast::Decl::Packet { id, fields, .. } | ast::Decl::Struct { id, fields, .. } => {
            process_packet_or_struct(schema, id, fields)
        }
        ast::Decl::Group { .. } => todo!(),
        _ => unimplemented!("type {decl:?} not supported"),
    }
}

fn process_enum<'a>(schema: &mut Schema<'a>, id: &'a str, tags: &'a [ast::Tag], width: usize) {
    schema.enums.insert(id, Enum { tags, width });
    schema.packets_and_structs.insert(
        id,
        PacketOrStruct {
            computed_offsets: HashMap::new(),
            computed_values: HashMap::new(),
            length: PacketOrStructLength::Static(width),
        },
    );
}

fn process_packet_or_struct<'a>(schema: &mut Schema<'a>, id: &'a str, fields: &'a [ast::Field]) {
    schema.packets_and_structs.insert(id, compute_getters(schema, fields));
}

fn compute_getters<'a>(schema: &Schema<'a>, fields: &'a [ast::Field]) -> PacketOrStruct<'a> {
    let mut prev_pos_id = None;
    let mut curr_pos_id = ComputedOffsetId::HeaderStart;
    let mut computed_values = HashMap::new();
    let mut computed_offsets = HashMap::new();

    let mut cnt = 0;

    let one_id = ComputedValueId::Custom(cnt);
    let one_val = ComputedValue::Constant(1);
    cnt += 1;
    computed_values.insert(one_id, one_val);

    let mut needs_length = false;

    for field in fields {
        // populate this only if we are an array with a knowable size
        let mut next_prev_pos_id = None;

        let next_pos = match field {
            ast::Field::Reserved { width, .. } => {
                ComputedOffset::ConstantPlusOffsetInBits(curr_pos_id, *width as i64)
            }
            ast::Field::Scalar { id, width, .. } => {
                computed_offsets
                    .insert(ComputedOffsetId::FieldOffset(id), ComputedOffset::Alias(curr_pos_id));
                ComputedOffset::ConstantPlusOffsetInBits(curr_pos_id, *width as i64)
            }
            ast::Field::Fixed { width, enum_id, .. } => {
                let offset = match (width, enum_id) {
                    (Some(width), _) => *width,
                    (_, Some(enum_id)) => schema.enums[enum_id.as_str()].width,
                    _ => unreachable!(),
                };
                ComputedOffset::ConstantPlusOffsetInBits(curr_pos_id, offset as i64)
            }
            ast::Field::Size { field_id, width, .. } => {
                computed_values.insert(
                    ComputedValueId::FieldSize(field_id),
                    ComputedValue::ValueAt { offset: curr_pos_id, width: *width },
                );
                ComputedOffset::ConstantPlusOffsetInBits(curr_pos_id, *width as i64)
            }
            ast::Field::Count { field_id, width, .. } => {
                computed_values.insert(
                    ComputedValueId::FieldCount(field_id.as_str()),
                    ComputedValue::ValueAt { offset: curr_pos_id, width: *width },
                );
                ComputedOffset::ConstantPlusOffsetInBits(curr_pos_id, *width as i64)
            }
            ast::Field::ElementSize { field_id, width, .. } => {
                computed_values.insert(
                    ComputedValueId::FieldElementSize(field_id),
                    ComputedValue::ValueAt { offset: curr_pos_id, width: *width },
                );
                ComputedOffset::ConstantPlusOffsetInBits(curr_pos_id, *width as i64)
            }
            ast::Field::Group { .. } => {
                unimplemented!("this should be removed by the linter...")
            }
            ast::Field::Checksum { .. } => unimplemented!("checksum not supported"),
            ast::Field::Body { .. } => {
                computed_offsets.insert(
                    ComputedOffsetId::FieldOffset("_body_"),
                    ComputedOffset::Alias(curr_pos_id),
                );
                let computed_size_id = ComputedValueId::FieldSize("_body_");
                let end_offset = if computed_values.contains_key(&computed_size_id) {
                    ComputedOffset::SumWithOctets(curr_pos_id, computed_size_id)
                } else {
                    if needs_length {
                        panic!("only one variable-length field can exist")
                    }
                    needs_length = true;
                    ComputedOffset::Alias(ComputedOffsetId::TrailerStart)
                };
                computed_offsets.insert(ComputedOffsetId::FieldEndOffset("_body_"), end_offset);
                end_offset
            }
            ast::Field::Payload { size_modifier, .. } => {
                if size_modifier.is_some() {
                    unimplemented!("size modifiers not supported")
                }
                computed_offsets.insert(
                    ComputedOffsetId::FieldOffset("_payload_"),
                    ComputedOffset::Alias(curr_pos_id),
                );
                let computed_size_id = ComputedValueId::FieldSize("_payload_");
                let end_offset = if computed_values.contains_key(&computed_size_id) {
                    ComputedOffset::SumWithOctets(curr_pos_id, computed_size_id)
                } else {
                    if needs_length {
                        panic!("only one variable-length field can exist")
                    }
                    needs_length = true;
                    ComputedOffset::Alias(ComputedOffsetId::TrailerStart)
                };
                computed_offsets.insert(ComputedOffsetId::FieldEndOffset("_payload_"), end_offset);
                end_offset
            }
            ast::Field::Array {
                id,
                width,
                type_id,
                size_modifier,
                size: statically_known_count,
                ..
            } => {
                if size_modifier.is_some() {
                    unimplemented!("size modifiers not supported")
                }

                computed_offsets
                    .insert(ComputedOffsetId::FieldOffset(id), ComputedOffset::Alias(curr_pos_id));

                // there are a few parameters to consider when parsing arrays
                // 1: the count of elements
                // 2: the total byte size (possibly by subtracting out the len of the trailer)
                // 3: whether the structs know their own lengths
                // parsing is possible if we know (1 OR 2) AND 3

                if let Some(count) = statically_known_count {
                    computed_values
                        .insert(ComputedValueId::FieldCount(id), ComputedValue::Constant(*count));
                }

                let statically_known_width_in_bits = if let Some(type_id) = type_id {
                    if let PacketOrStructLength::Static(len) =
                        schema.packets_and_structs[type_id.as_str()].length
                    {
                        Some(len)
                    } else {
                        None
                    }
                } else if let Some(width) = width {
                    Some(*width)
                } else {
                    unreachable!()
                };

                // whether the count is known *prior* to parsing the field
                let is_count_known = computed_values.contains_key(&ComputedValueId::FieldCount(id));
                // whether the total field size is explicitly specified
                let is_total_size_known =
                    computed_values.contains_key(&ComputedValueId::FieldSize(id));

                let element_size = if let Some(type_id) = type_id {
                    match schema.packets_and_structs[type_id.as_str()].length {
                        PacketOrStructLength::Static(width) => {
                            assert!(width % 8 == 0);
                            Some(width / 8)
                        }
                        PacketOrStructLength::Dynamic => None,
                        PacketOrStructLength::NeedsExternal => None,
                    }
                } else if let Some(width) = width {
                    assert!(width % 8 == 0);
                    Some(width / 8)
                } else {
                    unreachable!()
                };
                if let Some(element_size) = element_size {
                    computed_values.insert(
                        ComputedValueId::FieldElementSize(id),
                        ComputedValue::Constant(element_size),
                    );
                }

                // whether we can know the length of each element in the array by greedy parsing,
                let structs_know_length = if let Some(type_id) = type_id {
                    match schema.packets_and_structs[type_id.as_str()].length {
                        PacketOrStructLength::Static(_) => true,
                        PacketOrStructLength::Dynamic => true,
                        PacketOrStructLength::NeedsExternal => {
                            computed_values.contains_key(&ComputedValueId::FieldElementSize(id))
                        }
                    }
                } else {
                    width.is_some()
                };

                if !structs_know_length {
                    panic!("structs need to know their own length, if they live in an array")
                }

                let mut out = None;
                if let Some(count) = statically_known_count {
                    if let Some(width) = statically_known_width_in_bits {
                        // the fast path, if the count and width are statically known, is to just immediately multiply
                        // otherwise this becomes a dynamic computation
                        assert!(width % 8 == 0);
                        computed_values.insert(
                            ComputedValueId::FieldSize(id),
                            ComputedValue::Constant(count * width / 8),
                        );
                        out = Some(ComputedOffset::ConstantPlusOffsetInBits(
                            curr_pos_id,
                            (count * width) as i64,
                        ));
                    }
                }

                // note: this introduces a forward dependency with the total_size_id
                // however, the FieldSize(id) only depends on the FieldElementSize(id) if FieldCount() == true
                // thus, there will never be an infinite loop, since the FieldElementSize(id) only depends on the
                // FieldSize() if the FieldCount() is not unknown
                if !is_count_known {
                    // the count is not known statically, or from earlier in the packet
                    // thus, we must compute it from the total size of the field, known either explicitly or implicitly via the trailer
                    // the fast path is to do a divide, but otherwise we need to loop over the TLVs
                    computed_values.insert(
                        ComputedValueId::FieldCount(id),
                        if computed_values.contains_key(&ComputedValueId::FieldElementSize(id)) {
                            ComputedValue::Divide(
                                ComputedValueId::FieldSize(id),
                                ComputedValueId::FieldElementSize(id),
                            )
                        } else {
                            ComputedValue::CountStructsUpToSize {
                                base_id: curr_pos_id,
                                size: ComputedValueId::FieldSize(id),
                                struct_type: type_id.as_ref().unwrap(),
                            }
                        },
                    );
                }

                if let Some(out) = out {
                    // we are paddable if the total size is known
                    next_prev_pos_id = Some(curr_pos_id);
                    out
                } else if is_total_size_known {
                    // we are paddable if the total size is known
                    next_prev_pos_id = Some(curr_pos_id);
                    ComputedOffset::SumWithOctets(curr_pos_id, ComputedValueId::FieldSize(id))
                } else if is_count_known {
                    // we are paddable if the total count is known, since structs know their lengths
                    next_prev_pos_id = Some(curr_pos_id);

                    computed_values.insert(
                        ComputedValueId::FieldSize(id),
                        if computed_values.contains_key(&ComputedValueId::FieldElementSize(id)) {
                            ComputedValue::Product(
                                ComputedValueId::FieldCount(id),
                                ComputedValueId::FieldElementSize(id),
                            )
                        } else {
                            ComputedValue::SizeOfNStructs {
                                base_id: curr_pos_id,
                                n: ComputedValueId::FieldCount(id),
                                struct_type: type_id.as_ref().unwrap(),
                            }
                        },
                    );
                    ComputedOffset::SumWithOctets(curr_pos_id, ComputedValueId::FieldSize(id))
                } else {
                    // we can try to infer the total size if we are still in the header
                    // however, we are not paddable in this case
                    next_prev_pos_id = None;

                    if needs_length {
                        panic!("either the total size, or the count of elements in an array, must be known")
                    }
                    // now we are in the trailer
                    computed_values.insert(
                        ComputedValueId::FieldSize(id),
                        ComputedValue::Difference(ComputedOffsetId::TrailerStart, curr_pos_id),
                    );
                    needs_length = true;
                    ComputedOffset::Alias(ComputedOffsetId::TrailerStart)
                }
            }
            ast::Field::Padding { size, .. } => {
                if let Some(prev_pos_id) = prev_pos_id {
                    ComputedOffset::ConstantPlusOffsetInBits(prev_pos_id, *size as i64)
                } else {
                    panic!("padding must follow array field with known total size")
                }
            }
            ast::Field::Typedef { id, type_id, .. } => {
                computed_offsets
                    .insert(ComputedOffsetId::FieldOffset(id), ComputedOffset::Alias(curr_pos_id));

                match schema.packets_and_structs[type_id.as_str()].length {
                    PacketOrStructLength::Static(len) => {
                        ComputedOffset::ConstantPlusOffsetInBits(curr_pos_id, len as i64)
                    }
                    PacketOrStructLength::Dynamic => {
                        computed_values.insert(
                            ComputedValueId::FieldSize(id),
                            ComputedValue::SizeOfNStructs {
                                base_id: curr_pos_id,
                                n: one_id,
                                struct_type: type_id,
                            },
                        );
                        ComputedOffset::SumWithOctets(curr_pos_id, ComputedValueId::FieldSize(id))
                    }
                    PacketOrStructLength::NeedsExternal => {
                        let end_offset = if let Entry::Vacant(entry) =
                            computed_values.entry(ComputedValueId::FieldSize(id))
                        {
                            // its size is presently unknown
                            if needs_length {
                                panic!(
                                        "cannot have multiple variable-length fields in a single packet/struct"
                                    )
                            }
                            // we are now in the trailer
                            entry.insert(ComputedValue::Difference(
                                ComputedOffsetId::TrailerStart,
                                curr_pos_id,
                            ));
                            needs_length = true;
                            ComputedOffset::Alias(ComputedOffsetId::TrailerStart)
                        } else {
                            ComputedOffset::SumWithOctets(
                                curr_pos_id,
                                ComputedValueId::FieldSize(id),
                            )
                        };
                        computed_offsets.insert(ComputedOffsetId::FieldEndOffset(id), end_offset);
                        end_offset
                    }
                }

                // it is possible to size a struct in this variant of PDL, even though the linter doesn't allow it
            }
        };

        prev_pos_id = next_prev_pos_id;
        curr_pos_id = ComputedOffsetId::Custom(cnt);
        cnt += 1;
        computed_offsets.insert(curr_pos_id, next_pos);
    }

    // TODO(aryarahul): simplify compute graph to improve trailer resolution?

    // we are now at the end of the packet
    let length = if needs_length {
        // if we needed the length, use the PacketEnd and length to reconstruct the TrailerStart
        let trailer_length =
            compute_length_to_goal(&computed_offsets, curr_pos_id, ComputedOffsetId::TrailerStart)
                .expect("trailers should have deterministic length");
        computed_offsets.insert(
            ComputedOffsetId::TrailerStart,
            ComputedOffset::ConstantPlusOffsetInBits(ComputedOffsetId::PacketEnd, -trailer_length),
        );
        PacketOrStructLength::NeedsExternal
    } else {
        // otherwise, try to reconstruct the full length, if possible
        let full_length =
            compute_length_to_goal(&computed_offsets, curr_pos_id, ComputedOffsetId::HeaderStart);
        if let Some(full_length) = full_length {
            computed_offsets.insert(
                ComputedOffsetId::PacketEnd,
                ComputedOffset::ConstantPlusOffsetInBits(
                    ComputedOffsetId::HeaderStart,
                    full_length,
                ),
            );
            PacketOrStructLength::Static(full_length as usize)
        } else {
            computed_offsets
                .insert(ComputedOffsetId::PacketEnd, ComputedOffset::Alias(curr_pos_id));
            PacketOrStructLength::Dynamic
        }
    };

    PacketOrStruct { computed_values, computed_offsets, length }
}

fn compute_length_to_goal(
    computed_offsets: &HashMap<ComputedOffsetId, ComputedOffset>,
    start: ComputedOffsetId,
    goal: ComputedOffsetId,
) -> Option<i64> {
    let mut out = 0;
    let mut pos = start;
    while pos != goal {
        match computed_offsets.get(&pos).ok_or_else(|| format!("key {pos:?} not found")).unwrap() {
            ComputedOffset::ConstantPlusOffsetInBits(base_id, offset) => {
                out += offset;
                pos = *base_id;
            }
            ComputedOffset::Alias(alias) => pos = *alias,
            ComputedOffset::SumWithOctets { .. } => return None,
        }
    }
    Some(out)
}
