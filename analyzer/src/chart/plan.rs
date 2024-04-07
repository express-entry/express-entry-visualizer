use crate::analyze::non_pnp::PnpRemover;
use crate::chart::dataset::{ChartData, LineDataset, Tooltip};
use crate::chart::utils::{ToTimestamp, SERIALIZER};
use crate::data::{Invite, Plan, Pool};
use crate::utils::console_log;
use chrono::{Datelike, NaiveDate};
use itertools::Itertools;
use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

trait FromYear {
    fn y(year: i32) -> Self;
}

impl FromYear for NaiveDate {
    fn y(year: i32) -> Self {
        NaiveDate::from_yo_opt(year, 181).unwrap()
    }
}

#[wasm_bindgen]
pub fn wasm_plan_size_data(
    pool_data: *const Vec<Pool>,
    invite_data: *const Vec<Invite>,
    plan_data: *const Vec<Plan>,
) -> JsValue {
    let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    let plan_data = unsafe { plan_data.as_ref().unwrap_throw() };
    let invite_data = PnpRemover::remove_pnp(pool_data, invite_data);

    console_log!("{:?}", plan_data);

    let labels: Vec<_> = plan_data
        .iter()
        .map(|plan| NaiveDate::y(plan.year).to_timestamp() as f64)
        .collect();

    let plan_min = {
        let data: Vec<_> = plan_data.iter().map(|plan| Some(plan.min)).collect();

        LineDataset {
            label: "Planed".into(),
            data,
            background_color: "#F4D03F".into(),
            border_color: "#F4D03F".into(),
            border_dash: [5.0, 5.0],
            ..Default::default()
        }
    };

    let plan_max = {
        let data: Vec<_> = plan_data.iter().map(|plan| Some(plan.max)).collect();

        LineDataset {
            label: "none".into(),
            data,
            background_color: "#F4D03F".into(),
            border_color: "#F4D03F".into(),
            border_dash: [5.0, 5.0],
            ..Default::default()
        }
    };

    let actual = {
        let map: HashMap<_, _> = invite_data
            .iter()
            .group_by(|invite| invite.date.year())
            .into_iter()
            .map(|(year, invites)| (year, invites.map(|invite| invite.size).sum::<f64>() as f64))
            .collect();

        let data: Vec<_> = plan_data
            .iter()
            .map(|plan| map.get(&plan.year).copied())
            .collect();

        LineDataset {
            label: "none".into(),
            data,
            background_color: "#58D68D".into(),
            border_color: "#58D68D".into(),
            ..Default::default()
        }
    };

    let datasets = vec![plan_min, plan_max, actual];

    let tooltip_title: Vec<_> = plan_data
        .iter()
        .map(|plan| format!("{}", plan.year))
        .collect();

    ChartData {
        labels,
        datasets,
        tooltip: Tooltip {
            title: vec![tooltip_title],
            label: Vec::new(),
        },
    }
    .serialize(&SERIALIZER)
    .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_plan_x_min(plan_data: *const Vec<Plan>) -> JsValue {
    let plan_data = unsafe { plan_data.as_ref().unwrap_throw() };
    plan_data
        .first()
        .map(|plan| NaiveDate::y(plan.year - 1).to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap()
}

#[wasm_bindgen]
pub fn wasm_plan_x_max(plan_data: *const Vec<Plan>) -> JsValue {
    let plan_data = unsafe { plan_data.as_ref().unwrap_throw() };
    plan_data
        .last()
        .map(|plan| NaiveDate::y(plan.year + 1).to_timestamp() as f64)
        .unwrap_or(0.0)
        .serialize(&SERIALIZER)
        .unwrap()
}
