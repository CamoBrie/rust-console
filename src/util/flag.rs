use enum_iterator::{all, cardinality, Sequence};
use std::{marker::PhantomData, fmt::{Debug, Formatter, Result}};

pub trait Flag<T>: Sequence + Eq + Copy + Debug {
    fn handle(&self, data: &mut T, flags: &mut Flags<Self, T>);
    fn to_index(&self) -> usize {
        all::<Self>().position(|flag| &flag == self)
            .expect("The index of this variant should be within the list of all variants of this Sequence")
    }
}

pub struct Flags<T: Debug, X>{
    state: Vec<Option<T>>,
    phantom: PhantomData<X>,
}

impl<T: Flag<X> + Eq + Copy + Debug, X> Flags<T, X>{
  pub fn mark(&mut self, f: T){
      self.state[f.to_index()] = Some(f);
  }

  pub fn is_marked(&self, f: &T) -> bool{
      self.state[f.to_index()].is_some()
  }

  pub fn handle(&mut self, data: &mut X){
    let mut next: Flags<T, X> = Flags::new(); //TODO: only clear each flag when handled
    for f in self.state.iter().flatten(){
      f.handle(data, &mut next);
    }
    self.state = next.state;
  }

  pub fn new() -> Flags<T, X> {
    Flags{
      state: vec![None; cardinality::<T>()],
      phantom: PhantomData
    }
  }
}

impl<T: Flag<X>, X> Debug for Flags<T, X>{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&format!("Flags of type \"{}\": ",
                std::any::type_name::<T>().split("::").last().unwrap()
                ))?;
        f.write_str(all::<T>()
            .filter(|flag| self.is_marked(flag))
            .map(|flag| format!("{flag:?}"))
            .collect::<Vec<_>>()
            .join(", ")
            .as_str())
    }
}
