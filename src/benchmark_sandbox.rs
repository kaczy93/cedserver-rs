// #[cfg(test)]
// mod tests {
//     use test::Bencher;
// 
//     #[bench]
//     fn bench1(b: &mut Bencher) {
//         b.iter(|| println!("1"));
//     }
// 
//     #[bench]
//     fn bench2(b: &mut Bencher) {
//         b.iter(|| println!("{}", 1));
//     }
// 
//     #[bench]
//     fn bench3(b: &mut Bencher) {
//         b.iter(|| println!("{}", format!("{}", 1)));
//     }
// }