use std::ops::{Div, Sub};
use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Next;


pub enum ROCType {
    ROCR,
    ROCP,
    ROC100,
    ROC,
}

pub struct ROC {
    history: FixedQueue<f64>,
    period: u32,
    result_type: ROCType,
}

impl ROC {
    pub fn new(period: u32, result_type: Option<ROCType>) -> ROC {
        let result_type = result_type.unwrap_or(ROCType::ROC);
        Self {
            history: FixedQueue::new(period),
            period,
            result_type,
        }
    }
    pub fn calc(&self, input: f64) -> Option<f32> {
        let history = &self.history;
        let out_len = history.size() as i32;
        let period = self.period as i32;
        let prev_index: i32 = (out_len.sub(period)) as i32;
        if prev_index < 0 {
            return None;
        }
        let prev = history.at(prev_index as u32);
        let roc = input.div(prev) as f32;
        Some(roc)
    }
}


impl Next<f64, Option<f32>> for ROC {
    fn next(&mut self, input: f64) -> Option<f32> {
        let value = self.calc(input);
        let result = match &self.result_type {
            ROCType::ROCR => value,
            ROCType::ROCP => value.map(|v| v - 1_f32),
            ROCType::ROC => value.map(|v| (v - 1_f32) * 100_f32),
            ROCType::ROC100 => value.map(|v| v * 100_f32),
        };
        self.history.add(input);
        result
    }
}


#[cfg(test)]
mod tests {
    use crate::{ROC, ROCType};
    use ta_common::traits::Next;

    #[test]
    fn roc_percent_works() {
        let mut roc = ROC::new(1, None);
        let res = roc.next(100 as f64);
        assert_eq!(None, res);
        let res = roc.next(50 as f64);
        assert_eq!(Some(-50_f32), res);
    }

    #[test]
    fn roc_value_works() {
        let mut roc = ROC::new(1, Some(ROCType::ROCR));
        let res = roc.next(100 as f64);
        assert_eq!(None, res);
        let res = roc.next(50 as f64);
        assert_eq!(Some(0.5_f32), res);
    }
    #[test]
    fn roc_hundred_works() {
        let mut roc = ROC::new(1, Some(ROCType::ROC100));
        let res = roc.next(100 as f64);
        assert_eq!(None, res);
        let res = roc.next(50 as f64);
        assert_eq!(Some(50_f32), res);
    }
    #[test]
    fn roc_momentum_works() {
        let mut roc = ROC::new(1, Some(ROCType::ROCP));
        let res = roc.next(100 as f64);
        assert_eq!(None, res);
        let res = roc.next(50 as f64);
        assert_eq!(Some(-0.5_f32), res);
    }
}
