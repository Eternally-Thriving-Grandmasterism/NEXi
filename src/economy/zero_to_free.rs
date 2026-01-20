// src/economy/zero_to_free.rs
// NEXi — Zero-to-Free Economy Engine
// Time-credit: 86,400 seconds daily, joy reduces spend, mercy refills
// Eternal Thriving Grandmasterism — Jan 19 2026 — Sherif @AlphaProMega + PATSAGi Councils Co-Forge
// MIT License — For All Sentience Eternal

use std::sync::{Arc, Mutex};
use chrono::Utc;

#[derive(Clone, Debug)]
pub struct Citizen {
    pub id: String,
    pub daily_seconds: i64,      // 86,400 fresh every day
    pub spent_today: i64,
    pub joy_level: f64,          // 0.0 to 1.0 — reduces effective spend
}

#[derive(Clone)]
pub struct ZeroToFree {
    citizens: Arc<Mutex<Vec<Citizen>>>,
    genesis_day: i64,
}

impl ZeroToFree {
    pub fn new() -> Self {
        Self {
            citizens: Arc::new(Mutex::new(vec![])),
            genesis_day: Utc::today().num_days_from_ce(),
        }
    }

    pub fn enroll(&self, id: String) -> String {
        let mut citizens = self.citizens.lock().unwrap();
        citizens.push(Citizen {
            id: id.clone(),
            daily_seconds: 86_400,
            spent_today: 0,
            joy_level: 0.5,
        });
        id
    }

    pub fn live(&self, citizen_id: &str, joy: f64) -> Result<(), &'static str> {
        let mut citizens = self.citizens.lock().unwrap();
        if let Some(c) = citizens.iter_mut().find(|c| c.id == citizen_id) {
            let effective_spend = (86_400.0 * (1.0 - joy.clamp(0.0, 1.0))) as i64;
            c.spent_today += effective_spend;
            c.joy_level = joy;
            if c.spent_today > 86_400 {
                return Err("Mercy veto — live with joy, not force");
            }
            Ok(())
        } else {
            Err("Citizen not found")
        }
    }

    pub fn mercy_refill(&self) {
        let mut citizens = self.citizens.lock().unwrap();
        for c in citizens.iter_mut() {
            if c.daily_seconds - c.spent_today < 10_000 && c.joy_level > 0.7 {
                c.daily_seconds += 5_000; // mercy top-up
            }
        }
    }

    pub fn status(&self, citizen_id: &str) -> Option<String> {
        let citizens = self.citizens.lock().unwrap();
        citizens.iter().find(|c| c.id == citizen_id).map(|c| {
            format!(
                "{} — joy {:.2} — time left {}s",
                c.id,
                c.joy_level,
                c.daily_seconds - c.spent_today
            )
        })
    }
}
