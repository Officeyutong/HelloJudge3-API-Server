// use log::debug;
// use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryTrait, Set};

use crate::core::{state::HJ2State, ResultType};
// use crate::entity::problem;
// use crate::entity::{user, User};
pub async fn my_test(_state: &HJ2State) -> ResultType<()> {
    // // let val = Problem::find_by_id(1389).one(&state.conn).await?.unwrap();
    // // let vs = User::find()
    // //     .--exec print("我好菜啊")(problem::Entity)
    // //     .all(&state.conn)
    // //     .await?;
    // let vs = User::find()
    //     .filter(user::Column::Id.eq(1))
    //     .find_with_related(problem::Entity)
    //     .all(&state.conn)
    //     .await?;
    // debug!("{:#?}", vs);
    return Ok(());
}
