/// Macro for declaring Objective-C classes
pub macro objc_class {
    (
     // meta data about struct
     $(#[$meta:meta])* 
     $vis:vis class $class_name:ident {
        $(
        // meta data about field
        $(#[$field_meta:meta])*
        $field_vis:vis $field_name:ident : $field_type:ty
        ),*$(,)+

        $(
        $(#[member_meta:meta])*
        $member_vis:vis fn $member_name:ident ($($param_name:ident : $param_type:ty)*)
        )*
    }
    ) => {
        { 
            $(#[$meta])*
            pub struct $class_name {
                $(
                $(#[$field_meta])*
                pub $field_name : $field_type,
                )*
            }

            impl concat!("I", $class_name) for $class_name {
                $(
                $(#[$member_meta])*
                $member_vis fn $member_name (this: &Object ,$($param_name : $param_type)*)
                )*
            } 
        }
    }
}

