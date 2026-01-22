#[macro_export]
macro_rules! tagged_tuple_union 
{
    ($union_name:ident, $tag_enum:ident, { $( $variant:ident $( ( $ty:ty ) )? ),* $(,)? }) => {
        pub enum $union_name {
            $( $variant $( ( $ty ) )?, )*
        }

        impl $union_name {
            pub fn tag(&self) -> $tag_enum {
                match self {
                    $( $union_name::$variant { .. } => $tag_enum::$variant, )*
                }
            }
        }

        const _: () = {
            $( let _ = $tag_enum::$variant; )*
        };
    };
}

#[macro_export]
macro_rules! tagged_struct_union {
    (
        $union_name:ident, $tag_enum:ident, {
            $( $variant:ident { $( $field:ident : $field_ty:ty ),* $(,)? } ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $union_name {
            $( $variant { $( $field : $field_ty ),* }, )*
        }

        impl $union_name {
            pub fn tag(&self) -> $tag_enum {
                match self {
                    $( $union_name::$variant { .. } => $tag_enum::$variant, )*
                }
            }
        }

        // Compile-time check that all variants exist in the tag enum
        const _: () = {
            $( let _ = $tag_enum::$variant; )*
        };
    };
}
