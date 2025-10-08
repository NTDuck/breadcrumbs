pub mod pagination {
    use ::futures::prelude::*;

    #[derive(::core::clone::Clone, ::core::marker::Copy)]
    #[derive(::bon::Builder)]
    #[builder(const)]
    pub struct PaginationRequest {
        pub page_number: usize,
        pub max_page_size: usize,
    }

    impl PaginationRequest {
        pub const fn unbounded() -> Self {
            Self::builder()
                .page_number(MIN_PAGE_NUMBER)
                .max_page_size(::core::usize::MAX)
                .build()
        }
    }

    pub struct PaginationResponse<Item> {
        pub items: ::std::vec::Vec<Item>,

        pub page_size: usize,
        pub max_page_size: usize,
        pub page_number: usize,
        pub max_page_number: usize,
    }

    #[::bon::bon]

    impl<Item> PaginationResponse<Item> {
        #[builder(builder_type(vis = "pub"))]
        fn new(
            items: ::std::vec::Vec<Item>,
            request: PaginationRequest,
            total: usize,
        ) -> Self {
            let page_size = items.len();

            let PaginationRequest { page_number, max_page_size } = request;

            let max_page_number = match total {
                0 => MIN_PAGE_NUMBER,
                _ => total.div_ceil(max_page_size),
            };

            Self {
                items,

                page_size,
                max_page_size,
                page_number,
                max_page_number,
            }
        }
    }

    impl<Item> PaginationResponse<Item> {
        pub async fn map<Mapper, Future, MappedItem>(self, mut mapper: Mapper) -> ::aliases::result::Fallible<PaginationResponse<MappedItem>>
        where
            Mapper: ::core::ops::FnMut(Item) -> Future,
            Future: ::core::future::Future<Output = ::aliases::result::Fallible<MappedItem>>,
        {
            let items = ::futures::stream::iter(self.items)
                .then(|item| (mapper)(item))
                .try_collect::<::std::vec::Vec<_>>()
                .await?;

            let Self { page_size, max_page_size, page_number, max_page_number, .. } = self;

            ::aliases::result::Fallible::Ok(PaginationResponse {
                items,

                page_size,
                max_page_size,
                page_number,
                max_page_number,
            })
        }
    }

    impl<Item> ::core::ops::Deref for PaginationResponse<Item> {
        type Target = ::std::vec::Vec<Item>;

        fn deref(&self) -> &::std::vec::Vec<Item> {
            &self.items
        }
    }

    #[derive(::core::clone::Clone, ::core::marker::Copy)]
    #[derive(::bon::Builder)]
    #[builder(const)]
    pub struct PaginationRange {
        pub offset: usize,
        pub limit: usize,
    }

    impl ::core::convert::From<PaginationRange> for ::core::ops::Range<usize> {
        fn from(range: PaginationRange) -> Self {
            range.offset .. range.offset + range.limit
        }
    }

    impl ::core::convert::From<PaginationRequest> for PaginationRange {
        fn from(request: PaginationRequest) -> Self {
            let PaginationRequest { page_number, max_page_size } = request;

            let offset = page_number
                .saturating_sub(MIN_PAGE_NUMBER)
                .saturating_mul(max_page_size);
            let limit = max_page_size;

            Self::builder()
                .offset(offset)
                .limit(limit)
                .build()
        }
    }

    const MIN_PAGE_NUMBER: usize = 1;
}
