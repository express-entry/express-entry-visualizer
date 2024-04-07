use super::calc::ScorePool;
use crate::data::{CategoryCode, Invite, Pool};

pub struct PnpRemover;

impl PnpRemover {
    pub fn remove_pnp(pool_data: &[Pool], invite_data: &[Invite]) -> Vec<Invite> {
        let mut non_pnp_invite = Vec::with_capacity(invite_data.len());

        let mut pool_data: Vec<_> = pool_data.iter().copied().rev().collect();
        let mut pool_to_invite = None;
        for invite in invite_data {
            while let Some(pool) = pool_data.last() {
                if pool.date > invite.date {
                    break;
                }
                pool_to_invite = Some(ScorePool::from(pool_data.pop().unwrap()));
            }

            if let Some(pool) = pool_to_invite {
                let invite_as_pool = pool.invite(invite);
                pool_to_invite = Some(pool - invite_as_pool);

                if invite.category.code != CategoryCode::Province {
                    let mut new_invite = *invite;
                    new_invite.size = invite_as_pool.non_pnp().total();
                    non_pnp_invite.push(new_invite);
                }
            } else {
                if invite.category.code != CategoryCode::Province {
                    non_pnp_invite.push(*invite);
                }
            }
        }
        non_pnp_invite
    }
}
