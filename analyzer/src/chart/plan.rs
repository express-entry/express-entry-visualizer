use crate::analyze::non_pnp::PnpRemover;
use crate::chart::dataset::{ChartData, LineDataset, Tooltip};
use crate::chart::utils::{ToTimestamp, SERIALIZER};
use crate::data::{Invite, Plan, Pool};
use chrono::{Datelike, NaiveDate};
use itertools::Itertools;
use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use super::dataset::{BarDataset, Label, PieDataset};

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

    let labels: Vec<_> = plan_data
        .iter()
        .map(|plan| Label::from(NaiveDate::y(plan.year).to_timestamp() as f64))
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
            label: "Actual".into(),
            data,
            background_color: "#58D68D".into(),
            border_color: "#58D68D".into(),
            ..Default::default()
        }
    };

    let tooltip_title: Vec<_> = plan_data
        .iter()
        .map(|plan| format!("{}", plan.year))
        .collect();

    let tooltip_label_min: Vec<_> = plan_min
        .data
        .iter()
        .map(|count| match count {
            Some(count) => format!("Planed: {}", count),
            None => "".into(),
        })
        .collect();

    let tooltip_label_max: Vec<_> = plan_max
        .data
        .iter()
        .map(|count| match count {
            Some(count) => format!("Planed: {}", count),
            None => "".into(),
        })
        .collect();

    let tooltip_label_actual: Vec<_> = {
        let target = plan_min
            .data
            .iter()
            .zip(plan_max.data.iter())
            .map(|(minimum, maximum)| (minimum.unwrap_or(0.0) + maximum.unwrap_or(0.0)) / 2.0);

        actual
            .data
            .iter()
            .zip(target)
            .map(|(count, target)| match count {
                Some(count) => format!("Actual: {} ({:.2}%)", count, (count / target) * 100.0),
                None => "".into(),
            })
            .collect()
    };

    ChartData {
        labels,
        datasets: vec![plan_min, plan_max, actual],
        tooltip: Tooltip {
            title: vec![tooltip_title],
            label: vec![tooltip_label_min, tooltip_label_max, tooltip_label_actual],
        },
    }
    .serialize(&SERIALIZER)
    .unwrap_throw()
}

#[wasm_bindgen]
pub fn wasm_plan_pie_data(
    pool_data: *const Vec<Pool>,
    invite_data: *const Vec<Invite>,
    plan_data: *const Vec<Plan>,
    year: f64,
) -> JsValue {
    let invite_data = unsafe { invite_data.as_ref().unwrap_throw() };
    let pool_data = unsafe { pool_data.as_ref().unwrap_throw() };
    let plan_data = unsafe { plan_data.as_ref().unwrap_throw() };
    let invite_data = PnpRemover::remove_pnp(pool_data, invite_data);
    let year = year as i32;

    let labels: Vec<_> = vec![Label::from("Used"), Label::from("Unused")];

    let plan_min = plan_data
        .iter()
        .filter(|plan| plan.year == year)
        .map(|plan| plan.min)
        .exactly_one()
        .unwrap_or(0.0);
    let plan_max = plan_data
        .iter()
        .filter(|plan| plan.year == year)
        .map(|plan| plan.max)
        .exactly_one()
        .unwrap_or(0.0);
    let target = (plan_min + plan_max) / 2.0;

    let actual = invite_data
        .iter()
        .filter(|invite| invite.date.year() == year)
        .map(|invite| invite.size)
        .sum();

    let used = actual;
    let unused = f64::max(0.0, target - actual);

    let dataset = PieDataset {
        data: vec![used, unused],
        background_color: vec!["#58D68D".into(), "#F4D03F".into()],
        border_color: vec!["#58D68D".into(), "#F4D03F".into()],
        ..Default::default()
    };

    let label_used = format!("{} ({:.2}%)", used, used / (used + unused) * 100.0);
    let label_unused = format!("{} ({:.2}%)", unused, unused / (used + unused) * 100.0);

    ChartData {
        labels,
        datasets: vec![dataset],
        tooltip: Tooltip {
            title: Default::default(),
            label: vec![vec![label_used, label_unused]],
        },
    }
    .serialize(&SERIALIZER)
    .unwrap_throw()
}
