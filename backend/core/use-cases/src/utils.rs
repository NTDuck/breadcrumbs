pub mod pagination {
    use ::futures::prelude::*;

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

    impl<Item, Items> PaginationResponse<Item, Items>
    where
        Items: ::core::iter::IntoIterator<Item = Item>,
    {
        pub async fn map<Mapper, Future, MappedItem, MappedItems>(self, mut mapper: Mapper) -> ::aliases::result::Fallible<PaginationResponse<MappedItem, MappedItems>>
        where
            Mapper: ::core::ops::FnMut(Item) -> Future,
            Future: ::core::future::Future<Output = ::aliases::result::Fallible<MappedItem>>,
            MappedItems: ::core::default::Default + ::core::iter::Extend<MappedItem> + ::core::iter::FromIterator<MappedItem> + ::core::iter::IntoIterator<Item = MappedItem>,
        {
            let response = PaginationResponse {
                items: ::futures::stream::iter(self.items)
                    .then(|item| (mapper)(item))
                    .try_collect::<MappedItems>()
                    .await?,

                page_size: self.page_size,
                max_page_size: self.max_page_size,
                page_number: self.page_number,
                max_page_number: self.max_page_number,
            };

            ::aliases::result::Fallible::Ok(response)
        }
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
