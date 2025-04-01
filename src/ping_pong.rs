use std::{error::Error, slice::Iter};

macro_rules! ping_pong {
    ($index_type_number:ty [$($index_variant_index:literal => $index_variant_name:ident),*]) => {
        paste::paste! {
            pub enum [<PingPongIteratorIndex $index_type_number>] {
                $(
                    $index_variant_name,
                )*
            }
            pub enum [<PingPongResponse $index_type_number>] {
                Single([<PingPongIteratorIndex $index_type_number>]),
                Range(Vec<[<PingPongIteratorIndex $index_type_number>]>),
                All,
            }

            pub trait [<CanPingPong $index_type_number>]<TOutput, $([<$index_variant_name>]),*, TFunc>
            where
                TFunc: Fn($(Option<&[<$index_variant_name>]>,)*) -> Result<([<PingPongResponse $index_type_number>], Vec<TOutput>), Box<dyn Error>>,
            {
                fn ping_pong(self, func: TFunc) -> Result<Vec<TOutput>, Box<dyn Error>>;
            }

            impl<$($index_variant_name),*, TOutput, TFunc> [<CanPingPong $index_type_number>]<TOutput, $($index_variant_name),*, TFunc> for ($(Vec<$index_variant_name>),*)
            where
                TFunc: Fn($(Option<&[<$index_variant_name>]>,)*) -> Result<([<PingPongResponse $index_type_number>], Vec<TOutput>), Box<dyn Error>>,
            {
                fn ping_pong(self, func: TFunc) -> Result<Vec<TOutput>, Box<dyn Error>> {
                    return ($(self.$index_variant_index.iter(),)*).ping_pong(func);
                }
            }

            impl<$($index_variant_name),*, TOutput, TFunc> [<CanPingPong $index_type_number>]<TOutput, $($index_variant_name),*, TFunc> for ($(Iter<'_, $index_variant_name>,)*)
            where
                TFunc: Fn($(Option<&[<$index_variant_name>]>,)*) -> Result<([<PingPongResponse $index_type_number>], Vec<TOutput>), Box<dyn Error>>,
            {
                fn ping_pong(self, func: TFunc) -> Result<Vec<TOutput>, Box<dyn Error>> {
                    $(
                        let mut [<iter $index_variant_index>] = self.$index_variant_index.into_iter();
                        let mut [<next $index_variant_index>] = [<iter $index_variant_index>].next();
                    )*

                    let mut outputs = Vec::with_capacity($([<iter $index_variant_index>].len() + )* 0);
                    while $([<next $index_variant_index>].is_some() || )* false {
                        let (response, mut output) = (func)($([<next $index_variant_index>],)*)?;
                        outputs.append(&mut output);

                        match response {
                            [<PingPongResponse $index_type_number>]::Single(index) => {
                                match index {
                                    $(
                                        [<PingPongIteratorIndex $index_type_number>]::$index_variant_name => {
                                            [<next $index_variant_index>] = [<iter $index_variant_index>].next();
                                        },
                                    )*
                                }
                            },
                            [<PingPongResponse $index_type_number>]::Range(range) => {
                                for index in range {
                                    match index {
                                        $(
                                            [<PingPongIteratorIndex $index_type_number>]::$index_variant_name => {
                                                [<next $index_variant_index>] = [<iter $index_variant_index>].next();
                                            },
                                        )*
                                    }
                                }
                            },
                            [<PingPongResponse $index_type_number>]::All => {
                                $(
                                    [<next $index_variant_index>] = [<iter $index_variant_index>].next();
                                )*
                            },
                        }
                    }

                    Ok(outputs)
                }
            }
        }
    };
}

macro_rules! prelude {
    ($([$index_type_number:ty [$($index_variant_index:literal => $index_variant_name:ident,)*]],)*) => {
        paste::paste! {
            pub mod prelude {
                #[allow(unused_imports)]
                pub use super::{
                    $(
                        [<PingPongIteratorIndex $index_type_number>],
                        [<PingPongResponse $index_type_number>],
                        [<CanPingPong $index_type_number>],
                    )*
                };
            }
            $(
                ping_pong!($index_type_number [$($index_variant_index => $index_variant_name),*]);
            )*
        }
    };
}

prelude!(
    [Two [
        0 => T1,
        1 => T2,
    ]],
    [Three [
        0 => T1,
        1 => T2,
        2 => T3,
    ]],
    [Four [
        0 => T1,
        1 => T2,
        2 => T3,
        3 => T4,
    ]],
    [Five [
        0 => T1,
        1 => T2,
        2 => T3,
        3 => T4,
        4 => T5,
    ]],
    [Six [
        0 => T1,
        1 => T2,
        2 => T3,
        3 => T4,
        4 => T5,
        5 => T6,
    ]],
    [Seven [
        0 => T1,
        1 => T2,
        2 => T3,
        3 => T4,
        4 => T5,
        5 => T6,
        6 => T7,
    ]],
    [Eight [
        0 => T1,
        1 => T2,
        2 => T3,
        3 => T4,
        4 => T5,
        5 => T6,
        6 => T7,
        7 => T8,
    ]],
);

mod tests {
    #[allow(unused_imports)]
    use super::prelude::*;

    #[test]
    fn test_e8l1_small_example() {

        // lists already sorted by age
        let chicago = vec![
            ("John", 20),
            ("Jane", 28),
            ("John", 31),
            ("Al", 58),
        ];
        let new_york = vec![
            ("Bill", 19),
            ("John", 20),
            ("Sally", 31),
            ("Frank", 40),
        ];

        let expected_length = chicago.len() + new_york.len();

        let names = (chicago, new_york).ping_pong(|chicago, new_york| {
            if let (Some(chicago), Some(new_york)) = (chicago, new_york) {
                if chicago.1 < new_york.1 {
                    Ok((PingPongResponseTwo::Single(PingPongIteratorIndexTwo::T1), vec![chicago.0]))
                }
                else if chicago.1 > new_york.1 {
                    Ok((PingPongResponseTwo::Single(PingPongIteratorIndexTwo::T2), vec![new_york.0]))
                }
                else {
                    if chicago.0 < new_york.0 {
                        Ok((PingPongResponseTwo::Single(PingPongIteratorIndexTwo::T1), vec![chicago.0]))
                    }
                    else {
                        Ok((PingPongResponseTwo::Single(PingPongIteratorIndexTwo::T2), vec![new_york.0]))
                    }
                }
            }
            else if let Some(chicago) = chicago {
                Ok((PingPongResponseTwo::Single(PingPongIteratorIndexTwo::T1), vec![chicago.0]))
            }
            else if let Some(new_york) = new_york {
                Ok((PingPongResponseTwo::Single(PingPongIteratorIndexTwo::T2), vec![new_york.0]))
            }
            else {
                Err("Unexpected missing both types.".into())
            }
        })
            .unwrap();

        assert_eq!(expected_length, names.len());
        assert_eq!("Bill", names[0]); // 19
        assert_eq!("John", names[1]); // 20
        assert_eq!("John", names[2]); // 20
        assert_eq!("Jane", names[3]); // 28
        assert_eq!("John", names[4]); // 31
        assert_eq!("Sally", names[5]); // 31
        assert_eq!("Frank", names[6]); // 40
        assert_eq!("Al", names[7]); // 58
    }
}