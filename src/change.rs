/// Determine wether the `bitfield.set()` method changed the underlying value.
#[derive(Debug, PartialEq)]
pub enum Change {
  /// The value was changed. Equal to `true` in `mafintosh/sparse-bitfield`.
  Changed,
  /// The value was not changed. Equal to `false` in
  /// `mafintosh/sparse-bitfield`.
  Unchanged,
}

impl Change {
  /// Returns `true` if the result is `Changed`.
  #[inline]
  pub fn is_changed(&self) -> bool {
    *self == Change::Changed
  }

  /// Returns `true` if the result is `Unchanged`.
  #[inline]
  pub fn is_unchanged(&self) -> bool {
    !self.is_changed()
  }
}
