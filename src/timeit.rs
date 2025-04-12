#[macro_export]
 macro_rules! timeit {
     ($duration_var:expr, $block:block) => {{
         use std::time::Instant;
         let start = Instant::now();
         let result = $block;
         let duration = start.elapsed();
         *$duration_var = duration;
         result
     }};
 }