use super::skill::Skill;
use crate::ui::frame::{Drawable, Frame};

#[derive(Clone)]
pub struct SkillsControl {
    pub skills: Vec<Skill>,
}

impl SkillsControl {
    pub fn new() -> Self {
        Self { skills: Vec::new() }
    }

    pub fn push(&mut self, skill: Skill) {
        skill.render();
        self.skills.push(skill);
    }

    pub fn refresh(&mut self) {
        self.skills.retain(|skill| skill.is_active());
    }
}

impl Drawable for SkillsControl {
    fn draw(&self, frame: &mut Frame) {
        let drawables: Vec<&Skill> = self
            .skills
            .iter()
            .filter_map(|skill| if skill.is_active() { Some(skill) } else { None })
            .collect();

        for drawable in drawables {
            drawable.draw(frame);
        }
    }
}
