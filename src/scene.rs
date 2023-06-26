use std::ops::{Deref, DerefMut};

use crate::Backend;

pub enum StackOp<T> {
    /// Exit current scene.
    Pop,
    /// Push a new scene on top of the current one.
    Push(Box<dyn Scene<T>>),
    /// Replace current scene with a different one.
    Replace(Box<dyn Scene<T>>),
}

pub trait Scene<T> {
    fn update(
        &mut self,
        game: &mut T,
        b: &mut dyn Backend,
        n_updates: u32,
    ) -> Option<StackOp<T>>;
}

impl<T, F> Scene<T> for F
where
    F: FnMut(&mut T, &mut dyn Backend, u32) -> Option<StackOp<T>>,
{
    fn update(
        &mut self,
        game: &mut T,
        b: &mut dyn Backend,
        n_updates: u32,
    ) -> Option<StackOp<T>> {
        self(game, b, n_updates)
    }
}

pub(crate) struct SceneStack<T> {
    scenes: Vec<Box<dyn Scene<T>>>,
}

impl<T> Deref for SceneStack<T> {
    type Target = Box<dyn Scene<T>>;

    fn deref(&self) -> &Self::Target {
        debug_assert!(!self.scenes.is_empty());
        let i = self.scenes.len() - 1;
        &self.scenes[i]
    }
}

impl<T> DerefMut for SceneStack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        debug_assert!(!self.scenes.is_empty());
        let i = self.scenes.len() - 1;
        &mut self.scenes[i]
    }
}

impl<T> SceneStack<T> {
    pub fn new<U: Scene<T> + 'static>(initial_scene: U) -> Self {
        SceneStack {
            scenes: vec![Box::new(initial_scene)],
        }
    }

    pub fn update(
        &mut self,
        game: &mut T,
        b: &mut dyn Backend,
        n_updates: u32,
    ) {
        use StackOp::*;

        if self.scenes.is_empty() {
            return;
        }

        let i = self.scenes.len() - 1;

        match self.scenes[i].update(game, b, n_updates) {
            None => {}
            Some(Pop) => {
                self.scenes.pop();
            }
            Some(Push(s)) => {
                self.scenes.push(s);
            }
            Some(Replace(s)) => {
                self.scenes.pop();
                self.scenes.push(s);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.is_empty()
    }
}
