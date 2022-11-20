use surrealdb::sql::Object;

pub struct Ctx();

#[allow(dead_code)]
pub struct Record<T> {
	data: T,
}

pub struct Result<T>(pub T);

pub async fn bmc_get<T>(
	_ctx: std::sync::Arc<Ctx>,
	_entity: &'static str,
	_id: &str,
) -> Result<Record<T>>
where
	T: Default,
{
	Result(Record { data: T::default() })
}

pub async fn bmc_create(
	_ctx: std::sync::Arc<Ctx>,
	_entity: &'static str,
	_data: Object,
) -> Result<String> {
	Result("test".into())
}

pub async fn bmc_delete(
	_ctx: std::sync::Arc<Ctx>,
	_entity: &'static str,
	_id: &str,
) -> Result<String> {
	Result("test".into())
}

pub async fn bmc_update(
	_ctx: std::sync::Arc<Ctx>,
	_entity: &'static str,
	_id: &str,
	_data: Object,
) -> Result<String> {
	Result("test".into())
}

pub async fn bmc_select<T>(
	_ctx: std::sync::Arc<Ctx>,
	_entity: &'static str,
	_filter: Option<String>,
	_sort: Option<String>,
) -> Result<Vec<Record<T>>> {
	Result(vec![])
}
