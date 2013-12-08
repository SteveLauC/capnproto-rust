/*
 * Copyright (c) 2013, David Renshaw (dwrenshaw@gmail.com)
 *
 * See the LICENSE file in the capnproto-rust root directory.
 */

use layout::{PointerBuilder, PointerReader};
use common::Word;

pub trait FromPointerReader<'a> {
    fn get_from_pointer(reader : &PointerReader<'a>, default_value : *Word) -> Self;
}

pub trait FromPointerBuilder {
    fn init_pointer(PointerBuilder, uint) -> Self;
    fn get_from_pointer(builder : PointerBuilder, default_value : *Word) -> Self;
}

pub mod PrimitiveList {
    use super::{FromPointerReader, FromPointerBuilder};
    use layout::{ListReader, ListBuilder, PointerReader, PointerBuilder,
                 PrimitiveElement, POINTER};
    use common::Word;

    pub struct Reader<'a, T> {
        reader : ListReader<'a>
    }

    impl <'a, T : PrimitiveElement> Reader<'a, T> {
        pub fn new<'b>(reader : ListReader<'b>) -> Reader<'b, T> {
            Reader::<'b, T> { reader : reader }
        }

        pub fn size(&self) -> uint { self.reader.size() }
    }

    impl <'a, T : PrimitiveElement> FromPointerReader<'a> for Reader<'a, T> {
        fn get_from_pointer(reader : &PointerReader<'a>, default_value : *Word) -> Reader<'a, T> {
            Reader { reader : reader.get_list(POINTER, default_value) }
        }
    }

    impl <'a, T : PrimitiveElement> Index<uint, T> for Reader<'a, T> {
        fn index(&self, index : &uint) -> T {
            PrimitiveElement::get(&self.reader, *index)
        }
    }

    pub struct Builder<T> {
        builder : ListBuilder
    }

    impl <T : PrimitiveElement> Builder<T> {
        pub fn new(builder : ListBuilder) -> Builder<T> {
            Builder { builder : builder }
        }

        pub fn size(&self) -> uint { self.builder.size() }

        pub fn set(&self, index : uint, value : T) {
            PrimitiveElement::set(&self.builder, index, value);
        }
    }

    impl <T : PrimitiveElement> FromPointerBuilder for Builder<T> {
        fn init_pointer(_builder : PointerBuilder, _size : uint) -> Builder<T> {
//            builder.init_list(
            fail!();
        }
        fn get_from_pointer(_builder : PointerBuilder, _default_value : *Word) -> Builder<T> {
            fail!();
        }
    }


    impl <T : PrimitiveElement> Index<uint, T> for Builder<T> {
        fn index(&self, index : &uint) -> T {
            PrimitiveElement::get_from_builder(&self.builder, *index)
        }
    }
}

pub trait ToU16 {
    fn to_u16(self) -> u16;
}


pub mod EnumList {
    use layout::*;
    use list::*;
    use common::Word;

    pub struct Reader<'a, T> {
        reader : ListReader<'a>
    }

    impl <'a, T : FromPrimitive> Reader<'a, T> {
        pub fn new<'b>(reader : ListReader<'b>) -> Reader<'b, T> {
            Reader::<'b, T> { reader : reader }
        }

        pub fn size(&self) -> uint { self.reader.size() }

    }

    impl <'a, T : FromPrimitive> FromPointerReader<'a> for Reader<'a, T> {
        fn get_from_pointer(reader : &PointerReader<'a>, default_value : *Word) -> Reader<'a, T> {
            Reader { reader : reader.get_list(TWO_BYTES, default_value) }
        }
    }

    impl <'a, T : FromPrimitive> Index<uint, Option<T>> for Reader<'a, T> {
        fn index(&self, index : &uint) -> Option<T> {
            let result : u16 = PrimitiveElement::get(&self.reader, *index);
            FromPrimitive::from_u16(result)
        }
    }

    pub struct Builder<T> {
        builder : ListBuilder
    }

    impl <T : ToU16 + FromPrimitive> Builder<T> {
        pub fn new(builder : ListBuilder) -> Builder<T> {
            Builder { builder : builder }
        }

        pub fn size(&self) -> uint { self.builder.size() }

        pub fn set(&self, index : uint, value : T) {
            PrimitiveElement::set(&self.builder, index, value.to_u16());
        }
    }

    impl <T : FromPrimitive> FromPointerBuilder for Builder<T> {
        fn init_pointer(builder : PointerBuilder, size : uint) -> Builder<T> {
            Builder { builder : builder.init_list(TWO_BYTES, size) }
        }
        fn get_from_pointer(builder : PointerBuilder, default_value : *Word) -> Builder<T> {
            Builder { builder : builder.get_list(TWO_BYTES, default_value) }
        }
    }


    impl <T : ToU16 + FromPrimitive> Index<uint, Option<T>> for Builder<T> {
        fn index(&self, index : &uint) -> Option<T> {
            let result : u16 = PrimitiveElement::get_from_builder(&self.builder, *index);
            FromPrimitive::from_u16(result)
        }
    }
}

pub mod StructList {
    use super::{FromPointerReader, FromPointerBuilder};
    use common::Word;
    use layout::*;

    pub struct Reader<'a, T> {
        reader : ListReader<'a>
    }

    impl <'a, T : FromStructReader<'a>> Reader<'a, T> {
        pub fn new<'b>(reader : ListReader<'b>) -> Reader<'b, T> {
            Reader::<'b, T> { reader : reader }
        }

        pub fn size(&self) -> uint { self.reader.size() }
    }

    impl <'a, T : FromStructReader<'a>> FromPointerReader<'a> for Reader<'a, T> {
        fn get_from_pointer(reader : &PointerReader<'a>, default_value : *Word) -> Reader<'a, T> {
            Reader { reader : reader.get_list(INLINE_COMPOSITE, default_value) }
        }
    }

    impl <'a, T : FromStructReader<'a>> Index<uint, T> for Reader<'a, T> {
        fn index(&self, index : &uint) -> T {
            let result : T = FromStructReader::from_struct_reader(self.reader.get_struct_element(*index));
            result
        }
    }


    pub struct Builder<T> {
        builder : ListBuilder
    }

    impl <T : FromStructBuilder> Builder<T> {
        pub fn new(builder : ListBuilder) -> Builder<T> {
            Builder { builder : builder }
        }

        pub fn size(&self) -> uint { self.builder.size() }

//        pub fn set(&self, index : uint, value : T) {
//        }
    }

    impl <T : FromStructBuilder + HasStructSize> FromPointerBuilder for Builder<T> {
        fn init_pointer(builder : PointerBuilder, size : uint) -> Builder<T> {
            Builder {
                builder : builder.init_struct_list(size, HasStructSize::struct_size(None::<T>))
            }
        }
        fn get_from_pointer(builder : PointerBuilder, default_value : *Word) -> Builder<T> {
            Builder {
                builder : builder.get_struct_list(HasStructSize::struct_size(None::<T>), default_value)
            }
        }
    }

    impl <T : FromStructBuilder> Index<uint, T> for Builder<T> {
        fn index(&self, index : &uint) -> T {
            let result : T =
                FromStructBuilder::from_struct_builder(self.builder.get_struct_element(*index));
            result
        }
    }

}

pub mod ListList {
    use super::{FromPointerReader, FromPointerBuilder};
    use std;
    use common::Word;
    use layout::*;

    pub struct Reader<'a, T> {
        reader : ListReader<'a>
    }

    impl <'a, T> Reader<'a, T> {
        pub fn new<'b>(reader : ListReader<'b>) -> Reader<'b, T> {
            Reader::<'b, T> { reader : reader }
        }

        pub fn size(&self) -> uint { self.reader.size() }
    }

    impl <'a, T : FromPointerReader<'a>> FromPointerReader<'a> for Reader<'a, T> {
        fn get_from_pointer(reader : &PointerReader<'a>, default_value : *Word) -> Reader<'a, T> {
            Reader { reader : reader.get_list(POINTER, default_value) }
        }
    }

    impl <'a, T : FromPointerReader<'a>> Index<uint, T> for Reader<'a, T> {
        fn index(&self, index : &uint) -> T {
            assert!(*index <  self.size());
            FromPointerReader::get_from_pointer(
                &self.reader.get_pointer_element(*index), std::ptr::null())
        }
    }

    pub struct Builder<T> {
        builder : ListBuilder
    }

    impl <T : FromPointerBuilder> Builder<T> {
        pub fn new(builder : ListBuilder) -> Builder<T> {
            Builder { builder : builder }
        }

        pub fn size(&self) -> uint { self.builder.size() }
    }


    impl <T : FromPointerBuilder> FromPointerBuilder for Builder<T> {
        fn init_pointer(builder : PointerBuilder, size : uint) -> Builder<T> {
            Builder {
                builder : builder.init_list(POINTER, size)
            }
        }
        fn get_from_pointer(builder : PointerBuilder, default_value : *Word) -> Builder<T> {
            Builder {
                builder : builder.get_list(POINTER, default_value)
            }
        }
    }

    impl <T : FromPointerBuilder> Index<uint, T> for Builder<T> {
        fn index(&self, index : &uint) -> T {
            let result : T =
                FromPointerBuilder::get_from_pointer(
                self.builder.get_pointer_element(*index),
                std::ptr::null());
            result
        }
    }

}

// TODO BlobList
