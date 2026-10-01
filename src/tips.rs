pub struct Tips;

impl Tips {
    pub const WATER_TIPS: &'static [&'static str] = &[
        "Time for 250ml (a glass) of fresh water! Stay energized.",
        "Your brain is ~73% water. Hydration directly sharpens focus and memory!",
        "Mid-day fatigue is often just mild dehydration. Drink up!",
        "Drink a cool glass of water and take a deep breath.",
        "Water helps regulate body temperature and keeps joints lubricated.",
        "Hydration check! Refill your water bottle or grab a glass.",
        "Good work so far! Give your body the water it needs to power through.",
    ];

    pub const BREAK_TIPS: &'static [&'static str] = &[
        "20-20-20 Rule: Look at something 20 feet (6m) away for 20 seconds to relax your eyes.",
        "Stand up, stretch your arms, and gently roll your shoulders back.",
        "Step away from your monitor for a moment. Give your neck and spine a rest.",
        "Take 5 deep breaths: inhale 4s, hold 4s, exhale 4s.",
        "Walk around your room for 60 seconds to restore circulation.",
        "Rest your eyes and unclench your jaw. Posture check: sit up straight!",
        "A 2-minute movement break lowers stress and boosts productivity for the next hour.",
    ];

    pub fn get_water_tip(index: usize) -> &'static str {
        Self::WATER_TIPS[index % Self::WATER_TIPS.len()]
    }

    pub fn get_break_tip(index: usize) -> &'static str {
        Self::BREAK_TIPS[index % Self::BREAK_TIPS.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_tips_rotation() {
        assert_eq!(Tips::get_water_tip(0), Tips::WATER_TIPS[0]);
        let len = Tips::WATER_TIPS.len();
        assert_eq!(Tips::get_water_tip(len), Tips::WATER_TIPS[0]);
        assert_eq!(Tips::get_water_tip(len + 1), Tips::WATER_TIPS[1]);
    }

    #[test]
    fn test_break_tips_rotation() {
        assert_eq!(Tips::get_break_tip(0), Tips::BREAK_TIPS[0]);
        let len = Tips::BREAK_TIPS.len();
        assert_eq!(Tips::get_break_tip(len), Tips::BREAK_TIPS[0]);
        assert_eq!(Tips::get_break_tip(len + 2), Tips::BREAK_TIPS[2]);
    }
}
