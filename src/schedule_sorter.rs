use crate::schedule::Schedule;
use anyhow::Result;

pub trait BestSchedule {
    fn best(self) -> Result<Schedule>;
}

impl BestSchedule for Schedule {
    fn best(self) -> Result<Schedule> {
        Ok(self)
    }
}

impl BestSchedule for Vec<Schedule> {
    fn best(self) -> Result<Schedule> {
        self.into_iter()
            .max_by_key(|sched| {
                sched
                    .courses
                    .iter()
                    .flatten()
                    .filter_map(|c| {
                        sched
                            .catalog
                            .courses
                            .get(c)
                            .and_then(|(_, creds, _)| *creds)
                    })
                    .sum::<u32>()
            })
            .ok_or_else(|| anyhow::anyhow!("No valid schedule found"))
    }
}
