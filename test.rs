mod follow {
    #![allow(non_snake_case)]
    use crate::{
        core::{
            msg_err_wrp, ok_data_msg_wrp, session::ParsedSessionState, state::HJ3State,
            ActixResult, MySimpleResponse,
        },
        entity::user,
        util::log_ise,
    };
    use actix_session::Session;
    use actix_web::{post, web};
    use anyhow::anyhow;
    use log::debug;
    use sea_orm::{
        sea_query::{
            Alias, BinOper, Expr, IntoIden, IntoTableRef, MysqlQueryBuilder, Query, SimpleExpr,
            SubQueryStatement,
        },
        ActiveModelTrait, ColumnTrait, DatabaseConnection, DeriveIden, DynIden, EntityTrait,
        EnumIter, ModelTrait, PaginatorTrait, QueryFilter, Selector, Set, Value,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::sync::Arc;
    pub struct ToggleFollowStateJson {
        pub target: i32,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ToggleFollowStateJson {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "target" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"target" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ToggleFollowStateJson>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ToggleFollowStateJson;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ToggleFollowStateJson",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ToggleFollowStateJson with 1 element",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(ToggleFollowStateJson { target: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "target",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("target") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(ToggleFollowStateJson { target: __field0 })
                    }
                }
                const FIELDS: &'static [&'static str] = &["target"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ToggleFollowStateJson",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ToggleFollowStateJson>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct Followship {
        pub uid: i32,
        pub username: String,
        pub email: String,
        pub followedByMe: bool,
        pub time: i64,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Followship {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "Followship",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "uid",
                    &self.uid,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "username",
                    &self.username,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "email",
                    &self.email,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "followedByMe",
                    &self.followedByMe,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "time",
                    &self.time,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct FollowshipResponse {
        pub code: i32,
        pub data: Vec<Followship>,
        pub pageCount: usize,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for FollowshipResponse {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "FollowshipResponse",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "code",
                    &self.code,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "data",
                    &self.data,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "pageCount",
                    &self.pageCount,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[allow(non_camel_case_types, missing_docs)]
    pub struct toggle_follow_state;
    impl ::actix_web::dev::HttpServiceFactory for toggle_follow_state {
        fn register(self, __config: &mut actix_web::dev::AppService) {
            pub async fn toggle_follow_state(
                session: Session,
                state: web::Data<HJ3State>,
                json: web::Json<ToggleFollowStateJson>,
            ) -> ActixResult<MySimpleResponse> {
                let uid = session.uid()?;
                let target = json.target;
                if uid == target {
                    return msg_err_wrp("绂佹鍏虫敞浣犺嚜宸?");
                }
                use crate::entity::follower::*;
                let query =
                    Entity::find().filter(Column::Source.eq(uid).and(Column::Target.eq(target)));
                let followed = if let Some(val) = query.one(&*state.db).await.map_err(log_ise)? {
                    val.delete(&*state.db).await.map_err(log_ise)?;
                    false
                } else {
                    let total_count = Entity::find()
                        .filter(Column::Source.eq(uid))
                        .count(&*state.db)
                        .await
                        .map_err(log_ise)?;
                    let max_follow = state.config.common.following_count_limit as usize;
                    if total_count >= max_follow {
                        return msg_err_wrp(&{
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &[
                                    "\u{4f60}\u{6700}\u{591a}\u{53ea}\u{80fd}\u{5173}\u{6ce8} ",
                                    " \u{4eba}!",
                                ],
                                &[::core::fmt::ArgumentV1::new_display(&max_follow)],
                            ));
                            res
                        });
                    }
                    ActiveModel {
                        source: Set(uid),
                        target: Set(target),
                        time: Set(chrono::Local::now().naive_local()),
                    }
                    .insert(&*state.db)
                    .await
                    .map_err(log_ise)?;
                    true
                };
                return ok_data_msg_wrp(
                    &::serde_json::Value::Object({
                        let mut object = ::serde_json::Map::new();
                        let _ = object.insert(
                            ("followed").into(),
                            ::serde_json::to_value(&followed).unwrap(),
                        );
                        object
                    }),
                    "鎿嶄綔瀹屾垚!",
                );
            }
            let __resource = ::actix_web::Resource::new("/toggle_follow_state")
                .name("toggle_follow_state")
                .guard(::actix_web::guard::Post())
                .to(toggle_follow_state);
            ::actix_web::dev::HttpServiceFactory::register(__resource, __config)
        }
    }
    enum FollowshipQuery {
        SelfFollowing(i32),
        FollowingSelf(i32),
    }
    pub enum ResultCol {
        Target,
        Source,
        Username,
        Email,
        Time,
        SelfFollowed,
    }
    #[allow(missing_docs)]
    pub struct ResultColIter {
        idx: usize,
        back_idx: usize,
        marker: ::core::marker::PhantomData<()>,
    }
    impl ResultColIter {
        fn get(&self, idx: usize) -> Option<ResultCol> {
            match idx {
                0usize => ::core::option::Option::Some(ResultCol::Target),
                1usize => ::core::option::Option::Some(ResultCol::Source),
                2usize => ::core::option::Option::Some(ResultCol::Username),
                3usize => ::core::option::Option::Some(ResultCol::Email),
                4usize => ::core::option::Option::Some(ResultCol::Time),
                5usize => ::core::option::Option::Some(ResultCol::SelfFollowed),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl sea_orm::strum::IntoEnumIterator for ResultCol {
        type Iterator = ResultColIter;
        fn iter() -> ResultColIter {
            ResultColIter {
                idx: 0,
                back_idx: 0,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl Iterator for ResultColIter {
        type Item = ResultCol;
        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            self.nth(0)
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            let t = if self.idx + self.back_idx >= 6usize {
                0
            } else {
                6usize - self.idx - self.back_idx
            };
            (t, Some(t))
        }
        fn nth(&mut self, n: usize) -> Option<<Self as Iterator>::Item> {
            let idx = self.idx + n + 1;
            if idx + self.back_idx > 6usize {
                self.idx = 6usize;
                None
            } else {
                self.idx = idx;
                self.get(idx - 1)
            }
        }
    }
    impl ExactSizeIterator for ResultColIter {
        fn len(&self) -> usize {
            self.size_hint().0
        }
    }
    impl DoubleEndedIterator for ResultColIter {
        fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
            let back_idx = self.back_idx + 1;
            if self.idx + back_idx > 6usize {
                self.back_idx = 6usize;
                None
            } else {
                self.back_idx = back_idx;
                self.get(6usize - self.back_idx)
            }
        }
    }
    impl Clone for ResultColIter {
        fn clone(&self) -> ResultColIter {
            ResultColIter {
                idx: self.idx,
                back_idx: self.back_idx,
                marker: self.marker.clone(),
            }
        }
    }
    impl sea_query::Iden for ResultCol {
        fn unquoted(&self, s: &mut dyn sea_query::Write) {
            match self {
                Self::Target => s
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&"target")],
                    ))
                    .unwrap(),
                Self::Source => s
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&"source")],
                    ))
                    .unwrap(),
                Self::Username => s
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&"username")],
                    ))
                    .unwrap(),
                Self::Email => s
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&"email")],
                    ))
                    .unwrap(),
                Self::Time => s
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&"time")],
                    ))
                    .unwrap(),
                Self::SelfFollowed => s
                    .write_fmt(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&"self_followed")],
                    ))
                    .unwrap(),
            };
        }
    }
    async fn generate_followship_list(
        query: FollowshipQuery,
        me: Option<i32>,
        page: usize,
        page_size: usize,
        db: &DatabaseConnection,
    ) -> ActixResult<FollowshipResponse> {
        use crate::entity::follower::*;
        let f1: DynIden = Arc::new(Alias::new("f1"));
        let f2: DynIden = Arc::new(Alias::new("f2"));
        let self_followed: DynIden = Arc::new(Alias::new("self_followed"));
        let count = Entity::find()
            .filter(match &query {
                FollowshipQuery::SelfFollowing(v) => Column::Source.eq(*v),
                FollowshipQuery::FollowingSelf(v) => Column::Target.eq(*v),
            })
            .count(db)
            .await
            .map_err(|e| {
                ::anyhow::Error::msg({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Failed to perform database query: "],
                        &[::core::fmt::ArgumentV1::new_display(&e)],
                    ));
                    res
                })
            })
            .map_err(log_ise)?;
        let page_count = count / page_size + ((count % page_size != 0) as usize);
        let stmt = Query::select()
            .from_as(Entity.into_table_ref(), f1.clone())
            .column(Column::Target)
            .column(Column::Source)
            .column(user::Column::Username)
            .column(user::Column::Email)
            .column(Column::Time)
            .expr_as(
                match me {
                    Some(me) => SimpleExpr::Binary(
                        Box::new(SimpleExpr::SubQuery(Box::new(
                            SubQueryStatement::SelectStatement(
                                Query::select()
                                    .from_as(Entity.into_table_ref(), f2.clone())
                                    .expr(Expr::asterisk().count())
                                    .and_where(
                                        Expr::tbl(f2.clone(), Column::Source.into_iden())
                                            .eq(Value::Int(Some(me)))
                                            .and(
                                                Expr::tbl(f2.clone(), Column::Target.into_iden())
                                                    .equals(
                                                        f1.clone(),
                                                        match &query {
                                                            FollowshipQuery::SelfFollowing(_) => {
                                                                Column::Target.into_iden()
                                                            }
                                                            FollowshipQuery::FollowingSelf(_) => {
                                                                Column::Source.into_iden()
                                                            }
                                                        },
                                                    ),
                                            ),
                                    )
                                    .to_owned(),
                            ),
                        ))),
                        BinOper::NotEqual,
                        Box::new(SimpleExpr::Value(Value::Int(Some(0)))),
                    ),
                    None => SimpleExpr::Value(Value::Bool(Some(false))),
                },
                self_followed.clone(),
            )
            .join(
                sea_orm::JoinType::Join,
                user::Entity.into_table_ref(),
                Expr::tbl(user::Entity.into_iden(), user::Column::Id.into_iden()).equals(
                    f1.clone(),
                    match &query {
                        FollowshipQuery::SelfFollowing(_) => Column::Target,
                        FollowshipQuery::FollowingSelf(_) => Column::Source,
                    },
                ),
            )
            .and_where(match &query {
                FollowshipQuery::SelfFollowing(src_uid) => {
                    Expr::tbl(f1.clone(), Column::Source.into_iden()).eq(Value::Int(Some(*src_uid)))
                }
                FollowshipQuery::FollowingSelf(tgt_uid) => {
                    Expr::tbl(f1.clone(), Column::Target.into_iden()).eq(Value::Int(Some(*tgt_uid)))
                }
            })
            .limit(page_size as u64)
            .offset((page_size * (page - 1)) as u64)
            .to_owned();
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["Followship query:\n"],
                        &[::core::fmt::ArgumentV1::new_display(
                            &stmt.build(MysqlQueryBuilder).0,
                        )],
                    ),
                    lvl,
                    &(
                        "hellojudge3_api_server::route::user::follow",
                        "hellojudge3_api_server::route::user::follow",
                        "src\\route\\user\\follow.rs",
                        203u32,
                    ),
                );
            }
        };
        let entries = Selector::with_columns::<
            (i32, i32, String, String, chrono::NaiveDateTime, bool),
            ResultCol,
        >(stmt)
        .all(db)
        .await
        .map_err(log_ise)?;
        ::core::panicking::panic("not yet implemented");
    }
}
