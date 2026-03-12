#[macro_export]
macro_rules! impl_dto_from {
    ($domain:ty => $dto:ty { $($field:ident),* $(,)? }) => {

        impl From<$domain> for $dto {
            fn from(domain: $domain) -> Self {
                Self {
                    $(
                        $field: domain.$field,
                    )*
                }
            }
        }

        impl From<&$domain> for $dto {
            fn from(domain: &$domain) -> Self {
                Self {
                    $(
                        $field: domain.$field.clone(),
                    )*
                }
            }
        }
    };
}
