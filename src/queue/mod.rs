use std::{fmt::{Display, Debug}, collections::LinkedList, ops::{DerefMut, Deref}};

use crate::{list::List, error::Error};

pub struct Queue<const SIZE: usize, T, const WRITE_OVER: bool> (List<SIZE, T, WRITE_OVER>);

impl<const SIZE: usize, T, const WRITE_OVER: bool> Queue<SIZE, T, WRITE_OVER> {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn enqueue(&mut self, elem: T) -> Result<&mut Self, Error> {
        match self.0.push_back(elem) {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }

    pub fn peek(&mut self, elem: T) -> Option<&T> {
        if self.0.len() == 0 {
            return None;
        }
        Some(&self.0[0usize])
    }

    pub fn dequeue(&mut self, elem: T) -> Option<T> {
        self.0.remove_front()
    }
}

impl<const S: usize, T, const W: bool> Display for Queue<S, T, W> where T: Display{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const S: usize, T, const W: bool> Debug for Queue<S, T, W> where T: Debug{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Queue")
            .field("", &self.0)
            .finish()
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Vec<T>> for Queue<QUEUE_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Ok(
            Queue(
                {
                    match value.try_into() {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(err)
                        },
                    }
                }
            )
        )
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<LinkedList<T>> for Queue<QUEUE_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: LinkedList<T>) -> Result<Self, Self::Error> {
        Ok(
            Queue(
                {
                    match value.try_into() {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(err)
                        },
                    }
                }
            )
        )
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> FromIterator<T> for Queue<QUEUE_SIZE, T, WRITE_OVER> where T: Default {
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        Queue(iter.into_iter().collect())
    }
}

//generic generator
impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> TryFrom<Box<dyn Iterator<Item = T>>> for Queue<QUEUE_SIZE, T, WRITE_OVER> where T: Clone + Default {
    type Error = Error;

    fn try_from(value: Box<dyn Iterator<Item = T>>) -> Result<Self, Self::Error> {
        Ok(
            Queue(
                {
                    match value.try_into() {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(err)
                        },
                    }
                }
            )
        )
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> From<[T; QUEUE_SIZE]> for Queue<QUEUE_SIZE, T, WRITE_OVER> {
    fn from(value: [T; QUEUE_SIZE]) -> Self {
        Queue(value.into())
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> From<List<QUEUE_SIZE, T, WRITE_OVER>> for Queue<QUEUE_SIZE, T, WRITE_OVER>{
    fn from(value: List<QUEUE_SIZE, T, WRITE_OVER>) -> Self {
        Self(value)
    }
}

impl<const QUEUE_SIZE: usize, T> From<Queue<QUEUE_SIZE, T, true>> for Queue<QUEUE_SIZE, T, false> {
    fn from(value: Queue<QUEUE_SIZE, T, true>) -> Self {
        Self(value.0.into())
    }
}

impl<const QUEUE_SIZE: usize, T> From<Queue<QUEUE_SIZE, T, false>> for Queue<QUEUE_SIZE, T, true> {
    fn from(value: Queue<QUEUE_SIZE, T, false>) -> Self {
        Self(value.0.into())
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> Deref for Queue<QUEUE_SIZE, T, WRITE_OVER> {
    type Target = List<QUEUE_SIZE, T, WRITE_OVER>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const QUEUE_SIZE: usize, T, const WRITE_OVER: bool> DerefMut for Queue<QUEUE_SIZE, T, WRITE_OVER> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}