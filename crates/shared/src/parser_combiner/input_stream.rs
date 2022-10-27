// struct InputStream<Item> {
//     data: Vec<Item>,
//     cur: usize,
// }
//
// impl<Item> InputStream<Item> {
//     pub fn new<Iter>(iter: Iter) -> Self
//     where
//         Iter: Iterator<Item = Item>,
//     {
//         InputStream {
//             data: iter.collect(),
//             cur: 0,
//         }
//     }
//     fn next(&mut self) -> Option<Item> {
//         match self.data.get(self.cur) {
//             Some(next) => {
//                 self.cur += 1;
//                 Some(next)
//             }
//             _ => None,
//         }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn tt() {}
//
//     // #[test]
//     // fn can_be_an_iterator() {
//     //     todo!()
//     // }
//     //
//     // #[test]
//     // fn countable() {
//     //     todo!()
//     // }
//     //
//     // #[test]
//     // fn can_slice() {
//     //     todo!()
//     // }
//     //
//     // #[test]
//     // fn can_dump_in_and_collect() {
//     //     todo!()
//     // }
//     //
//     // #[test]
//     // fn can_be_clone() {
//     //     todo!()
//     // }
//     //
//     // #[test]
//     // fn can_be_copy() {
//     //     todo!()
//     // }
// }
