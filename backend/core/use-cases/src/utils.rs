pub mod aliases {
    pub mod result {
        pub type Fallible<T = ()> = ::core::result::Result<T, ::anyhow::Error>;
    }
}

pub mod pagination {
    #[derive(::bon::Builder)]
    #[builder(const)]
    pub struct PaginationRequest {
        pub page_number: usize,
        pub max_page_size: usize,
    }

    impl PaginationRequest {
        pub const fn unbounded() -> Self {
            Self::builder()
                .page_number(Self::MIN_PAGE_NUMBER)
                .max_page_size(::core::usize::MAX)
                .build()
        }

        const MIN_PAGE_NUMBER: usize = 1;
    }

    #[derive(::bon::Builder)]
    pub struct PaginationResponse<Item, Items = ::std::vec::Vec<Item>>
    where
        Items: ::core::iter::IntoIterator<Item = Item>,
    {
        pub items: Items,

        pub page_size: usize,
        pub max_page_size: usize,
        pub page_number: usize,
        pub max_page_number: usize,
    }

    impl<Item, Items> ::core::ops::Deref for PaginationResponse<Item, Items>
    where
        Items: ::core::iter::IntoIterator<Item = Item>,
    {
        type Target = Items;

        fn deref(&self) -> &Items {
            &self.items
        }
    }
}
