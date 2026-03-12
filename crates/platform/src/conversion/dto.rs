pub trait IntoVecDto<T> {
    fn to_dto(self) -> Vec<T>;
}

impl<T, U> IntoVecDto<U> for Vec<T>
where
    U: From<T>,
{
    fn to_dto(self) -> Vec<U> {
        self.into_iter().map(Into::into).collect()
    }
}
