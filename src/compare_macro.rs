//macro_rules! compare_and_map {
//    ( $type_name_0:ty => $type_name_1:ty ) => {
//        paste::paste! {
//            pub trait [<CompareAndMap $type_name_0 To $type_name_1>] {
//                fn compare_and_map(&self, other: &$type_name_1, map: impl Fn(Option<$type_name_0>, Option<$type_name_1>) -> ());
//            }
//
//            impl [<CompareAndMap $type_name_0 To $type_name_1>] for $type_name_0 {
//                fn compare_and_map(&self, other: &$type_name_1, map: impl Fn(Option<$type_name_0>, Option<$type_name_1>) -> ()) {
//
//                }
//            }
//        }
//    };
//}
//
//macro_rules! compare_all {
//    ($all:tt) => {
//    };
//}
//
//struct First;
//struct Second;
//struct Third;
//
//impl First {
//    fn get_tick(&self) -> Tick {
//        Tick { generation_time: 1 }
//    }
//}
//impl Second {
//    fn get_tick(&self) -> Tick {
//        Tick { generation_time: 2 }
//    }
//}
//impl Third {
//    fn get_tick(&self) -> Tick {
//        Tick { generation_time: 3 }
//    }
//}
//
//struct Tick {
//    generation_time: i32,
//}
//
//compare_and_map!(First => Second);
//
//compare_all!(
//    Snapshot => Client => GameEvent => ServerEvent
//)