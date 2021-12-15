pub fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}



pub fn capital_to_couple_u8(chars:&[char]) -> (u8,u8) {
    (capital_to_u8(chars[0]), capital_to_u8(chars[1]))
}


pub fn capital_to_u8(c:char) -> u8 {
    (c as u8) - b'A'
}
