use sink::Sink;
use std::cell::RefCell;
use std::fmt::{ Debug };

#[derive(Debug)]
pub enum Error {
    Overflow,
}

/// Vec is a simple accumulator to capture signals from a source
#[derive(Clone)]
pub struct VecSink<TInput>
where
    TInput: Clone,
{
    data: RefCell<Vec<TInput>>,
}

impl<TInput> VecSink<TInput>
where
    TInput: Clone + Debug,
{
    pub fn new() -> Self {
        VecSink {
            data: RefCell::new(vec![]),
        }
    }

    pub fn data(&self) -> Vec<TInput> {
        let data = self.data.borrow();
        data.clone()
    }

    fn push(&self, input: TInput) -> Result<usize, Error> {
        let mut data = self.data.borrow_mut();
        (*data).push(input);
        println!("Data: {:?}", *data);
        Ok(data.len() - 1)
    }
}

impl<TInput> Sink for VecSink<TInput>
where
    TInput: Clone + Debug,
{
    type TInput = TInput;
    type TResult = Result<usize, Error>;

    fn send(&self, input: Self::TInput) -> Self::TResult {
        self.push(input)
    }
}

impl<'a, TInput> Sink for &'a VecSink<TInput>
where
    TInput: Clone + Debug,
{
    type TInput = TInput;
    type TResult = Result<usize, Error>;

    fn send(&self, input: Self::TInput) -> Self::TResult {
        self.push(input)
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn populate_vecsink_with_handled_message() {
        let sink = VecSink::new();
        sink.send(10).unwrap();
        sink.send(20).unwrap();
        assert_eq!(vec![10, 20], sink.data());
    }

    #[test]
    fn return_data_index_as_result_handle() {
        let sink = VecSink::new();
        assert_eq!(0, sink.send(10).unwrap());
        assert_eq!(1, sink.send(20).unwrap());
    }
}
