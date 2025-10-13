#![no_std]

#[macro_export]
macro_rules! c_rust {
    ($($code:tt)*) => {
        parse_c!($($code)*);
    };
}

#[macro_export]
macro_rules! parse_c {
    (
        typedef struct { $($ty:ident $field:ident;)* } $name:ident;
        $($rest:tt)*
    ) => {
        struct $name {
            $( $field: c_ty!($ty), )*
        }
        parse_c! { $($rest)* }
    };

    (
        struct $name:ident { $($ty:ident $field:ident;)* };
        $($rest:tt)*
    ) => {
        struct $name {
            $( $field: c_ty!($ty), )*
        }
        parse_c! { $($rest)* }
    };


    (
        $ret:ident $fn_name:ident ($($ty: ident $arg_name: ident),*) { $($body:tt)* }
        $($rest:tt)*
    ) => {
        unsafe fn $fn_name($($arg_name: c_ty!($ty)),*) -> c_ty!($ret)  {
            #[allow(unused_unsafe)]
            unsafe {
                gen_body! { $($body)* }
            }
        }
        parse_c! { $($rest)* }
    };

    () => {};
}

#[macro_export]
macro_rules! gen_body {
    (typedef struct { $($ty:ident $field_name:ident;)* } $name: ident; $($rest: tt)*) => {
        struct $name {
            $(
                $field_name: c_ty!($ty),
            )*
        }
        gen_body! { $($rest)* }
    };
    ($typ:ident $name:ident = ($cast_ty:ident)$val:expr; $($rest: tt)*) => {
       #[allow(unused_mut, unnecessary_transmutes)]
       let mut $name: c_ty!($cast_ty) = unsafe { ::core::mem::transmute($val) };
       gen_body! {$($rest)* }

    };
    (auto $name:ident = $val:expr; $($rest: tt)*) => {
        #[allow(unused_mut)]
        let mut $name = $val;
        gen_body! { $($rest)* }
    };
    ($typ:ident $name:ident = $val:expr; $($rest: tt)*) => {
       #[allow(unused_mut)]
       let mut $name: c_ty!($typ) = $val;
       gen_body! { $($rest)* }

    };
    ($name:ident = $ex:expr; $($rest: tt)*) => {
            $name = $ex;
            gen_body! { $($rest)* }
    };
        (
        struct $name:ident { $($ty:ident $field:ident;)* };
        $($rest:tt)*
    ) => {
        struct $name {
            $( $field: c_ty!($ty), )*
        }
        parse_c! { $($rest)* }
    };

    (return $body: expr;) => {
        return $body;
    };

    ($block:expr; $($rest: tt)*) => {
        let _ = $block;

        gen_body! { $($rest)* }
    };

    () => {}

}

#[macro_export]
macro_rules! c_ty {
    (int) => { i32 };
    (uint64_t) => { u64 };
    (float) => { f32 };
    (ptr_int) => { *mut i32 };
    (void) => { () };

    ($ty: tt) => { $ty };
}

