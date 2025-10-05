macro_rules! c_rust {
    ($($code:tt)*) => {
        parse_c!($($code)*);
    };
}

macro_rules! parse_c {
    ($($typ:ident $fn_name:ident () { $($body:tt)* } )*) => {
        $(
            unsafe fn $fn_name() -> c_ty!($typ) {
                #[allow(unused_unsafe)]
                unsafe {
                    gen_body!(@body $($body)*);
                }
            }
        )*
    };
}

macro_rules! gen_body { 
    (@body $typ:ident $name:ident = ($cast_ty:ident)$val:expr; $($rest: tt)*) => {
       #[allow(unused_mut, unnecessary_transmutes)]
       let mut $name: c_ty!($cast_ty) = unsafe { ::core::mem::transmute($val) }; 
       gen_body! {@body $($rest)* }

    };
    (@body $typ:ident $name:ident = $val:expr; $($rest: tt)*) => {
       #[allow(unused_mut)]
       let mut $name: i32 = $val; 
       gen_body! {@body $($rest)* }

    };
    (@body $name:ident = $ex:expr; $($rest: tt)*) => {
            $name = $ex;
            gen_body! { @body $($rest)* }
    };
    (@body return $body: expr;) => { 
        return $body;
    };
 
}

macro_rules! c_ty {
    (int) => { i32 };
    (uint64_t) => { u64 };
    (float) => { f32 };
    (ptr_int) => { *mut i32 };
    (void) => { () };
}

fn magic_rust_fn() -> i32 { 
    90
}

fn print(t: &str) {
    println!("{t}");
}

c_rust! {
   int c_other() {
        return 2;
   }

   ptr_int null_pointer() {
        ptr_int a = (ptr_int)0_u64;
        return a;
   }

   float c_start() {
       float d = (float)20;

       return d; 
   }
}

fn main() {
    unsafe {
      println!("{} {} {}", c_start(), null_pointer().addr(), c_other()); 
    }
}
