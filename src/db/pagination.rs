const DEFAULT_PAGE_SIZE: i64 = 10;

pub struct Paginated<T> {
    query: T,
    page: i64,
    page_size: i64,
}

pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            page_size: DEFAULT_PAGE_SIZE,
            page,
        }
    }
}

impl<T: std::fmt::Display> Paginated<T> {
    pub fn paginated_query(&self) -> String {
        let limit = self.page_size;
        let offset = (self.page - 1) * self.page_size;
        format!(
            "SELECT *, COUNT(*) OVER() FROM ({sub}) t LIMIT {limit} OFFSET {offset}",
            sub = self.query,
            limit = limit,
            offset = offset
        )
    }
}
