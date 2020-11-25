[![Build Status](https://travis-ci.com/immortalinfidel/roc-rs.svg?branch=master)](https://travis-ci.com/immortalinfidel/roc-rs)
# roc-rs
Rate Of Change (ROC) implementation in rust
```
use roc_rs::ROC;
use ta_common::traits::Indicator;

let mut roc = ROC::new(1, None);
let res = roc.next(100 as f64);
assert_eq!(None, res);
let res = roc.next(50 as f64);
assert_eq!(Some(-50_f32), res);

```
### Calculation
ROCR=(Current Price/Price Prev n Ago);  
ROCP=ROCR-1;  
ROC100=(ROCR)*100;  
ROC=(ROCR-1)*100; //DEFAULT  