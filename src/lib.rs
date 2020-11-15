use std::ops::{Div, Sub};
use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Indicator;


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
        let size = history.size() as i32;
        let period = self.period as i32;
        let prev_index: i32 = (size-period) as i32;
        let prev = history.at(prev_index);

        match prev {
            None=>None,
            Some(val)=>Some(input.div(val) as f32)
        }

    }
}


impl Indicator<f64, Option<f32>> for ROC {
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

    fn reset(&mut self) {
        self.history.clear();
    }
}


#[cfg(test)]
mod tests {
    use crate::{ROC, ROCType};
    use ta_common::traits::{ Indicator};

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
