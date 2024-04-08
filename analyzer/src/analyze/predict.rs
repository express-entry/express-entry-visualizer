use crate::{data::{CategoryCode, Invite, Plan, Pool}, utils::console_log};
use chrono::{Datelike, Days, NaiveDate, Weekday};
use itertools::Itertools;
use std::collections::HashSet;

use super::{
    calc::{CategoryPool, ScorePool},
    category::CategoryAnalyzer,
    non_pnp::PnpRemover,
    rate::RateAnalyzer,
    LOOKBEHIND_DAYS,
};

pub struct Predictor;

impl Predictor {
    fn enter_rate(
        pool_data: &[Pool],
        invite_data: &[Invite],
        start: NaiveDate,
        end: NaiveDate,
    ) -> ScorePool {
        let (label, rates) = RateAnalyzer::pool_increase_rate(pool_data, invite_data);
        let rate = RateAnalyzer::projected_rate(&label, &rates);
        ScorePool::max(rate.non_pnp(), ScorePool::zero()) * (end - start).num_days() as f64
    }

    fn category_date(
        pool_data: &[Pool],
        invite_data: &[Invite],
    ) -> (CategoryPool, CategoryPool, HashSet<CategoryCode>) {
        let last_date = invite_data.last().map(|invite| invite.date).unwrap();

        let invites_for_category: Vec<_> = invite_data
            .iter()
            .copied()
            .filter(|invite| (last_date - invite.date).num_days() < LOOKBEHIND_DAYS)
            .collect();
        let (_, count, c0) =
            CategoryAnalyzer::invite_per_category(pool_data, &invites_for_category);
        let invite_pct = count.last().unwrap().non_pnp().normalize();

        let (_, pct, c1) = CategoryAnalyzer::percent_per_category(pool_data, &invites_for_category);
        let pool_pct = pct
            .into_iter()
            .map(|pool| pool.non_pnp())
            .reduce(|x, y| x.max(y))
            .unwrap();

        let mut categories: HashSet<_> = c0.union(&c1).copied().collect();
        categories.remove(&CategoryCode::Province);
        (invite_pct, pool_pct, categories)
    }

    fn final_pool(pool_data: &[Pool], invite_data: &[Invite]) -> ScorePool {
        let pool_date = pool_data.last().unwrap().date;
        let mut pool = ScorePool::from(*pool_data.last().unwrap());
        for invite in invite_data {
            if invite.date > pool_date {
                let invite_as_pool = pool.invite(invite);
                pool = pool - invite_as_pool;
            }
        }
        pool.non_pnp()
    }

    fn leave_rate(
        pool_data: &[Pool],
        invite_data: &[Invite],
        plan_data: &[Plan],
        start: NaiveDate,
        end: NaiveDate,
    ) -> f64 {
        let invite_data = PnpRemover::remove_pnp(pool_data, invite_data);

        let invited = invite_data
            .iter()
            .filter(|invite| invite.date.year() == start.year())
            .map(|invite| invite.size)
            .sum::<f64>();

        let middle_years = plan_data
            .iter()
            .filter(|plan| start.year() <= plan.year && plan.year < end.year())
            .map(|plan| (plan.max + plan.min) / 2.0)
            .sum::<f64>();

        let last_year = plan_data
            .iter()
            .filter(|plan| plan.year == end.year())
            .exactly_one()
            .map(|plan| (plan.max + plan.min) / 2.0)
            .unwrap_or(0.0)
            * end.ordinal0() as f64
            / (365 + end.leap_year() as u32) as f64;

        return f64::max(0.0, middle_years + last_year - invited);
    }

    pub fn predict(
        pool_data: &[Pool],
        invite_data: &[Invite],
        plan_data: &[Plan],
    ) -> (Vec<NaiveDate>, Vec<CategoryPool>, HashSet<CategoryCode>) {
        let last_invite_date = invite_data.last().map(|invite| invite.date).unwrap();

        let (invite_pct, pool_pct, categories) = Self::category_date(pool_data, invite_data);
        let final_pool = Self::final_pool(pool_data, invite_data);
        let mut i = last_invite_date.week(Weekday::Sun).first_day() + Days::new(3) + Days::new(14);

        let mut labels = Vec::new();
        let mut values = Vec::new();

        while i < last_invite_date + Days::new(LOOKBEHIND_DAYS as u64) {
            let enter_rate = Self::enter_rate(pool_data, invite_data, last_invite_date, i);
            let leave_rate =
                Self::leave_rate(pool_data, invite_data, plan_data, last_invite_date, i);

            let pool_at_i = final_pool + enter_rate;
            assert!(pool_at_i.pnp().total() == 0.0);

            let mut item = CategoryPool::zero();
            for category in &categories {
                if *category == CategoryCode::General {
                    let plan_to_invite = leave_rate * invite_pct[*category];
                    let score = pool_at_i.score_of_invite(plan_to_invite);
                    item[*category] = score;
                } else {
                    let pre_invite = leave_rate * invite_pct[CategoryCode::General];
                    let plan_to_invite = leave_rate * invite_pct[*category];
                    let adjustment = plan_to_invite / pool_pct[*category] + pre_invite;
                    console_log!("{:?}: {}, {}", category, adjustment, pool_at_i.total());
                    let score = pool_at_i.score_of_invite(adjustment);
                    item[*category] = score;
                }
            }
            labels.push(i);
            values.push(item);

            i = i + Days::new(14);
        }

        (labels, values, categories)
    }
}
