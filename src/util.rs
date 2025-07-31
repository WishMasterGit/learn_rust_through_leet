#[derive(Debug, PartialEq)]
pub enum Status {
    Success,
    Fail,
}
pub struct Result<T> {
    pub status: Status,
    pub data: T,
}

impl <T> Result<T> 
where T: Default {
    pub fn new(status: Status, data: T) -> Self {
        Result { status, data }
    }

    pub fn fail() -> Self {
        Result {
            status: Status::Fail,
            data: T::default(),
        }
    }

    pub fn success(data:T) -> Self{
        Result { status: Status::Success, data: data }
    }
}

pub fn split_to_digits(num:i32) -> Vec<i32>{
    let mut number = num.clone(); 
    if number == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    while number > 0 {
        digits.push(number % 10);
        number /= 10;
    }
    digits.reverse();
    return digits;
}
