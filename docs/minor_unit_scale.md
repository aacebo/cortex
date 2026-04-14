# Minor Unit Scale

## Definition

`minor_unit_scale` tells you how many decimal places sit between a currency’s major unit and its stored minor units.

If:

```
minor_unit_scale = N
```

then:

```
1 major unit = 10^N minor units
```

---

## Examples

### USD
```
minor_unit_scale = 2
```

- 1 dollar = 100 cents  
- $12.34 = 1234 minor units  

---

### JPY
```
minor_unit_scale = 0
```

- 1 yen = 1 minor unit  
- ¥500 = 500 minor units  

---

### KWD
```
minor_unit_scale = 3
```

- 1 dinar = 1000 minor units  
- 1.250 KWD = 1250 minor units  

---

## Conversions

### Display → Stored

```
stored_minor_units = display_amount × 10^scale
```

Example:
- USD (scale 2)
- 19.99 → 1999

---

### Stored → Display

```
display_amount = minor_units / 10^scale
```

Example:
- USD (scale 2)
- 1999 → 19.99

---

## Code Example

```rust
pub struct Currency {
    pub code: String,
    pub minor_unit_scale: u32,
}
```

Example:

```rust
Currency {
    code: "USD".into(),
    minor_unit_scale: 2,
}
```

---

## Why It Matters

The raw integer (`i64`) has no meaning without scale.

Example:

```
1500
```

Could mean:
- $15.00 (scale 2)
- ¥1500 (scale 0)
- 1.500 (scale 3)

---

## Summary

`minor_unit_scale` = number of decimal places used by the currency’s smallest unit.

Typical values:
- USD → 2
- EUR → 2
- JPY → 0
