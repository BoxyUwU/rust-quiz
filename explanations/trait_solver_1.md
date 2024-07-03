# Trait Solver 1 @BoxyUwU @WaffleLapkin

The trait implementation is for a higher ranked function pointer (`for<'a> fn`).
But the where clause is different, there the `for<'a>` is parsed as part of the bound, so the bound is on a *not* higher-ranked function pointer.

impl:
```
type: for<'a> fn(&'a u32)
trait: Trait
```

bound:
```
type: fn(&'a u32)
trait: Trait
```

Only higher-ranked function pointers implement the trait, so it fails to compile.
