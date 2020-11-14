# roc-rs
Rate Of Change (ROC) implementation in rust
```
let mut roc = ROC::new(1, None);
let res = roc.next(100 as f64);
assert_eq!(None, res);
let res = roc.next(50 as f64);
assert_eq!(Some(-50_f32), res);

```