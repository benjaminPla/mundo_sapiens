use poll_promise::Promise;

use crate::application::sellers::errors::SellersAppError;
use crate::application::sellers::list::SellersListUseCase;
use crate::domain::sellers::Seller;
use crate::infrastructure::sellers::pg_sellers_repository::PgSellersRepository;
use crate::presentation::promise::spawn;

type SellersPromise = Promise<Result<Vec<Seller>, SellersAppError>>;

#[derive(Default)]
pub struct AppData {
    sellers: Option<SellersPromise>,
}

impl AppData {
    pub fn sellers(&mut self, ctx: &egui::Context, runtime: &tokio::runtime::Handle, sellers_repo: &PgSellersRepository) -> Option<&[Seller]> {
        let promise = self.sellers.get_or_insert_with(|| {
            let use_case = SellersListUseCase::new(sellers_repo.clone());
            spawn(ctx, runtime, async move { use_case.execute().await })
        });

        match promise.ready() {
            Some(Ok(sellers)) => Some(sellers.as_slice()),
            Some(Err(err))    => {
                eprintln!("Failed to list sellers: {err}"); // TODO: surface error in UI
                None
            }
            None => None,
        }
    }

    pub fn invalidate_sellers(&mut self) {
        self.sellers = None;
    }
}
